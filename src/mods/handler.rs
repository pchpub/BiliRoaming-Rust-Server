use super::request::{async_getwebpage, async_postwebpage, redis_get, redis_set};
use super::types::{
    random_string, Area, BackgroundTaskType, BiliConfig, EpAreaCacheType, PlayurlParams,
    SearchParams, UserResignInfo,
};
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use async_channel::Sender;
use chrono::prelude::Local;
use deadpool_redis::Pool;
use md5;
use pcre2::bytes::Regex;
use qstring::QString;
use serde_json::{self, json};
use std::sync::Arc;

use super::cache::{
    get_cached_ep_area, get_cached_playurl, get_cached_season, update_area_cache,
    update_area_cache_force, update_cached_playurl_background, update_cached_season,
};
use super::upstream_res::{
    get_upstream_bili_playurl, get_upstream_bili_search, get_upstream_bili_season,
};
use super::user_info::*;

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
    let bilisender_cl = Arc::clone(bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let mut params = PlayurlParams {
        is_app,
        is_th,
        ..Default::default()
    };
    // deal with req
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
                // query param must have area
                return build_response("{\"code\":-412,\"message\":\"请求被拦截\"}".to_string());
            }
        }
    };
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => {
            return build_response(
                "{\"code\":-10403,\"message\":\"草,没ua你看个der\"}".to_string(),
            );
        }
    };

    if is_app && config.limit_biliroaming_version_open {
        match req.headers().get("build") {
            Some(value) => {
                let version: u16 = value.to_str().unwrap_or("0").parse().unwrap_or(0);
                if version < config.limit_biliroaming_version_min
                    || version > config.limit_biliroaming_version_max
                {
                    return build_response(
                        "{\"code\":-412,\"message\":\"什么旧版本魔人,升下级\"}".to_string(),
                    );
                }
            }
            None => (),
        }
    }

    params.app_key = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436",
    };

    match params.appkey_to_sec() {
        Ok(_) => (),
        Err(_) => {
            return build_response("{\"code\":-10403,\"message\":\"未知设备\"}".to_string());
        }
    };

    if is_app || is_th {
        if query_string.len() <= 39
            || (format!(
                "{:x}",
                md5::compute(format!(
                    "{}{}",
                    &query_string[..query_string.len() - 38],
                    params.app_sec
                ))
            ) != &query_string[query_string.len() - 32..])
        {
            return build_response("{\"code\":-3,\"message\":\"API校验密匙错误\"}".to_string());
        }
    }

    params.access_key = match query.get("access_key") {
        Option::Some(key) => {
            let key = key;
            if key.len() == 0 {
                return build_response(
                    "{\"code\":-101,\"message\":\"没有accesskey,你b站和漫游需要换个版本\"}"
                        .to_string(),
                );
            } else {
                key
            }
        }
        _ => {
            return build_response(
                "{\"code\":-101,\"message\":\"草,没登陆你看个der,让我凭空拿到你账号是吧\"}"
                    .to_string(),
            );
        }
    };

    params.ep_id = match query.get("ep_id") {
        Option::Some(key) => key,
        _ => "",
    };

    params.cid = match query.get("cid") {
        Option::Some(key) => key,
        _ => "",
    };

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

    let user_info = match get_user_info(
        params.access_key,
        params.app_key,
        params.app_sec,
        params.user_agent,
        false,
        &config,
        redis_pool,
    )
    .await
    {
        Ok(value) => value,
        Err((err_code, err_msg)) => {
            return build_response(format!("{{\"code\":{err_code},\"message\":\"{err_msg}\"}}"));
        }
    };

    params.is_vip = user_info.user_is_vip();

    // TODO: add check ep vip status here, forbid non-vip user get vip

    let user_cer_info = match get_blacklist_info(&user_info.uid, &config, redis_pool).await {
        Ok(value) => value,
        Err((err_code, err_msg)) => {
            return build_response(format!("{{\"code\":{err_code},\"message\":\"{err_msg}\"}}"));
        }
    };
    let white: bool;
    match user_cer_info {
        super::types::UserCerStatus::Black(value) => {
            return build_response(format!(r#"{{"code":-10403,"message":"{}"}}"#, value));
        }
        super::types::UserCerStatus::White => {
            white = true;
        }
        super::types::UserCerStatus::Normal => {
            white = false;
        }
    }

    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let new_access_key;
    if is_th {
        params.is_vip = false;
        if *config.resign_open.get("4").unwrap_or(&false)
            && (white || *config.resign_pub.get("4").unwrap_or(&false))
        {
            (new_access_key, _) = get_resign_accesskey(redis_pool, &4, &params.user_agent, &config)
                .await
                .unwrap_or((params.access_key.to_string(), 1));
            params.is_vip = true;
            params.access_key = new_access_key.as_str();
        }
    } else {
        if user_info.vip_expire_time >= ts {
            params.is_vip = true;
        } else if *config.resign_open.get("4").unwrap_or(&false)
            && (white
                || *config
                    .resign_pub
                    .get(&params.area_num.to_string())
                    .unwrap_or(&false))
        {
            (new_access_key, _) =
                get_resign_accesskey(redis_pool, &params.area_num, &params.user_agent, &config)
                    .await
                    .unwrap_or((params.access_key.to_string(), 1));
            params.access_key = new_access_key.as_str();
            let user_info = match get_user_info(
                params.access_key,
                params.app_key,
                params.app_sec,
                params.user_agent,
                false,
                &config,
                redis_pool,
            )
            .await
            {
                Ok(value) => value,
                Err((err_code, err_msg)) => {
                    return build_response(format!("{{\"code\":{err_code},\"message\":\"{err_msg}\"}}"));
                }
            };
            params.is_vip = user_info.user_is_vip();
        }
    }

    if config.area_cache_open {
        if params.ep_id == "" {
            let return_data = match get_upstream_bili_playurl(
                &mut params,
                &config,
                bilisender,
                user_info,
            )
            .await
            {
                Ok(value) => value,
                Err(value) => value,
            };
            return build_response(return_data);
        };
        if let Ok(value) = get_cached_ep_area(&params, redis_pool).await {
            let http_body_return = match value {
                // if without current area cache data then such ep is never accessed, of course doesnt have cache
                EpAreaCacheType::NoCurrentAreaData(key, redis_value) => {
                    match get_upstream_bili_playurl(&mut params, &config, bilisender, user_info)
                        .await
                    {
                        Ok(http_body) => {
                            update_area_cache(&http_body, &params, &key, &redis_value, redis_pool)
                                .await;
                            http_body
                        }
                        Err(http_body) => http_body,
                    }
                }
                EpAreaCacheType::OnlyHasCurrentAreaData(is_exist) => {
                    if is_exist {
                        let return_data =
                            match get_cached_playurl(&params, &bilisender_cl, redis_pool).await {
                                Ok(data) => data,
                                Err(_) => {
                                    match get_upstream_bili_playurl(
                                        &mut params,
                                        &config,
                                        bilisender,
                                        user_info,
                                    )
                                    .await
                                    {
                                        Ok(value) => value,
                                        Err(value) => value,
                                    }
                                }
                            };
                        return_data
                    } else {
                        // should not have such condition,
                        // if so, maybe proxy settings do not correspond one-to-one with the expected region,
                        // causing update_area_cache_force got error area limit info
                        // if really encounter with such condition, try to traditionally update area cache
                        update_cached_playurl_background(&params, &bilisender_cl).await;
                        // update_area_cache_force(bilisender_cl, params.ep_id).await;
                        "{\"code\":-10403,\"message\":\"该剧集被判定为没有地区能播放\"}".to_string()
                    }
                }
                EpAreaCacheType::Available(area) => {
                    match area {
                        Area::Th => {
                            params.is_th = true;
                        }
                        _ => {
                            params.is_th = false;
                        }
                    }
                    params.init_params();
                    let return_data =
                        match get_cached_playurl(&params, &bilisender_cl, redis_pool).await {
                            Ok(data) => data,
                            Err(_) => match get_upstream_bili_playurl(
                                &mut params,
                                &config,
                                bilisender,
                                user_info,
                            )
                            .await
                            {
                                Ok(value) => value,
                                Err(value) => value,
                            },
                        };
                    return_data
                }
                EpAreaCacheType::NoEpData => {
                    // if havent any cache info, try to manally update area cache for later use
                    update_area_cache_force(bilisender_cl, params.ep_id).await;
                    match get_upstream_bili_playurl(&mut params, &config, bilisender, user_info)
                        .await
                    {
                        Ok(http_body) => http_body,
                        Err(http_body) => http_body,
                    }
                }
            };
            return build_response(http_body_return);
        } else {
            let return_data = match get_upstream_bili_playurl(
                &mut params,
                &config,
                bilisender,
                user_info,
            )
            .await
            {
                Ok(value) => value,
                Err(value) => value,
            };
            return build_response(return_data);
        }
    } else {
        let return_data =
            match get_upstream_bili_playurl(&mut params, &config, bilisender, user_info).await {
                Ok(value) => value,
                Err(value) => value,
            };
        return build_response(return_data);
    }
}

pub async fn handle_search_request(req: &HttpRequest, is_app: bool, is_th: bool) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let _bilisender_cl = Arc::clone(bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let mut params = SearchParams {
        is_app,
        is_th,
        ..Default::default()
    };
    // deal with req
    params.user_agent = match req.headers().get("user-agent") {
        Option::Some(_ua) => req.headers().get("user-agent").unwrap().to_str().unwrap(),
        _ => {
            return build_response(
                "{\"code\":-10403,\"message\":\"草,没ua你搜个der\"}".to_string(),
            );
        }
    };

    params.access_key = match query.get("access_key") {
        Option::Some(key) => key,
        _ => {
            if params.is_app && (!params.is_th) {
                return HttpResponse::Ok().content_type(ContentType::json()).body(
                    "{\"code\":-101,\"message\":\"草,没登陆你搜个der,让我凭空拿到你账号是吧\"}",
                );
            } else {
                ""
            }
        }
    };

    params.app_key = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436", //为了应对新的appkey,应该设定默认值
    };

    params.keyword = match query.get("keyword") {
        Option::Some(key) => key,
        _ => "",
    };

    (params.area, params.area_num) = match query.get("area") {
        Some(area) => match area {
            "cn" => ("cn", 1),
            "hk" => ("hk", 2),
            "tw" => ("tw", 3),
            "th" => {
                params.app_key = "7d089525d3611b1c";
                ("th", 4)
            }
            _ => ("hk", 2),
        },
        _ => {
            if is_th {
                ("th", 4)
            } else {
                // query param must have area
                return build_response("{\"code\":-412,\"message\":\"请求被拦截\"}".to_string());
            }
        }
    };

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

    //println!("[Debug] cookie:{}", cookie);

    match params.appkey_to_sec() {
        Ok(_) => (),
        Err(_) => {
            return build_response("{\"code\":-10403,\"message\":\"未知设备\"}".to_string());
        }
    };

    if is_app && (!is_th) {
        let user_info = match get_user_info(
            params.access_key,
            params.app_key,
            params.app_sec,
            params.user_agent,
            false,
            &config,
            redis_pool,
        )
        .await
        {
            Ok(value) => value,
            Err((err_code, err_msg)) => {
                return build_response(format!("{{\"code\":{err_code},\"message\":\"{err_msg}\"}}"));
            }
        };

        match get_blacklist_info(&user_info.uid, &config, redis_pool).await {
            //为了记录accesskey to uid
            _ => (),
        };
    }

    params.build = query
        .get("build")
        .unwrap_or(if is_th { "1080003" } else { "6400000" });

    params.device = query.get("device").unwrap_or("android");

    params.statistics = match query.get("statistics") {
        Some(value) => value,
        _ => "",
    };

    params.pn = query.get("pn").unwrap_or("1");

    params.fnval = query.get("fnval").unwrap_or("976");

    let body_data = match get_upstream_bili_search(&params, &query, config, bilisender).await {
        Ok(value) => {
            if params.pn != "1" {
                return build_response(value);
            };
            // if !is_app {
            //     return build_response(value);
            // }
            value
        }
        Err(value) => return build_response(value),
    };

    let host = match req.headers().get("Host") {
        Some(host) => host.to_str().unwrap(),
        _ => match req.headers().get("authority") {
            Some(host) => host.to_str().unwrap(),
            _ => "",
        },
    };

    let mut body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
    let upstream_code = body_data_json["code"]
        .as_str()
        .unwrap_or("233")
        .parse::<i64>()
        .unwrap_or(233);
    if upstream_code != 0 {
        // if config.report_open {
        //     let num = redis_get(&redis_pool, &format!("02{}1301", area_num))
        //         .await
        //         .unwrap_or("0".to_string())
        //         .as_str()
        //         .parse::<u32>()
        //         .unwrap();
        //     if num == 4 {
        //         redis_set(&redis_pool, &format!("02{}1301", area_num), "1", 0)
        //             .await
        //             .unwrap_or_default();
        //         let senddata = SendData::Health(SendHealthData {
        //             area_num,
        //             data_type: SesourceType::PlayUrl,
        //             health_type: HealthType::Offline,
        //         });
        //         tokio::spawn(async move {
        //             //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
        //             match bilisender_cl.try_send(senddata) {
        //                 Ok(_) => (),
        //                 Err(TrySendError::Full(_)) => {
        //                     println!("[Error] channel is full");
        //                 }
        //                 Err(TrySendError::Closed(_)) => {
        //                     println!("[Error] channel is closed");
        //                 }
        //             };
        //         });
        //     } else {
        //         redis_set(
        //             &redis_pool,
        //             &format!("02{}1301", area_num),
        //             &(num + 1).to_string(),
        //             0,
        //         )
        //         .await
        //         .unwrap_or_default();
        //     }
        // }
        return build_response("{\"code\":-10403,\"message\":\"获取失败喵\"}".to_string());
    }

    let search_remake_date = {
        if is_app {
            if let Some(value) = config.appsearch_remake.get(host) {
                value
            } else {
                return build_response(body_data);
            }
        } else {
            if let Some(value) = config.websearch_remake.get(host) {
                value
            } else {
                return build_response(body_data);
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
    // TODO: MOVE TO HEALTH
    // if config.report_open {
    //     match redis_get(&redis_pool, &format!("02{}1301", area_num)).await {
    //         Some(value) => {
    //             let err_num = value.parse::<u16>().unwrap();
    //             if err_num >= 4 {
    //                 redis_set(&redis_pool, &format!("02{}1301", area_num), "0", 0)
    //                     .await
    //                     .unwrap_or_default();
    //                 let senddata = SendData::Health(SendHealthData {
    //                     area_num,
    //                     data_type: SesourceType::Search,
    //                     health_type: HealthType::Online,
    //                 });
    //                 tokio::spawn(async move {
    //                     //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
    //                     match bilisender_cl.try_send(senddata) {
    //                         Ok(_) => (),
    //                         Err(TrySendError::Full(_)) => {
    //                             println!("[Error] channel is full");
    //                         }
    //                         Err(TrySendError::Closed(_)) => {
    //                             println!("[Error] channel is closed");
    //                         }
    //                     };
    //                 });
    //             } else if err_num != 0 {
    //                 redis_set(&redis_pool, &format!("02{}1301", area_num), "0", 0)
    //                     .await
    //                     .unwrap_or_default();
    //             }
    //         }
    //         None => {
    //             redis_set(&redis_pool, &format!("02{}1301", area_num), "0", 0)
    //                 .await
    //                 .unwrap_or_default();
    //             let senddata = SendData::Health(SendHealthData {
    //                 area_num,
    //                 data_type: SesourceType::Search,
    //                 health_type: HealthType::Online,
    //             });
    //             tokio::spawn(async move {
    //                 //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
    //                 match bilisender_cl.try_send(senddata) {
    //                     Ok(_) => (),
    //                     Err(TrySendError::Full(_)) => {
    //                         println!("[Error] channel is full");
    //                     }
    //                     Err(TrySendError::Closed(_)) => {
    //                         println!("[Error] channel is closed");
    //                     }
    //                 };
    //             });
    //         }
    //     }
    // }
    let body_data = body_data_json.to_string();
    return build_response(body_data);
}

pub async fn get_season(req: &HttpRequest, _is_app: bool, _is_th: bool) -> HttpResponse {
    let (pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    let _bilisender_cl = Arc::clone(bilisender);
    match req.headers().get("user-agent") {
        Option::Some(_ua) => (),
        _ => {
            return build_response(
                "{\"code\":-10403,\"message\":\"草,没ua你看个der\"}".to_string(),
            );
        }
    }

    let user_agent = format!(
        "{}",
        req.headers().get("user-agent").unwrap().to_str().unwrap()
    );
    let query = QString::from(req.query_string());

    let access_key = match query.get("access_key") {
        Option::Some(key) => key,
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":2403,\"message\":\"草,没登陆你搜个der,让我凭空拿到你账号是吧\"}");
        }
    };

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

    let season_id = query.get("season_id").unwrap_or("114514");

    let build = query.get("build").unwrap_or("1080003");

    match get_cached_season(pool, season_id).await {
        Ok(value) => return build_response(value),
        Err(_) => {
            match get_upstream_bili_season(access_key, build, season_id, &user_agent, config).await
            {
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
                                &false,
                                "",
                                &user_agent,
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
                                let ep: usize =
                                    sub_replace_json["data"][index_of_replace_json]["ep"]
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
                                let len_of_episodes = body_data_json["result"]["modules"][0]
                                    ["data"]["episodes"]
                                    .as_array()
                                    .unwrap()
                                    .len();
                                let mut index = 0;
                                while index < len_of_episodes {
                                    body_data_json["result"]["modules"][0]["data"]["episodes"]
                                        [index]
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
                    // TODO: MOVE TO HEALTH
                    // if config.report_open {
                    //     match redis_get(&pool, "0441301").await {
                    //         Some(value) => {
                    //             let err_num = value.parse::<u16>().unwrap();
                    //             if err_num >= 4 {
                    //                 redis_set(&pool, "0441301", "0", 0)
                    //                     .await
                    //                     .unwrap_or_default();
                    //                 let senddata = SendData::Health(SendHealthData {
                    //                     area_num: 4,
                    //                     data_type: SesourceType::PlayUrl,
                    //                     health_type: HealthType::Online,
                    //                 });
                    //                 tokio::spawn(async move {
                    //                     //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                    //                     match bilisender_cl.try_send(senddata) {
                    //                         Ok(_) => (),
                    //                         Err(TrySendError::Full(_)) => {
                    //                             println!("[Error] channel is full");
                    //                         }
                    //                         Err(TrySendError::Closed(_)) => {
                    //                             println!("[Error] channel is closed");
                    //                         }
                    //                     };
                    //                 });
                    //             } else if err_num != 0 {
                    //                 redis_set(&pool, "0441301", "0", 0)
                    //                     .await
                    //                     .unwrap_or_default();
                    //             }
                    //         }
                    //         None => {
                    //             redis_set(&pool, "0441301", "0", 0)
                    //                 .await
                    //                 .unwrap_or_default();
                    //             let senddata = SendData::Health(SendHealthData {
                    //                 area_num: 4,
                    //                 data_type: SesourceType::PlayUrl,
                    //                 health_type: HealthType::Online,
                    //             });
                    //             tokio::spawn(async move {
                    //                 //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                    //                 match bilisender_cl.try_send(senddata) {
                    //                     Ok(_) => (),
                    //                     Err(TrySendError::Full(_)) => {
                    //                         println!("[Error] channel is full");
                    //                     }
                    //                     Err(TrySendError::Closed(_)) => {
                    //                         println!("[Error] channel is closed");
                    //                     }
                    //                 };
                    //             });
                    //         }
                    //     }
                    // }
                    update_cached_season(season_id, &body_data, pool, config).await;
                    return build_response(body_data);
                }
                Err(_) => {
                    return build_response(
                        "{\"code\":-404,\"message\":\"获取失败喵\"}".to_string(),
                    );
                }
            }
        }
    }
}

pub async fn get_resign_accesskey(
    redis: &Pool,
    area_num: &u8,
    user_agent: &str,
    config: &BiliConfig,
) -> Option<(String, u64)> {
    if *config
        .resign_api_policy
        .get(&area_num.to_string())
        .unwrap_or(&false)
    {
        let key = format!("a{area_num}1201");
        let dt = Local::now();
        let ts = dt.timestamp() as u64;
        match redis_get(redis, &key).await {
            Some(value) => {
                let resign_info_json: UserResignInfo = serde_json::from_str(&value).unwrap();
                if resign_info_json.expire_time > ts {
                    return Some((resign_info_json.access_key, resign_info_json.expire_time));
                }
            }
            None => (),
        };
        let area_num_str = area_num.to_string();
        let url = format!(
            "{}?area_num={}&sign={}",
            &config.resign_api.get(&area_num_str).unwrap(),
            &area_num,
            &config.resign_api_sign.get(&area_num_str).unwrap()
        );
        let webgetpage_data = if let Ok(data) = async_getwebpage(&url, &false, "", "", "").await {
            data
        } else {
            println!("[Error] 从非官方接口处获取accesskey失败");
            return None;
        };
        let webgetpage_data_json: serde_json::Value =
            if let Ok(value) = serde_json::from_str(&webgetpage_data) {
                value
            } else {
                println!("[Error] json解析失败: {}", webgetpage_data);
                return None;
            };
        if webgetpage_data_json["code"].as_i64().unwrap() != 0 {
            println!("err3");
            return None;
        }
        let access_key = webgetpage_data_json["access_key"]
            .as_str()
            .unwrap()
            .to_string();
        let resign_info = UserResignInfo {
            area_num: *area_num as i32,
            access_key: access_key.clone(),
            refresh_token: "".to_string(),
            expire_time: webgetpage_data_json["expire_time"]
                .as_u64()
                .unwrap_or(ts + 3600),
        };

        redis_set(redis, &key, &resign_info.to_json(), 3600).await;
        return Some((access_key, resign_info.expire_time));
    } else {
        let area_num = match area_num {
            4 => 4,
            _ => 1,
        };
        let resign_info_str = match redis_get(redis, &format!("a{area_num}1101")).await {
            Some(value) => value,
            None => return None,
        };
        let resign_info_json: UserResignInfo = serde_json::from_str(&resign_info_str).unwrap();
        let dt = Local::now();
        let ts = dt.timestamp() as u64;
        if resign_info_json.expire_time > ts {
            return Some((resign_info_json.access_key, resign_info_json.expire_time));
        } else {
            match area_num {
                4 => get_accesskey_from_token_th(redis, user_agent, config).await,
                _ => get_accesskey_from_token_cn(redis, user_agent, config).await,
            }
        }
    }
}

async fn get_accesskey_from_token_th(
    redis: &Pool,
    user_agent: &str,
    config: &BiliConfig,
) -> Option<(String, u64)> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let resign_info = to_resign_info(&redis_get(redis, &format!("a41101")).await.unwrap()).await;
    let access_key = resign_info.access_key;
    let refresh_token = resign_info.refresh_token;
    let url = "https://passport.biliintl.com/x/intl/passport-login/oauth2/refresh_token";
    let content = format!("access_token={access_key}&refresh_token={refresh_token}");
    let proxy_open = &config.th_proxy_token_open;
    let proxy_url = &config.th_proxy_token_url;
    let getpost_string =
        match async_postwebpage(&url, &content, proxy_open, proxy_url, user_agent).await {
            Ok(value) => value,
            Err(_) => return None,
        };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    let resign_info = UserResignInfo {
        area_num: 4,
        access_key: getpost_json["data"]["token_info"]["access_token"]
            .as_str()
            .unwrap()
            .to_string(),
        refresh_token: getpost_json["data"]["token_info"]["refresh_token"]
            .as_str()
            .unwrap()
            .to_string(),
        expire_time: getpost_json["data"]["token_info"]["expires_in"]
            .as_u64()
            .unwrap()
            + ts
            - 3600,
    };
    redis_set(redis, "a41101", &resign_info.to_json(), 0).await;
    Some((resign_info.access_key, resign_info.expire_time))
}

async fn get_accesskey_from_token_cn(
    redis: &Pool,
    user_agent: &str,
    config: &BiliConfig,
) -> Option<(String, u64)> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let resign_info = to_resign_info(&redis_get(redis, &format!("a11101")).await.unwrap()).await;
    let access_key = resign_info.access_key;
    let refresh_token = resign_info.refresh_token;
    let unsign_request_body = format!(
        "access_token={access_key}&appkey=1d8b6e7d45233436&refresh_token={refresh_token}&ts={ts}"
    );
    let url = "https://passport.bilibili.com/x/passport-login/oauth2/refresh_token";
    let content = format!(
        "{unsign_request_body}&sign={:x}",
        md5::compute(format!(
            "{unsign_request_body}560c52ccd288fed045859ed18bffd973"
        ))
    );
    let proxy_open = &config.cn_proxy_token_open;
    let proxy_url = &config.cn_proxy_token_url;
    let getpost_string =
        match async_postwebpage(&url, &content, proxy_open, proxy_url, user_agent).await {
            Ok(value) => value,
            Err(_) => return None,
        };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    let resign_info = UserResignInfo {
        area_num: 1,
        access_key: getpost_json["data"]["token_info"]["access_token"]
            .as_str()
            .unwrap()
            .to_string(),
        refresh_token: getpost_json["data"]["token_info"]["refresh_token"]
            .as_str()
            .unwrap()
            .to_string(),
        expire_time: getpost_json["data"]["token_info"]["expires_in"]
            .as_u64()
            .unwrap()
            + ts
            - 3600,
    };
    redis_set(redis, "a11101", &resign_info.to_json(), 0).await;
    Some((resign_info.access_key, resign_info.expire_time))
}

async fn to_resign_info(resin_info_str: &str) -> UserResignInfo {
    serde_json::from_str(resin_info_str).unwrap()
}

pub async fn get_subtitle_th(req: &HttpRequest, _: bool, _: bool) -> HttpResponse {
    let (pool, config, _bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    match req.headers().get("user-agent") {
        Option::Some(_ua) => (),
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":1403,\"message\":\"草,没ua你看个der\"}");
        }
    }
    let user_agent = format!(
        "{}",
        req.headers().get("user-agent").unwrap().to_str().unwrap()
    );
    let mut query = QString::from(req.query_string());
    let ep_id = query.get("ep_id").unwrap();
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let key = format!("e{ep_id}41201");
    let is_expire: bool;
    let mut redis_get_data = String::new();
    match redis_get(&pool, &key).await {
        Some(value) => {
            if &value[..13].parse::<u64>().unwrap() < &(ts * 1000) {
                is_expire = true;
            } else {
                redis_get_data = value[13..].to_string();
                is_expire = false;
            }
        }
        None => {
            is_expire = true;
        }
    };
    if is_expire {
        query.add_str(&format!(
            "&appkey=7d089525d3611b1c&mobi_app=bstar_a&s_locale=zh_SG&ts={ts}"
        ));
        let mut query_vec = query.to_pairs();
        query_vec.sort_by_key(|v| v.0);
        // 硬编码app_sec
        let app_sec = "acd495b248ec528c2eed1e862d393126";
        let proxy_open = &config.th_proxy_subtitle_open;
        let proxy_url = &config.th_proxy_subtitle_url;
        let unsigned_url = qstring::QString::new(query_vec);
        let unsigned_url = format!("{unsigned_url}");
        let signed_url = format!(
            "{unsigned_url}&sign={:x}",
            md5::compute(format!("{unsigned_url}{app_sec}"))
        );
        let api = "https://app.biliintl.com/intl/gateway/v2/app/subtitle";
        let body_data = match async_getwebpage(
            &format!("{api}?{signed_url}"),
            proxy_open,
            proxy_url,
            &user_agent,
            "",
        )
        .await
        {
            Ok(data) => data,
            Err(_) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body("{\"code\":2404,\"message\":\"获取字幕失败喵\"}");
            }
        };
        let expire_time = config.cache.get("thsub").unwrap_or(&14400);
        let value = format!("{}{body_data}", (ts + expire_time) * 1000);
        redis_set(pool, &key, &value, *expire_time).await;
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body(body_data);
    } else {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body(redis_get_data);
    }
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
