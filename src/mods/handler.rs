use super::cache::{
    get_cached_ep_area, get_cached_playurl, get_cached_th_season, get_cached_th_subtitle,
    update_th_season_cache, update_th_subtitle_cache,
};
use super::request::async_getwebpage;
use super::types::{
    random_string, BackgroundTaskType, BiliConfig, BiliRuntime, EType, PlayurlParams, SearchParams,
};
use super::upstream_res::{
    get_upstream_bili_playurl, get_upstream_bili_search, get_upstream_bili_season,
    get_upstream_bili_subtitle, get_upstream_resigned_access_key,
};
use super::user_info::*;
use crate::return_http;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use async_channel::Sender;
use deadpool_redis::Pool;
use md5;
use pcre2::bytes::Regex;
use qstring::QString;
use serde_json::{self, json};
use std::sync::Arc;

// playurl分流
pub async fn handle_playurl_request(
    req: &HttpRequest,
    is_app: bool,
    is_th: bool,
    // cache: &mut BiliCache,
) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let bili_runtime = BiliRuntime::new(config, redis_pool, bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let mut params = PlayurlParams {
        is_app,
        is_th,
        ..Default::default()
    };
    // detect req area
    (params.area, params.area_num) = match query.get("area") {
        Some(area) => match area {
            "cn" => ("cn", 1),
            "hk" => ("hk", 2),
            "tw" => ("tw", 3),
            "th" => ("th", 4),
            _ => ("hk", 2),
        },
        _ => {
            if is_th {
                ("th", 4)
            } else {
                // query param must have "area", or must be invalid req
                return bili_error(EType::InvalidReq);
            }
        }
    };
    // detect req UA
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => return bili_error(EType::ReqUAError),
    };
    // detect req client ver
    if is_app && config.limit_biliroaming_version_open {
        match req.headers().get("build") {
            Some(value) => {
                let version: u16 = value.to_str().unwrap_or("0").parse().unwrap_or(0);
                if version < config.limit_biliroaming_version_min
                    || version > config.limit_biliroaming_version_max
                {
                    return bili_error(EType::OtherError(-412, "什么旧版本魔人,升下级"));
                }
            }
            None => (),
        }
    }
    // detect user's appkey
    params.appkey = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436",
    };
    if let Err(_) = params.appkey_to_sec() {
        return bili_error(EType::OtherError(-412, "未知设备"));
    };
    // verify req sign
    // TODO: add ignore sign err
    if is_app || is_th {
        if query_string.len() <= 39
            || (format!(
                "{:x}",
                md5::compute(format!(
                    "{}{}",
                    &query_string[..query_string.len() - 38],
                    params.appsec
                ))
            ) != &query_string[query_string.len() - 32..])
        {
            return bili_error(EType::ReqSignError);
        }
    }
    // detect user's access_key
    params.access_key = match query.get("access_key") {
        Option::Some(key) => {
            let key = key;
            if key.len() == 0 {
                return bili_error(EType::UserNotLoginedError);
            } else {
                key
            }
        }
        _ => {
            return bili_error(EType::UserNotLoginedError);
        }
    };
    // detect req ep
    params.ep_id = match query.get("ep_id") {
        Option::Some(key) => key,
        _ => "",
    };
    params.cid = match query.get("cid") {
        Option::Some(key) => key,
        _ => "",
    };
    // detect other info
    params.build = query.get("build").unwrap_or("6800300");

    params.device = query.get("device").unwrap_or("android");

    params.is_tv = match query.get("fnval") {
        Some(value) => match value {
            "130" => true,
            "0" => true,
            "2" => true,
            _ => false,
        },
        None => false,
    };

    // get user_info
    let user_info = match get_user_info(
        params.access_key,
        params.appkey,
        params.appsec,
        params.user_agent,
        false,
        &bili_runtime,
    )
    .await
    {
        Ok(value) => value,
        Err(value) => {
            return bili_error(value);
        }
    };
    // get user's vip status
    params.is_vip = user_info.is_vip();
    // get user's blacklist info
    let white = match get_blacklist_info(&user_info, &bili_runtime).await {
        Ok(value) => value,
        Err(value) => return bili_error(value),
    };
    // resign if needed
    let resigned_access_key;
    match resign_user_info(white, &params, &bili_runtime).await {
        Ok(value) => {
            if let Some(value) = value {
                (params.is_vip, resigned_access_key) = (value.0, value.1);
                params.access_key = &resigned_access_key;
            }
        }
        Err(value) => return bili_error(value),
    }
    // get area cache
    if config.area_cache_open && !(params.ep_id == "") {
        if let Some(area) = get_cached_ep_area(&params, &bili_runtime).await {
            params.area_num = area.num();
            params.init_params(area);
        }
    }
    let resp = match get_cached_playurl(&params, &bili_runtime).await {
        Ok(data) => Ok(data),
        Err(_) => get_upstream_bili_playurl(&mut params, &user_info, &bili_runtime).await,
    };
    return_http!(resp);
}

pub async fn handle_search_request(req: &HttpRequest, is_app: bool, is_th: bool) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let bili_runtime = BiliRuntime::new(config, redis_pool, bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let mut params = SearchParams {
        is_app,
        is_th,
        ..Default::default()
    };
    // detect req area
    (params.area, params.area_num) = match query.get("area") {
        Some(area) => match area {
            "cn" => ("cn", 1),
            "hk" => ("hk", 2),
            "tw" => ("tw", 3),
            "th" => ("th", 4),
            _ => ("hk", 2),
        },
        _ => {
            if is_th {
                ("th", 4)
            } else {
                // query param must have "area", or must be invalid req
                return bili_error(EType::InvalidReq);
            }
        }
    };
    // detect req UA
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => return bili_error(EType::ReqUAError),
    };
    // detect req client ver
    if is_app && config.limit_biliroaming_version_open {
        match req.headers().get("build") {
            Some(value) => {
                let version: u16 = value.to_str().unwrap_or("0").parse().unwrap_or(0);
                if version < config.limit_biliroaming_version_min
                    || version > config.limit_biliroaming_version_max
                {
                    return bili_error(EType::OtherError(-412, "什么旧版本魔人,升下级"));
                }
            }
            None => (),
        }
    }
    // detect user's appkey
    params.appkey = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436",
    };
    if let Err(_) = params.appkey_to_sec() {
        return bili_error(EType::OtherError(-412, "未知设备"));
    };
    // verify req sign
    // TODO: add ignore sign err
    if is_app || is_th {
        if query_string.len() <= 39
            || (format!(
                "{:x}",
                md5::compute(format!(
                    "{}{}",
                    &query_string[..query_string.len() - 38],
                    params.appsec
                ))
            ) != &query_string[query_string.len() - 32..])
        {
            return bili_error(EType::ReqSignError);
        }
    }
    // detect user's access_key
    params.access_key = match query.get("access_key") {
        Option::Some(key) => {
            let key = key;
            if key.len() == 0 {
                return bili_error(EType::UserNotLoginedError);
            } else {
                key
            }
        }
        _ => {
            return bili_error(EType::UserNotLoginedError);
        }
    };
    // detect other info
    params.build = query.get("build").unwrap_or("6800300");

    params.device = query.get("device").unwrap_or("android");

    params.statistics = match query.get("statistics") {
        Some(value) => value,
        _ => "",
    };

    params.pn = query.get("pn").unwrap_or("1");

    params.fnval = query.get("fnval").unwrap_or("976");

    params.keyword = query.get("keyword").unwrap_or("null");

    let cookie_buvid3_default = format!("buvid3={}", random_string());
    let cookie_buvid3_default = cookie_buvid3_default.as_str();
    params.cookie = if !is_app && !is_th {
        match req.headers().get("cookie") {
            Some(value) => {
                if let Ok(cookie_raw) = value.to_str() {
                    cookie_raw
                } else {
                    cookie_buvid3_default
                }
            }
            None => cookie_buvid3_default,
        }
    } else {
        ""
    };

    //为了记录accesskey to uid
    if is_app && (!is_th) {
        let user_info = match get_user_info(
            params.access_key,
            params.appkey,
            params.appsec,
            params.user_agent,
            false,
            &bili_runtime,
        )
        .await
        {
            Ok(value) => value,
            Err(value) => {
                return bili_error(value);
            }
        };
        get_blacklist_info(&user_info, &bili_runtime)
            .await
            .unwrap_or(false);
    }

    let host = match req.headers().get("Host") {
        Some(host) => host.to_str().unwrap(),
        _ => match req.headers().get("authority") {
            Some(host) => host.to_str().unwrap(),
            _ => "",
        },
    };

    let mut body_data_json: serde_json::Value =
        match get_upstream_bili_search(&params, &query, &bili_runtime).await {
            Ok(value) => {
                if params.pn != "1" {
                    return build_response(value.to_string());
                };
                // if !is_app {
                //     return build_response(value);
                // }
                value
            }
            Err(value) => return bili_error(value),
        };

    let search_remake_date = {
        if is_app {
            if let Some(value) = config.appsearch_remake.get(host) {
                value
            } else {
                return build_response(body_data_json.to_string());
            }
        } else {
            if let Some(value) = config.websearch_remake.get(host) {
                value
            } else {
                return build_response(body_data_json.to_string());
            }
        }
    };

    if is_app {
        match body_data_json["data"]["items"].as_array_mut() {
            Some(value2) => {
                value2.insert(0, serde_json::from_str(&search_remake_date).unwrap());
            }
            None => {
                body_data_json["data"]["items"] = json!([]);
                // Bad design! 好像找不到其他办法 寄
                body_data_json["data"]["items"]
                    .as_array_mut()
                    .unwrap()
                    .insert(0, serde_json::from_str(&search_remake_date).unwrap());
            }
        }
    } else {
        match body_data_json["data"]["result"].as_array_mut() {
            Some(value2) => {
                value2.insert(0, serde_json::from_str(&search_remake_date).unwrap());
            }
            None => {
                body_data_json["data"]["result"] = json!([]);
                body_data_json["data"]["result"]
                    .as_array_mut()
                    .unwrap()
                    .insert(0, serde_json::from_str(&search_remake_date).unwrap());
            }
        }
    }

    let body_data = body_data_json.to_string();
    return build_response(body_data);
}

pub async fn handle_th_season_request(
    req: &HttpRequest,
    _is_app: bool,
    _is_th: bool,
) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let bili_runtime = BiliRuntime::new(config, redis_pool, bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let mut params = PlayurlParams {
        area: "th",
        area_num: 4,
        ..Default::default()
    };
    // detect req UA
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => return bili_error(EType::ReqUAError),
    };
    // detect user's access_key
    params.access_key = match query.get("access_key") {
        Option::Some(key) => {
            let key = key;
            if key.len() == 0 {
                return bili_error(EType::UserNotLoginedError);
            } else {
                key
            }
        }
        _ => {
            return bili_error(EType::UserNotLoginedError);
        }
    };
    // init th appkey & appsec
    params.appkey_to_sec().unwrap();

    // let user_info = match getuser_list(
    //     pool,
    //     access_key,
    //     "1d8b6e7d45233436",
    //     &appkey_to_sec("1d8b6e7d45233436").unwrap(),
    //     &user_agent,
    // )
    // .await
    // {
    //     Ok(value) => value,
    //     Err(value) => {
    //         return HttpResponse::Ok()
    //             .content_type(ContentType::json())
    //             .body(format!("{{\"code\":-2337,\"message\":\"{value}\"}}"));
    //     }
    // };

    // let (_, _) = match auth_user(pool, &user_info.uid, &access_key, &config).await {
    //     Ok(value) => value,
    //     Err(_) => (false, false),
    // }; //为了让不带access key 的web搜索脚本能用(不带用户信息，这是极坏的)

    params.season_id = if let Some(value) = query.get("season_id") {
        value
    } else {
        return bili_error(EType::OtherError(-10403, "参数错误"));
    };

    params.build = query.get("build").unwrap_or("1080003");

    match get_cached_th_season(params.season_id, &bili_runtime).await {
        Ok(value) => return build_response(value),
        Err(_) => match get_upstream_bili_season(&params, &bili_runtime).await {
            Ok(value) => {
                let body_data = value;
                let season_remake = move || async move {
                    if config.th_app_season_sub_open {
                        let mut body_data_json: serde_json::Value =
                            serde_json::from_str(&body_data).unwrap();
                        let season_id: Option<u64>;
                        let is_result: bool;
                        match &body_data_json["result"] {
                            serde_json::Value::Object(value) => {
                                is_result = true;
                                season_id = Some(value["season_id"].as_u64().unwrap());
                            }
                            serde_json::Value::Null => {
                                is_result = false;
                                match &body_data_json["data"] {
                                    serde_json::Value::Null => {
                                        season_id = None;
                                    }
                                    serde_json::Value::Object(value) => {
                                        season_id = Some(value["season_id"].as_u64().unwrap());
                                    }
                                    _ => {
                                        season_id = None;
                                    }
                                }
                            }
                            _ => {
                                is_result = false;
                                season_id = None;
                            }
                        }

                        match season_id {
                            None => {
                                return body_data;
                            }
                            Some(_) => (),
                        }

                        let sub_replace_str = match async_getwebpage(
                            &format!("{}{}", &config.th_app_season_sub_api, season_id.unwrap()),
                            false,
                            "",
                            params.user_agent,
                            "",
                        )
                        .await
                        {
                            Ok(value) => value,
                            Err(_) => {
                                return body_data;
                            }
                        };
                        let sub_replace_json: serde_json::Value =
                            if let Ok(value) = serde_json::from_str(&sub_replace_str) {
                                value
                            } else {
                                return body_data;
                            };
                        match sub_replace_json["code"].as_i64().unwrap_or(233) {
                            0 => {
                                if body_data_json["result"]["modules"]
                                    .as_array_mut()
                                    .unwrap()
                                    .len()
                                    == 0
                                {
                                    return body_data;
                                }
                            }
                            _ => {
                                return body_data;
                            }
                        }
                        let mut index_of_replace_json = 0;
                        let len_of_replace_json =
                            sub_replace_json["data"].as_array().unwrap().len();
                        while index_of_replace_json < len_of_replace_json {
                            let ep: usize = sub_replace_json["data"][index_of_replace_json]["ep"]
                                .as_u64()
                                .unwrap() as usize;
                            let key = sub_replace_json["data"][index_of_replace_json]["key"]
                                .as_str()
                                .unwrap();
                            let lang = sub_replace_json["data"][index_of_replace_json]["lang"]
                                .as_str()
                                .unwrap();
                            let url = sub_replace_json["data"][index_of_replace_json]["url"]
                                .as_str()
                                .unwrap();
                            if is_result {
                                let element = format!("{{\"id\":{index_of_replace_json},\"key\":\"{key}\",\"title\":\"[非官方] {lang} {}\",\"url\":\"https://{url}\"}}",config.th_app_season_sub_name);
                                body_data_json["result"]["modules"][0]["data"]["episodes"][ep]
                                    ["subtitles"]
                                    .as_array_mut()
                                    .unwrap()
                                    .insert(0, serde_json::from_str(&element).unwrap());
                            }
                            index_of_replace_json += 1;
                        }

                        if config.aid_replace_open {
                            let len_of_episodes = body_data_json["result"]["modules"][0]["data"]
                                ["episodes"]
                                .as_array()
                                .unwrap()
                                .len();
                            let mut index = 0;
                            while index < len_of_episodes {
                                body_data_json["result"]["modules"][0]["data"]["episodes"][index]
                                    .as_object_mut()
                                    .unwrap()
                                    .insert("aid".to_string(), serde_json::json!(&config.aid));
                                index += 1;
                            }
                        }

                        let body_data = body_data_json.to_string();
                        return body_data;
                    } else {
                        return body_data;
                    }
                };
                let body_data = season_remake().await;
                update_th_season_cache(params.season_id, &body_data, &bili_runtime).await;
                return build_response(body_data);
            }
            Err(error_type) => return bili_error(error_type),
        },
    }
}

pub async fn handle_th_subtitle_request(req: &HttpRequest, _: bool, _: bool) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let bili_runtime = BiliRuntime::new(config, redis_pool, bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let mut params = PlayurlParams {
        area: "th",
        area_num: 4,
        ..Default::default()
    };
    params.appkey_to_sec().unwrap();
    // detect req UA
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => return bili_error(EType::ReqUAError),
    };
    // detect req ep
    params.ep_id = match query.get("ep_id") {
        Option::Some(key) => key,
        _ => "",
    };

    match get_cached_th_subtitle(&params, &bili_runtime).await {
        Ok(value) => build_response(value),
        Err(is_expired) => {
            if is_expired {
                match get_upstream_bili_subtitle(&params, query_string, &bili_runtime).await {
                    Ok(value) => {
                        update_th_subtitle_cache(&value, &params, &bili_runtime).await;
                        build_response(value)
                    }
                    Err(error_type) => bili_error(error_type),
                }
            } else {
                bili_error(EType::ServerGeneral)
            }
        }
    }
}

pub async fn handle_api_access_key_request(req: &HttpRequest) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let bili_runtime = BiliRuntime::new(config, redis_pool, bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let area_num: u8 = match query.get("area_num") {
        Some(key) => key.parse().unwrap(),
        _ => {
            // query param must have "area", or must be invalid req
            return bili_error(EType::OtherError(-10403, "参数错误: area_num为空"));
        }
    };

    match query.get("sign") {
        Option::Some(key) => {
            if key != &config.api_sign {
                return bili_error(EType::OtherError(-412, "签名错误"));
            }
        }
        _ => {
            return bili_error(EType::OtherError(-412, "无签名参数"));
        }
    };

    let user_agent = "User-Agent:Mozilla/5.0 (Linux; Android 4.1.2; Nexus 7 Build/JZ054K) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Safari/535.19";

    let (access_key, expire_time) = if let Some(value) =
        get_upstream_resigned_access_key(&area_num, user_agent, &bili_runtime).await
    {
        value
    } else {
        return bili_error(EType::OtherError(-404, "获取AK失败"));
    };

    build_response(format!(
        r#"{{"code":0,"message":"","access_key":"{access_key}","expire_time":{expire_time}}}"#
    ))
}

pub async fn errorurl_reg(url: &str) -> Option<u8> {
    let re = if let Ok(value) = Regex::new(
        r"(/pgc/player/api/playurl)|(/pgc/player/web/playurl)|(/intl/gateway/v2/ogv/playurl)|(/x/v2/search/type)|(/x/web-interface/search/type)|(/intl/gateway/v2/app/search/type)|(/intl/gateway/v2/ogv/view/app/season)|(/intl/gateway/v2/app/subtitle)",
    ) {
        value
    } else {
        return None;
    };
    let caps = if let Ok(value) = re.captures(url.as_bytes()) {
        match value {
            Some(cap) => cap,
            None => return None,
        }
    } else {
        return None;
    };
    //println!("{:?}",caps);
    let mut res_url: &str = "";
    let mut index = 1;
    while index <= 8 {
        match &caps.get(index) {
            Some(value) => {
                res_url = std::str::from_utf8(value.as_bytes()).unwrap();
                break;
            }
            None => (),
        }
        index += 1;
    }

    match res_url {
        "/pgc/player/api/playurl" => Some(1),
        "/pgc/player/web/playurl" => Some(2),
        "/intl/gateway/v2/ogv/playurl" => Some(3),
        "/x/v2/search/type" => Some(4),
        "/x/web-interface/search/type" => Some(5),
        "/intl/gateway/v2/app/search/type" => Some(6),
        "/intl/gateway/v2/ogv/view/app/season" => Some(7),
        "/intl/gateway/v2/app/subtitle" => Some(8),
        _ => None,
    }
}

fn build_response(message: String) -> HttpResponse {
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("From", "biliroaming-rust-server"))
        .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
        .insert_header(("Access-Control-Allow-Credentials", "true"))
        .insert_header(("Access-Control-Allow-Methods", "GET"))
        .body(message);
}
fn bili_error(error_type: EType) -> HttpResponse {
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("From", "biliroaming-rust-server"))
        .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
        .insert_header(("Access-Control-Allow-Credentials", "true"))
        .insert_header(("Access-Control-Allow-Methods", "GET"))
        .body(error_type.err_json());
}
