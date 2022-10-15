use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use async_channel::Sender;
use deadpool_redis::Pool;
use qstring::QString;
use std::sync::Arc;
use super::cache::{get_cached_ep_area, update_area_cache, get_cached_playurl};
use super::types::{Area, BiliConfig, BiliPlayurlParams, GetEpAreaType, SendData};
use super::upstream_res::get_upstream_bili_playurl;
use super::user_info::*;

// playurl分流
pub async fn handle_playurl_request(
    req: &HttpRequest,
    is_app: bool,
    is_th: bool,
    // cache: &mut BiliCache,
) -> HttpResponse {
    let (redis_pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<SendData>>)>()
        .unwrap();
    let bilisender_cl = Arc::clone(bilisender);
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let mut params = BiliPlayurlParams {
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
            return build_response("{\"code\":-10403,\"message\":\"校验失败\"}".to_string());
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

    let user_info = match get_user_info(
        params.access_key,
        params.app_key,
        params.app_sec,
        params.user_agent,
        &config,
        redis_pool
    )
    .await
    {
        Ok(value) => value,
        Err(value) => {
            return build_response(format!("{{\"code\":-10403,\"message\":\"{value}\"}}"));
        }
    };

    let white = match get_blacklist_info(&user_info.uid, &config, redis_pool).await {
        Ok(value) => {
            if value.0 {
                return build_response(
                    "{\"code\":-10403,\"message\":\"黑名单用户,建议换号重开\"}".to_string(),
                );
            } else {
                value.1
            }
        }
        Err(value) => {
            return build_response(value);
        }
    };

    // TODO: add resign related code

    // TODO: add cache
    if config.area_cache_open {
        if params.ep_id == "" {
            let return_data = match get_upstream_bili_playurl(&params, &config).await {
                Ok(value) => value,
                Err(value) => value,
            };
            return build_response(return_data);
        };
        if let Ok(value) = get_cached_ep_area(&params, redis_pool).await {
            let http_body_return = match value {
                GetEpAreaType::NoCurrentAreaData(key, redis_value) => {
                    match get_upstream_bili_playurl(&params, &config).await {
                        Ok(http_body) => {
                            update_area_cache(
                                &http_body,
                                &params,
                                &key,
                                &redis_value,
                                redis_pool,
                            )
                            .await;
                            http_body
                        }
                        Err(http_body) => http_body,
                    }
                }
                GetEpAreaType::OnlyHasCurrentAreaData(is_exist) => {
                    if is_exist {
                        let return_data = match get_upstream_bili_playurl(&params, &config).await {
                            Ok(value) => value,
                            Err(value) => value,
                        };
                        return_data
                    } else {
                        "{\"code\":-10403,\"message\":\"该剧集被判定为没有地区能播放\"}".to_string()
                    }
                }
                GetEpAreaType::Available(area) => {
                    match area {
                        Area::Th => {
                            params.is_th = true;
                        }
                        _ => {
                            params.is_th = false;
                        }
                    }
                    params.init_params();
                    let return_data = match get_cached_playurl(&params, bilisender_cl, redis_pool).await {
                        Ok(data) => data,
                        Err(_) => {
                            match get_upstream_bili_playurl(&params, &config).await {
                                Ok(value) => value,
                                Err(value) => value,
                            }
                        }
                    };
                    return_data
                }
                GetEpAreaType::NoEpData(key) => {
                    match get_upstream_bili_playurl(&params, &config).await {
                        Ok(http_body) => {
                            update_area_cache(
                                &http_body,
                                &params,
                                &key,
                                "2222",
                                redis_pool,
                            )
                            .await;
                            http_body
                        }
                        Err(http_body) => http_body,
                    }
                }
            };
            return build_response(http_body_return);
        } else {
            let return_data = match get_upstream_bili_playurl(&params, &config).await {
                Ok(value) => value,
                Err(value) => value,
            };
            return build_response(return_data);
        }
    } else {
        let return_data = match get_upstream_bili_playurl(&params, &config).await {
            Ok(value) => value,
            Err(value) => value,
        };
        return build_response(return_data);
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
