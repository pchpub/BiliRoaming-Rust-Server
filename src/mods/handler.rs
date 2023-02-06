use super::cache::{
    get_cached_ep_area, get_cached_playurl, get_cached_th_season, get_cached_th_subtitle,
};
use super::health::report_health;
use super::types::{
    random_string, Area, BackgroundTaskType, BiliConfig, BiliRuntime, ClientType, EType,
    HealthData, HealthReportType, PlayurlParams, SearchParams,
};
use super::upstream_res::{
    get_upstream_bili_playurl, get_upstream_bili_search, get_upstream_bili_season,
    get_upstream_bili_subtitle,
};
use super::user_info::*;
use crate::{build_response, build_result_response, calc_md5};
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use async_channel::Sender;
use crypto::digest::Digest;
use crypto::md5::Md5;
use deadpool_redis::Pool;
use log::{debug, error, warn};
use pcre2::bytes::Regex;
use qstring::QString;
use serde_json::{self, json};
use std::sync::Arc;

// playurl分流
pub async fn handle_playurl_request(req: &HttpRequest, is_app: bool, is_th: bool) -> HttpResponse {
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
    // detect client ip for log
    let client_ip: String = match req.headers().get("X-Real-IP") {
        Some(value) => value.to_str().unwrap().to_owned(),
        None => format!("{:?}", req.peer_addr()),
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
                build_response!(EType::InvalidReq);
            }
        }
    };

    // detect req UA
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(ua) => ua.to_str().unwrap(),
        _ => {
            warn!("[GET PLAYURL] IP {client_ip} -> Detect req without UA");
            build_response!(EType::ReqUAError)
        }
    };

    // detect req client ver
    if config.limit_biliroaming_version_open && is_app {
        match req.headers().get("build") {
            Some(value) => {
                let version: u16 = value.to_str().unwrap_or("0").parse().unwrap_or(0);
                if version < config.limit_biliroaming_version_min
                    || version > config.limit_biliroaming_version_max
                {
                    build_response!(-412, "什么旧版本魔人,升下级");
                }
            }
            None => (),
        }
    }

    // detect user's appkey
    params.appkey = query.get("appkey").unwrap_or_else(|| {
        if params.is_app {
            "1d8b6e7d45233436"
        } else {
            // 网页端是ios的key
            "27eb53fc9058f8c3"
        }
    });
    if let Err(_) = params.appkey_to_sec() {
        error!(
            "[GET PLAYURL] IP {client_ip} -> Detect unknown appkey: {}",
            params.appkey
        );
        report_health(
            HealthReportType::Others(HealthData {
                is_custom: true,
                custom_message: format!(
                    "[GET PLAYURL] IP {client_ip} -> Detect unknown appkey: {}",
                    params.appkey
                ),
                ..Default::default()
            }),
            &bili_runtime,
        )
        .await;
        build_response!("-412", "未知设备");
    };

    // verify req sign
    // TODO: add ignore sign err
    if is_app || is_th {
        if query_string.len() <= 39
            || ({
                let mut raw_unsign_query_string = String::with_capacity(600);
                raw_unsign_query_string.push_str(&query_string[..query_string.len() - 38]);
                raw_unsign_query_string.push_str(params.appsec);
                calc_md5!(&raw_unsign_query_string)
            } != &query_string[query_string.len() - 32..])
        {
            build_response!(EType::ReqSignError);
        }
    }

    // detect user's access_key
    params.access_key = match query.get("access_key") {
        Some(key) => {
            if key.len() == 0 {
                error!("[GET PLAYURL] IP {client_ip} -> Detect req without access_key");
                build_response!(EType::UserNotLoginedError);
            } else {
                key
            }
        }
        _ => {
            error!("[GET PLAYURL] IP {client_ip} -> Detect req without access_key");
            build_response!(EType::UserNotLoginedError);
        }
    };

    // detect req ep
    params.ep_id = if let Some(value) = query.get("ep_id") {
        value
    } else {
        build_response!(EType::InvalidReq)
    };
    params.cid = query.get("cid").unwrap_or("");
    params.bvid = query.get("bvid").unwrap_or("");

    // detect client_type
    let client_type =
        if let Some(value) = ClientType::init(params.appkey, params.is_app, params.is_th, req) {
            value
        } else {
            build_response!(EType::InvalidReq)
        };
    // detect other info
    params.build = query.get("build").unwrap_or("6800300");
    params.session = query.get("session").unwrap_or("");
    params.device = query
        .get("device")
        .unwrap_or(client_type.device().unwrap_or(""));
    params.platform = query
        .get("platform")
        .unwrap_or(client_type.platform().unwrap_or(""));
    params.mobi_app = query
        .get("platform")
        .unwrap_or(client_type.mobi_app().unwrap_or(""));

    params.is_tv = match query.get("fnval") {
        Some(value) => match value {
            "130" | "0" | "2" => true,
            _ => false,
        },
        None => false,
    };
    // detect client accesskey type
    let client_ak_type = if let Some(value) =
        ClientType::init_for_ak(params.appkey, params.is_app, params.is_th, req)
    {
        value
    } else {
        ClientType::Unknown
    };

    // get user_info
    let user_info = match get_user_info(
        params.access_key,
        params.appkey,
        params.is_app,
        &client_ak_type,
        &bili_runtime,
    )
    .await
    {
        Ok(value) => value,
        Err(value) => {
            build_response!(value);
        }
    };

    // get user's vip status
    params.is_vip = if params.is_th {
        false
    } else {
        user_info.is_vip()
    };

    // get user's blacklist info
    let white = match get_blacklist_info(&user_info, &bili_runtime).await {
        Ok(value) => value,
        Err(value) => build_response!(value),
    };

    // resign if needed
    let resigned_access_key;
    match resign_user_info(white, &mut params, &bili_runtime).await {
        Ok(value) => {
            if let Some(value) = value {
                (params.is_vip, resigned_access_key) = (value.0, value.1);
                debug!(
                    "[GET PLAYURL] IP {client_ip} | UID {} | AREA {} | EP {} -> Use Resigned UserInfo: AK {} isVIP {}",
                    user_info.uid,
                    params.area.to_ascii_uppercase(),
                    params.ep_id,
                    &resigned_access_key,
                    params.is_vip
                );
                params.access_key = &resigned_access_key;
            }
        }
        Err(value) => build_response!(value),
    }

    // get area cache
    if config.area_cache_open && params.ep_id != "" {
        match get_cached_ep_area(&params, &bili_runtime).await {
            Ok(value) => match value {
                Some(area) => {
                    debug!(
                        "[GET PLAYURL] IP {client_ip} | UID {} | AREA {} | EP {} -> Use Cached Area: AREA_NUM {}",
                        user_info.uid,
                        params.area.to_ascii_uppercase(),
                        params.ep_id,
                        area.num()
                    );
                    params.area_num = area.num();
                    params.init_params(area);
                }
                None => {
                    debug!(
                        "[GET PLAYURL] IP {client_ip} | UID {} | AREA {} | EP {} -> No Cached Area",
                        user_info.uid,
                        params.area.to_ascii_uppercase(),
                        params.ep_id,
                    );
                }
            },
            Err(value) => build_response!(value),
        }
    }

    debug!(
        "[GET PLAYURL] IP {client_ip} | UID {} | AREA {} | EP {} -> REQ TRACE",
        user_info.uid,
        params.area.to_ascii_uppercase(),
        params.ep_id
    );
    let resp = match get_cached_playurl(&params, &bili_runtime).await {
        // 允许-999时用户获取缓存, 但不是VIP
        Ok(data) => {
            debug!(
                "[GET PLAYURL] IP {client_ip} | UID {} | AREA {} | EP {} -> Serve from cache",
                user_info.uid,
                params.area.to_ascii_uppercase(),
                params.ep_id
            );
            Ok(data)
        }
        Err(_) => get_upstream_bili_playurl(&mut params, &user_info, &bili_runtime).await,
    };
    build_result_response!(resp);
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
    // detect client ip for log
    let client_ip: String = match req.headers().get("X-Real-IP") {
        Some(value) => value.to_str().unwrap().to_owned(),
        None => format!("{:?}", req.peer_addr()),
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
                build_response!(EType::InvalidReq);
            }
        }
    };

    // detect req UA
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => {
            warn!("[GET SEARCH] IP {client_ip} | Detect req without UA");
            build_response!(EType::ReqUAError)
        }
    };

    // detect req client ver
    if is_app && config.limit_biliroaming_version_open {
        match req.headers().get("build") {
            Some(value) => {
                let version: u16 = value.to_str().unwrap_or("0").parse().unwrap_or(0);
                if version < config.limit_biliroaming_version_min
                    || version > config.limit_biliroaming_version_max
                {
                    build_response!(-412, "什么旧版本魔人,升下级");
                }
            }
            None => (),
        }
    }

    // detect client_type
    let client_type =
        if let Some(value) = ClientType::init(params.appkey, params.is_app, params.is_th, req) {
            value
        } else {
            build_response!(EType::InvalidReq)
        };

    // detect user's appkey
    params.appkey = query.get("appkey").unwrap_or_else(|| {
        if params.is_app {
            "1d8b6e7d45233436"
        } else {
            "27eb53fc9058f8c3"
        }
    });
    if let Err(_) = params.appkey_to_sec() {
        error!(
            "[GET SEARCH] IP {client_ip} | Detect unknown appkey: {}",
            params.appkey
        );
        report_health(
            HealthReportType::Others(HealthData {
                is_custom: true,
                custom_message: format!(
                    "[GET PLAYURL] IP {client_ip} -> Detect unknown appkey: {}",
                    params.appkey
                ),
                ..Default::default()
            }),
            &bili_runtime,
        )
        .await;
        build_response!(-412, "未知设备");
    };

    // rewrite appkey
    // 哔哩哔哩国际版客户端发的请求中的appkey是国内版的（不换会导致-663）
    params.appkey = client_type.appkey();

    // verify req sign
    // TODO: add ignore sign err
    if is_app || is_th {
        let mut raw_unsign_query_string = String::with_capacity(600);
        raw_unsign_query_string.push_str(&query_string[..query_string.len() - 38]);
        raw_unsign_query_string.push_str(params.appsec);
        let mut new_md5 = Md5::new();
        new_md5.input_str(&raw_unsign_query_string);
        if query_string.len() <= 39
            || (new_md5.result_str() != &query_string[query_string.len() - 32..])
        {
            build_response!(EType::ReqSignError);
        }
    }

    // detect user's access_key
    params.access_key = match query.get("access_key") {
        Option::Some(key) => {
            let key = key;
            if !params.is_app {
                ""
            } else if key.len() == 0 {
                build_response!(EType::UserNotLoginedError);
            } else {
                key
            }
        }
        _ => {
            build_response!(EType::UserNotLoginedError);
        }
    };

    params.device = query
        .get("device")
        .unwrap_or(client_type.device().unwrap_or("android"));
    params.mobi_app = client_type.mobi_app().unwrap_or("android");
    params.platform = client_type.platform().unwrap_or("android");

    params.is_tv = match query.get("fnval") {
        Some(value) => match value {
            "130" | "0" | "2" => true,
            _ => false,
        },
        None => false,
    };
    params.build = query.get("build").unwrap_or("6800300");
    params.device =
        query
            .get("device")
            .unwrap_or_else(|| if params.is_app { "android" } else { "iphone" });
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
    //deteect client accesskey type
    let client_type = {
        if !params.is_app {
            ClientType::Unknown
        } else {
            if let Some(value) =
                ClientType::init_for_ak(params.appkey, params.is_app, params.is_th, req)
            {
                value
            } else {
                ClientType::Unknown
            }
        }
    };

    //为了记录accesskey to uid
    let uid = if is_app && (!is_th) {
        match get_user_info(
            params.access_key,
            params.appkey,
            params.is_app,
            &client_type,
            &bili_runtime,
        )
        .await
        {
            Ok(value) => {
                get_blacklist_info(&value, &bili_runtime)
                    .await
                    .unwrap_or(false);
                value.uid
            }
            Err(_) => 0, // allow blacklist user to search
        }
    } else {
        0
    };

    let host = match req.headers().get("Host") {
        Some(host) => host.to_str().unwrap(),
        _ => match req.headers().get("authority") {
            Some(host) => host.to_str().unwrap(),
            _ => "",
        },
    };

    debug!(
        "[GET SEARCH] IP {client_ip} | UID {} | AREA {} | KEYWORD {} -> REQ TRACE",
        uid,
        params.area.to_ascii_uppercase(),
        params.keyword
    );
    let mut body_data_json: serde_json::Value =
        match get_upstream_bili_search(&params, &query, &bili_runtime).await {
            Ok(value) => {
                if params.pn != "1" {
                    build_response!(value);
                };
                // if !is_app {
                //     return build_response(value);
                // }
                value
            }
            Err(value) => build_response!(value),
        };

    let search_remake_date = {
        if is_app {
            if let Some(value) = config.appsearch_remake.get(host) {
                value
            } else {
                build_response!(body_data_json);
            }
        } else {
            if let Some(value) = config.websearch_remake.get(host) {
                value
            } else {
                build_response!(body_data_json);
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

    build_response!(body_data_json);
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
    // detect client ip for log
    let client_ip: String = match req.headers().get("X-Real-IP") {
        Some(value) => value.to_str().unwrap().to_owned(),
        None => format!("{:?}", req.peer_addr()),
    };

    // detect req UA
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => {
            warn!("[GET TH_SEASON] IP {client_ip} | Detect req without UA");
            build_response!(EType::ReqUAError)
        }
    };

    // detect user's access_key
    params.access_key = match query.get("access_key") {
        Option::Some(key) => {
            let key = key;
            if key.len() == 0 {
                build_response!(EType::UserNotLoginedError);
            } else {
                key
            }
        }
        _ => {
            build_response!(EType::UserNotLoginedError);
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
        // zone th req must have season_id, ep_id is not supported
        build_response!(-10403, "参数错误");
    };

    params.build = query.get("build").unwrap_or("1080003");

    debug!(
        "[GET TH_SEASON] IP {client_ip} | AREA TH | SID {} -> REQ TRACE",
        params.season_id
    );
    let resp = match get_cached_th_season(params.season_id, &bili_runtime).await {
        Ok(value) => {
            debug!(
                "[GET TH_SEASON] IP {client_ip} | AREA TH | SID {} -> Serve from cache",
                params.season_id
            );
            Ok(value)
        }
        Err(_) => get_upstream_bili_season(&params, &bili_runtime).await,
    };
    build_result_response!(resp);
}

pub async fn handle_th_subtitle_request(req: &HttpRequest, _: bool, _: bool) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let bili_runtime = BiliRuntime::new(config, redis_pool, bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let mut params = PlayurlParams {
        ..Default::default()
    };
    params.init_params(Area::Th);
    // detect client ip for log
    let client_ip: String = match req.headers().get("X-Real-IP") {
        Some(value) => value.to_str().unwrap().to_owned(),
        None => format!("{:?}", req.peer_addr()),
    };

    // detect req UA
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => {
            warn!("[GET TH_SUBTITLE] IP {client_ip} | Detect req without UA");
            build_response!(EType::ReqUAError)
        }
    };
    // detect req ep
    params.ep_id = match query.get("ep_id") {
        Option::Some(key) => key,
        _ => "",
    };

    debug!(
        "[GET TH_SUBTITLE] IP {client_ip} | AREA TH | EP {} -> Req trace",
        params.ep_id
    );
    let resp = match get_cached_th_subtitle(&params, &bili_runtime).await {
        Ok(value) => {
            debug!(
                "[GET TH_SUBTITLE] IP {client_ip} | AREA TH | EP {} -> Serve from cache",
                params.ep_id
            );
            Ok(value)
        }
        Err(is_expired) => {
            if is_expired {
                get_upstream_bili_subtitle(&params, query_string, &bili_runtime).await
            } else {
                Err(EType::ServerGeneral)
            }
        }
    };
    build_result_response!(resp)
}

pub async fn handle_api_access_key_request(req: &HttpRequest) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let bili_runtime = BiliRuntime::new(config, redis_pool, bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    // detect client ip for log
    // let client_ip: String = match req.headers().get("X-Real-IP") {
    //     Some(value) => value.to_str().unwrap().to_owned(),
    //     None => format!("{:?}", req.peer_addr()),
    // };

    let area_num: u8 = match query.get("area_num") {
        Some(key) => key.parse().unwrap(),
        _ => {
            // query param must have "area", or must be invalid req
            build_response!(-10403, "参数错误: area_num为空");
        }
    };

    match query.get("sign") {
        Option::Some(key) => {
            if key != &config.api_sign {
                build_response!(-412, "签名错误");
            }
        }
        _ => {
            build_response!(-412, "无签名参数");
        }
    };

    let user_agent = "User-Agent:Mozilla/5.0 (Linux; Android 4.1.2; Nexus 7 Build/JZ054K) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Safari/535.19";

    let (access_key, expire_time) =
        if let Some(value) = get_resigned_access_key(&area_num, user_agent, &bili_runtime).await {
            value
        } else {
            build_response!(-404, "获取AK失败");
        };

    build_response!(format!(
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
    debug!("[ERRORURL_REG] {:?}", caps);
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

// fn build_response(message: String) -> HttpResponse {
//     return HttpResponse::Ok()
//         .content_type(ContentType::json())
//         .insert_header(("From", "biliroaming-rust-server"))
//         .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
//         .insert_header(("Access-Control-Allow-Credentials", "true"))
//         .insert_header(("Access-Control-Allow-Methods", "GET"))
//         .body(message);
// }
