use super::cache::update_cached_user_info_force;
use super::health::report_health;
use super::request::async_getwebpage;
use super::tools::{check_playurl_need_vip, remove_parameters_playurl};
use super::types::{
    Area, BackgroundTaskType, BiliConfig, EpInfo, HealthData, HealthReportType,
    OnlineBlackListConfig, PlayurlParams, PlayurlParamsStatic, SearchParams, UpstreamReply,
    UserCerinfo, UserInfo, SERVER_GENERAL_ERROR_MESSAGE,
};
use async_channel::Sender;
use chrono::prelude::*;
use md5;
use qstring::QString;
use std::string::String;
use std::sync::Arc;

pub async fn get_upstream_bili_account_info(
    access_key: &str,
    app_key: &str,
    app_sec: &str,
    user_agent: &str,
    config: &BiliConfig,
) -> Result<UserInfo, (i64, String)> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_min = dt.timestamp() as u64;
    let sign = md5::compute(format!(
        "access_key={}&app_key={}&ts={}{}",
        access_key, app_key, ts_min, app_sec
    ));
    let url: String = format!(
        "https://app.bilibili.com/x/v2/account/myinfo?access_key={}&app_key={}&ts={}&sign={:x}",
        access_key, app_key, ts_min, sign
    );
    //println!("{}",url);
    let output = match async_getwebpage(
        &url,
        &config.cn_proxy_accesskey_open,
        &config.cn_proxy_accesskey_url,
        user_agent,
        "",
    )
    .await
    {
        Ok(data) => data,
        Err(value) => {
            // println!("getuser_list函数寄了 url:{}",url);
            return Err((-500, value.to_string()));
        }
    };

    //println!("{}",output);
    let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();
    let output_struct: UserInfo;
    let code = if let Some(value) = output_json["code"].as_i64() {
        value
    } else {
        // error!("[USER INFO] Parsing Upstream reply failed, Upstream Reply -> {}", output);
        return Err((-500, SERVER_GENERAL_ERROR_MESSAGE.to_string()));
    };
    output_struct = match code {
        0 => {
            UserInfo {
                access_key: String::from(access_key),
                uid: output_json["data"]["mid"].as_u64().unwrap(),
                vip_expire_time: output_json["data"]["vip"]["due_date"].as_u64().unwrap(),
                expire_time: {
                    if ts < output_json["data"]["vip"]["due_date"].as_u64().unwrap()
                        && output_json["data"]["vip"]["due_date"].as_u64().unwrap()
                            < ts + 25 * 24 * 60 * 60 * 1000
                    {
                        output_json["data"]["vip"]["due_date"].as_u64().unwrap()
                    } else {
                        ts + 25 * 24 * 60 * 60 * 1000
                    }
                }, //用户状态25天强制更新
            }
        }
        -400 => {
            // trace!("[USER INFO] AK {} | Get UserInfo failed -400. REQ Params -> APP_KEY {} | TS {} | APP_SEC {} | SIGN {:?}. Upstream Reply -> {}",
            //     access_key, app_key, ts_min, app_sec, sign, output_json
            // );
            return Err((code, "可能你用的不是手机".to_string()));
        }
        -101 => {
            // trace!(
            //     "[USER INFO] AK {} | Get UserInfo failed -101. Upstream Reply -> {}",
            //     access_key, output_json
            // );
            return Err((
                code,
                "账号未登录喵(b站api说的,估计你access_key过期了)".to_string(),
            ));
        }
        -3 => {
            // warn!("[USER INFO] AK {} | Get UserInfo failed -3. REQ Params -> APP_KEY {} | TS {} | APP_SEC {} | SIGN {:?}. Upstream Reply -> {}",
            //     access_key, app_key, ts_min, app_sec, sign, output_json
            // );
            return Err((code, "可能我sign参数算错了,非常抱歉喵".to_string()));
        }
        -412 => {
            // error!(
            //     "[USER INFO] AK {} | Get UserInfo failed -412. Upstream Reply -> {}",
            //     access_key, output_json
            // );
            return Err((code, "被草到风控了.....".to_string()));
        }
        _ => {
            // trace!("[USER INFO] AK {} | Get UserInfo failed. REQ Params -> APP_KEY {} | TS {} | APP_SEC {} | SIGN {:?}. Upstream Reply -> {}",
            //     access_key, app_key, ts_min, app_sec, sign, output_json
            // );
            return Err((
                code,
                format!("鼠鼠说:{}", output_json["code"].as_i64().unwrap()),
            ));
        }
    };
    return Ok(output_struct);
}

pub async fn get_upstream_blacklist_info(
    config: &OnlineBlackListConfig,
    uid: &u64,
) -> Option<UserCerinfo> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    //let user_cerinfo_str = String::new();
    let user_agent = format!("biliroaming-rust-server/{}", env!("CARGO_PKG_VERSION"));
    let getwebpage_data = match async_getwebpage(
        &format!("{}{uid}", config.api),
        &false,
        "",
        &user_agent,
        "",
    )
    .await
    {
        Ok(data) => data,
        Err(_) => return None,
    };
    let getwebpage_json: serde_json::Value = match serde_json::from_str(&getwebpage_data) {
        Ok(value) => value,
        Err(_) => {
            let return_data = UserCerinfo {
                uid: uid.clone(),
                black: true,
                white: false,
                ban_until: 0,
                status_expire_time: 0,
            };
            // println!("[Error] 请接入在线黑名单");
            return Some(return_data);
        }
    };
    if getwebpage_json["code"].as_i64().unwrap_or(233) == 0 {
        let return_data = UserCerinfo {
            uid: getwebpage_json["data"]["uid"].as_u64().unwrap(),
            black: getwebpage_json["data"]["is_blacklist"]
                .as_bool()
                .unwrap_or(false),
            white: getwebpage_json["data"]["is_whitelist"]
                .as_bool()
                .unwrap_or(false),
            status_expire_time: {
                match getwebpage_json["data"]["ban_until"].as_u64() {
                    Some(ban_until) => {
                        if ban_until > ts && ban_until < ts + 1 * 24 * 60 * 60 {
                            ban_until
                        } else {
                            ts + 1 * 24 * 60 * 60
                        }
                    }
                    None => ts + 1 * 24 * 60 * 60,
                }
            },
            ban_until: getwebpage_json["data"]["ban_until"].as_u64().unwrap_or(0),
        };
        //println!("[Debug] uid:{}", return_data.uid);
        return Some(return_data);
    } else {
        return None;
    }
}

pub async fn get_upstream_bili_playurl(
    // query: QString,
    params: &mut PlayurlParams<'_>,
    config: &BiliConfig,
    bilisender: &Arc<Sender<BackgroundTaskType>>,
    user_info: UserInfo,
) -> Result<String, String> {
    let bilisender_cl = Arc::clone(bilisender);
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(&str, &str)>;
    let playurl_type = params.get_playurl_type();
    if params.is_tv {
        query_vec = vec![
            ("access_key", &params.access_key[..]),
            ("app_key", params.app_key),
            ("build", params.build),
            ("device", params.device),
            ("fnval", "130"),
            ("fnver", "0"),
            ("fourk", "1"),
            ("platform", "android"),
            //("qn", query.get("qn").unwrap_or("112")), //720P 64 1080P高码率 112
            ("qn", "112"), //测试了下,没会员会回落到下一档,所以没必要区分 DLNA投屏就最高一档好了,支持其他档没必要,还增加服务器负担
            ("ts", &ts_string),
        ];
    } else {
        query_vec = vec![
            ("access_key", &params.access_key[..]),
            ("app_key", params.app_key),
            ("build", params.build),
            ("device", params.device),
            ("fnval", "4048"),
            ("fnver", "0"),
            ("fourk", "1"),
            ("platform", "android"),
            ("qn", "125"),
            ("ts", &ts_string),
        ];
    }

    // match ep_id {
    //     Some(value) => query_vec.push(("ep_id", value)),
    //     None => (),
    // }
    // match cid {
    //     Some(value) => query_vec.push(("cid", value)),
    //     None => (),
    // }
    // match area_num {
    //     4 => {
    //         // app_key = "7d089525d3611b1c";
    //         // app_sec = app_key_to_sec(&app_key).unwrap();
    //         // query_vec.push(("s_locale", "zh_SG"));
    //     }
    //     _ => (),
    // }
    if params.is_th {
        query_vec.push(("s_locale", "zh_SG"));
    }

    query_vec.sort_by_key(|v| v.0);
    let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{unsigned_url}");
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{}", params.app_sec))
    );
    let proxy_open = match params.area_num {
        1 => &config.cn_proxy_playurl_open,
        2 => &config.hk_proxy_playurl_open,
        3 => &config.tw_proxy_playurl_open,
        4 => &config.th_proxy_playurl_open,
        _ => &config.tw_proxy_playurl_open,
    };
    let proxy_url = match params.area_num {
        1 => &config.cn_proxy_playurl_url,
        2 => &config.hk_proxy_playurl_url,
        3 => &config.tw_proxy_playurl_url,
        4 => &config.th_proxy_playurl_url,
        _ => &config.tw_proxy_playurl_url,
    };
    let api = match params.is_app {
        true => match params.area_num {
            1 => &config.cn_app_playurl_api,
            2 => &config.hk_app_playurl_api,
            3 => &config.tw_app_playurl_api,
            4 => &config.th_app_playurl_api,
            _ => &config.tw_app_playurl_api,
        },
        false => match params.area_num {
            1 => &config.cn_web_playurl_api,
            2 => &config.hk_web_playurl_api,
            3 => &config.tw_web_playurl_api,
            4 => &config.th_web_playurl_api,
            _ => &config.tw_web_playurl_api,
        },
    };
    let mut body_data = match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        proxy_url,
        params.user_agent,
        "",
    )
    .await
    {
        Ok(data) => data,
        Err(_) => return Err("{\"code\":-404,\"message\":\"获取播放地址失败喵\"}".to_string()),
    };
    let mut body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
    let code = body_data_json["code"].as_i64().unwrap().clone();
    remove_parameters_playurl(&playurl_type, &mut body_data_json).unwrap_or_default();

    // let backup_policy = match params.area_num {
    //     1 => &config.cn_proxy_playurl_backup_policy,
    //     2 => &config.hk_proxy_playurl_backup_policy,
    //     3 => &config.tw_proxy_playurl_backup_policy,
    //     4 => &config.th_proxy_playurl_backup_policy,
    //     _ => &false,
    // };
    // TODO: 优化错误码处理, 即利用health_check机制
    // if code == -10500 as i64 && *backup_policy {
    //     let api = match params.is_app {
    //         true => match params.area_num {
    //             1 => &config.cn_app_playurl_backup_api,
    //             2 => &config.hk_app_playurl_backup_api,
    //             3 => &config.tw_app_playurl_backup_api,
    //             4 => &config.th_app_playurl_backup_api,
    //             _ => &config.tw_app_playurl_backup_api,
    //         },
    //         false => match params.area_num {
    //             1 => &config.cn_web_playurl_backup_api,
    //             2 => &config.hk_web_playurl_backup_api,
    //             3 => &config.tw_web_playurl_backup_api,
    //             4 => &config.th_web_playurl_backup_api,
    //             _ => &config.tw_web_playurl_backup_api,
    //         },
    //     };
    //     let proxy_open = match params.area_num {
    //         1 => &config.cn_proxy_playurl_backup_open,
    //         2 => &config.hk_proxy_playurl_backup_open,
    //         3 => &config.tw_proxy_playurl_backup_open,
    //         4 => &config.th_proxy_playurl_backup_open,
    //         _ => &config.tw_proxy_playurl_backup_open,
    //     };
    //     let proxy_url = match params.area_num {
    //         1 => &config.cn_proxy_playurl_backup_url,
    //         2 => &config.hk_proxy_playurl_backup_url,
    //         3 => &config.tw_proxy_playurl_backup_url,
    //         4 => &config.th_proxy_playurl_backup_url,
    //         _ => &config.tw_proxy_playurl_backup_url,
    //     };
    //     body_data = match async_getwebpage(
    //         &format!("{api}?{signed_url}"),
    //         proxy_open,
    //         proxy_url,
    //         params.user_agent,
    //         "",
    //     )
    //     .await
    //     {
    //         Ok(data) => data,
    //         Err(_) => return Err("{\"code\":-404,\"message\":\"获取播放地址失败喵\"}".to_string()),
    //     };
    //     body_data_json = serde_json::from_str(&body_data).unwrap();
    //     // code = body_data_json["code"].as_i64().unwrap();
    // }
    // code = body_data_json["code"].as_i64().unwrap_or(-233);
    //TODO: update user's vip status if cached non-vip user successfully get vip's ep
    if !params.is_vip {
        if let Ok(value) = check_playurl_need_vip(playurl_type, &body_data_json) {
            if value {
                let bilisender_cl = Arc::clone(bilisender);
                update_cached_user_info_force(bilisender_cl, params.access_key.to_string()).await
            }
        }
    }
    let message = body_data_json["message"]
        .as_str()
        .unwrap_or("Error on parsing Json Response")
        .to_string();
    let health_report_data = HealthReportType::Playurl(HealthData::init(
        Area::new(params.area_num as u8),
        true,
        UpstreamReply {
            code,
            message,
            proxy_open: *proxy_open,
            // .clone used here may do harm to perf for such func is used frequently
            // as biliconfig lives much longer, why not use String::from to create a new String?
            proxy_url: String::from(proxy_url.as_str()),
        },
    ));
    report_health(health_report_data, bilisender_cl).await;
    Ok(body_data_json.to_string())
}

pub async fn get_upstream_bili_playurl_background(
    params: &PlayurlParamsStatic,
    config: &BiliConfig,
) -> Result<String, String> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(&str, &str)>;
    let playurl_type = params.get_playurl_type();
    if params.is_tv {
        query_vec = vec![
            ("access_key", &params.access_key[..]),
            ("app_key", &params.app_key),
            ("build", &params.build),
            ("device", &params.device),
            ("fnval", "130"),
            ("fnver", "0"),
            ("fourk", "1"),
            ("platform", "android"),
            //("qn", query.get("qn").unwrap_or("112")), //720P 64 1080P高码率 112
            ("qn", "112"), //测试了下,没会员会回落到下一档,所以没必要区分 DLNA投屏就最高一档好了,支持其他档没必要,还增加服务器负担
            ("ts", &ts_string),
        ];
    } else {
        query_vec = vec![
            ("access_key", &params.access_key[..]),
            ("app_key", &params.app_key),
            ("build", &params.build),
            ("device", &params.device),
            ("fnval", "4048"),
            ("fnver", "0"),
            ("fourk", "1"),
            ("platform", "android"),
            ("qn", "125"),
            ("ts", &ts_string),
        ];
    }

    // match ep_id {
    //     Some(value) => query_vec.push(("ep_id", value)),
    //     None => (),
    // }
    // match cid {
    //     Some(value) => query_vec.push(("cid", value)),
    //     None => (),
    // }
    // match area_num {
    //     4 => {
    //         // app_key = "7d089525d3611b1c";
    //         // app_sec = app_key_to_sec(&app_key).unwrap();
    //         // query_vec.push(("s_locale", "zh_SG"));
    //     }
    //     _ => (),
    // }
    if params.is_th {
        query_vec.push(("s_locale", "zh_SG"));
    }

    query_vec.sort_by_key(|v| v.0);
    let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{unsigned_url}");
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{}", params.app_sec))
    );
    let proxy_open = match params.area_num {
        1 => &config.cn_proxy_playurl_open,
        2 => &config.hk_proxy_playurl_open,
        3 => &config.tw_proxy_playurl_open,
        4 => &config.th_proxy_playurl_open,
        _ => &config.tw_proxy_playurl_open,
    };
    let proxy_url = match params.area_num {
        1 => &config.cn_proxy_playurl_url,
        2 => &config.hk_proxy_playurl_url,
        3 => &config.tw_proxy_playurl_url,
        4 => &config.th_proxy_playurl_url,
        _ => &config.tw_proxy_playurl_url,
    };
    let api = match params.is_app {
        true => match params.area_num {
            1 => &config.cn_app_playurl_api,
            2 => &config.hk_app_playurl_api,
            3 => &config.tw_app_playurl_api,
            4 => &config.th_app_playurl_api,
            _ => &config.tw_app_playurl_api,
        },
        false => match params.area_num {
            1 => &config.cn_web_playurl_api,
            2 => &config.hk_web_playurl_api,
            3 => &config.tw_web_playurl_api,
            4 => &config.th_web_playurl_api,
            _ => &config.tw_web_playurl_api,
        },
    };
    let body_data = match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        proxy_url,
        &params.user_agent,
        "",
    )
    .await
    {
        Ok(data) => data,
        Err(_) => {
            // println!(
            //     "[Debug] get_playurl_background getwebpage: {}\n{}\n{}\n{}",
            //     &receive_data.url,
            //     &receive_data.proxy_open,
            //     &receive_data.proxy_url,
            //     &receive_data.user_agent
            // );
            return Err("[Warning] fn get_playurl_background getwebpage error".to_string());
        }
    };
    let mut body_data_json: serde_json::Value = match serde_json::from_str(&body_data) {
        Ok(value) => value,
        Err(_) => {
            return Err("[Error] fn get_playurl_background serde_json::from_str error".to_string())
        }
    };
    remove_parameters_playurl(&playurl_type, &mut body_data_json).unwrap_or_default();

    Ok(body_data)
    // health_check not here
    // let expire_time = match config.cache.get(
    //     &body_data_json["code"]
    //         .as_i64()
    //         .unwrap_or_default()
    //         .to_string(),
    // ) {
    //     Some(value) => value,
    //     None => config.cache.get("other").unwrap_or(&1380),
    // };
    // let value = format!("{}{body_data}", ts + expire_time * 1000);
    // match redis_set(&redis, &receive_data.key, &value, *expire_time).await {
    //     Some(_) => return Ok(()),
    //     None => return Err("[Error] fn get_playurl_background redis set error".to_string()),
    // }
}

pub async fn get_upstream_bili_search(
    // query: QString,
    params: &SearchParams<'_>,
    raw_query: &QString,
    config: &BiliConfig,
    bilisender: &Arc<Sender<BackgroundTaskType>>,
) -> Result<String, String> {
    let bilisender_cl = Arc::clone(bilisender);
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(String, String)>;
    if params.is_th {
        query_vec = vec![
            // ("access_key".to_string(), access_key.to_string()),
            ("appkey".to_string(), params.app_key.to_string()),
            ("build".to_string(), params.build.to_string()),
            ("c_locale".to_string(), "zh_SG".to_string()),
            ("channel".to_string(), "master".to_string()),
            ("device".to_string(), params.device.to_string()),
            ("disable_rcmd".to_string(), "0".to_string()),
            ("fnval".to_string(), params.fnval.to_string()),
            ("fnver".to_string(), "0".to_string()),
            ("fourk".to_string(), "1".to_string()),
            ("highlight".to_string(), "1".to_string()),
            ("keyword".to_string(), params.keyword.to_string()),
            ("lang".to_string(), "hans".to_string()),
            ("mobi_app".to_string(), "bstar_a".to_string()),
            ("platform".to_string(), "android".to_string()),
            ("pn".to_string(), params.pn.to_string()),
            ("ps".to_string(), "20".to_string()),
            ("qn".to_string(), "120".to_string()),
            ("s_locale".to_string(), "zh_SG".to_string()),
            ("sim_code".to_string(), "52004".to_string()),
            ("ts".to_string(), ts_string.to_string()),
            ("type".to_string(), "7".to_string()),
        ];
        if !params.access_key.is_empty() {
            query_vec.push(("access_key".to_string(), params.access_key.to_string()));
        }
        if !params.statistics.is_empty() {
            query_vec.push(("statistics".to_string(), params.statistics.to_string()));
        }
    } else {
        if params.is_app {
            query_vec = vec![
                ("access_key".to_string(), params.access_key.to_string()),
                ("appkey".to_string(), params.app_key.to_string()),
                ("build".to_string(), params.build.to_string()),
                ("c_locale".to_string(), "zh_CN".to_string()),
                ("channel".to_string(), "master".to_string()),
                ("device".to_string(), params.device.to_string()),
                ("disable_rcmd".to_string(), "0".to_string()),
                ("fnval".to_string(), "4048".to_string()),
                ("fnver".to_string(), "0".to_string()),
                ("fourk".to_string(), "1".to_string()),
                ("highlight".to_string(), "1".to_string()),
                ("keyword".to_string(), params.keyword.to_string()),
                ("mobi_app".to_string(), "android".to_string()),
                ("platform".to_string(), "android".to_string()),
                ("pn".to_string(), params.pn.to_string()),
                ("ps".to_string(), "20".to_string()),
                ("qn".to_string(), "120".to_string()),
                ("s_locale".to_string(), "zh_CN".to_string()),
                ("ts".to_string(), ts_string.to_string()),
                ("type".to_string(), "7".to_string()),
            ];
            if !params.statistics.is_empty() {
                query_vec.push(("statistics".to_string(), params.statistics.to_string()));
            }
        } else {
            query_vec = raw_query.clone().into_pairs();
        }
    }

    query_vec.sort_by_key(|v| v.0.clone());
    //let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{}", qstring::QString::new(query_vec));
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{}", params.app_sec))
    );
    let api = match (params.area_num, params.is_app) {
        (1, true) => &config.cn_app_search_api,
        (2, true) => &config.hk_app_search_api,
        (3, true) => &config.tw_app_search_api,
        (4, true) => &config.th_app_search_api,
        (1, false) => &config.cn_web_search_api,
        (2, false) => &config.hk_web_search_api,
        (3, false) => &config.tw_web_search_api,
        (4, false) => &config.th_web_search_api,
        _ => &config.hk_app_search_api,
    };

    let (proxy_open, proxy_url) = match params.area_num {
        1 => (&config.cn_proxy_search_open, &config.cn_proxy_search_url),
        2 => (&config.hk_proxy_search_open, &config.hk_proxy_search_url),
        3 => (&config.tw_proxy_search_open, &config.tw_proxy_search_url),
        4 => (&config.th_proxy_search_open, &config.th_proxy_search_url),
        _ => (&config.hk_proxy_search_open, &config.hk_proxy_search_url),
    };

    match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        &proxy_url,
        params.user_agent,
        params.cookie,
    )
    .await
    {
        Ok(data) => {
            // TODO: 有时候, 上游啥都没返回, 程序却还是正常插入search_remake返回了, 待排查原因
            report_health(
                HealthReportType::Search(HealthData::init(
                    Area::new(params.area_num as u8),
                    true,
                    UpstreamReply {
                        proxy_open: *proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                )),
                bilisender_cl,
            )
            .await;
            Ok(data)
        }
        Err(_) => {
            report_health(
                HealthReportType::Search(HealthData::init(
                    Area::new(params.area_num as u8),
                    false,
                    UpstreamReply {
                        proxy_open: *proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                )),
                bilisender_cl,
            )
            .await;
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

            Err("{\"code\":-500,\"message\":\"服务器网络问题\"}".to_string())
        }
    }
}

pub async fn get_upstream_bili_season(
    access_key: &str,
    build: &str,
    season_id: &str,
    user_agent: &str,
    config: &BiliConfig,
) -> Result<String, ()> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec = vec![
        ("access_key", access_key),
        ("appkey", "7d089525d3611b1c"),
        ("build", build),
        ("mobi_app", "bstar_a"),
        ("season_id", season_id),
        ("s_locale", "zh_SG"),
        ("ts", &ts_string),
    ];

    query_vec.sort_by_key(|v| v.0);
    //let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{}", qstring::QString::new(query_vec));
    // 硬编码app_sec, 参考docs
    let app_sec = "acd495b248ec528c2eed1e862d393126";
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{app_sec}"))
    );
    let proxy_open = &config.th_proxy_playurl_open;
    let proxy_url = &config.th_proxy_playurl_url;
    let api = &config.th_app_season_api;
    match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        &proxy_url,
        user_agent,
        "",
    )
    .await
    {
        Ok(data) => {
            // println!("[Debug] ss_id:{}", season_id);
            // println!("[Debug] data:{}", data);
            Ok(data)
        }
        Err(_) => {
            // if config.report_open {
            //     let num = redis_get(&pool, "0441301")
            //         .await
            //         .unwrap_or("0".to_string())
            //         .as_str()
            //         .parse::<u32>()
            //         .unwrap();
            //     if num == 4 {
            //         redis_set(&pool, "0441301", "1", 0)
            //             .await
            //             .unwrap_or_default();
            //         let senddata = SendData::Health(SendHealthData {
            //             data_type: SesourceType::PlayUrl,
            //             health_type: HealthType::Offline,
            //             area_num: 4,
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
            //         redis_set(&pool, "0441301", &(num + 1).to_string(), 0)
            //             .await
            //             .unwrap_or_default();
            //     }
            // }
            Err(())
        }
    }
}

pub async fn get_upstream_bili_ep_info(
    ep_id: &str,
    proxy_open: bool,
    proxy_url: &str,
) -> Result<(EpInfo, Vec<EpInfo>), ()> {
    // 获取番剧信息
    // 1 season_id for later use
    // 2 ep need vip
    fn parse_data(value: String, ep_id: &str) -> Result<(EpInfo, Vec<EpInfo>), ()> {
        let value_json = serde_json::from_str(&value).unwrap_or(serde_json::json!({"code":-2333}));
        let mut ep_info_vec: Vec<EpInfo> = vec![];
        let mut current_ep_info: EpInfo = EpInfo {
            ..Default::default()
        };
        if value_json["code"].as_i64().unwrap_or(-2333) == 0 {
            let result_json = &value_json["result"];
            let series_title = result_json["series_title"]
                .as_str()
                .unwrap_or("N/A")
                .to_string();
            let title = result_json["title"]
                .as_str()
                .unwrap_or(series_title.as_str())
                .to_string();
            let season_id = result_json["season_id"].as_u64().unwrap_or(0);
            let episodes = &result_json["episodes"];
            for episode in episodes.as_object() {
                let episode_ep_id = episode["ep_id"].as_u64().unwrap_or(0);
                let episode_need_vip = {
                    if episode.contains_key("badge") && episode.contains_key("badge_type") {
                        // DEBUG
                        println!(
                            "Detect EP {episode_ep_id} need vip: badge {} badge_type {}",
                            episode["badge"].as_str().unwrap_or("N/A"),
                            episode["badge_type"].as_str().unwrap_or("N/A")
                        );
                        true
                    } else {
                        false
                    }
                };
                let ep_info = EpInfo {
                    ep_id: episode_ep_id,
                    need_vip: episode_need_vip,
                    title: title.clone(),
                    season_id,
                };
                if ep_id.parse::<u64>().unwrap_or(0) == episode_ep_id {
                    current_ep_info = ep_info.clone();
                }
                ep_info_vec.push(ep_info);
            }
            Ok((current_ep_info, ep_info_vec))
        } else {
            return Err(());
        }
    }
    let bili_hidden_season_api =
        format!("https://bangumi.bilibili.com/view/web_api/season?ep_id={ep_id}");
    let bili_season_api = format!("http://api.bilibili.com/pgc/view/web/season?ep_id={ep_id}");
    let user_agent = "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
    match async_getwebpage(
        &bili_hidden_season_api,
        &proxy_open,
        proxy_url,
        user_agent,
        "",
    )
    .await
    {
        Ok(value) => match parse_data(value, ep_id) {
            Ok(value) => Ok(value),
            Err(_) => {
                match async_getwebpage(&bili_season_api, &proxy_open, proxy_url, user_agent, "")
                    .await
                {
                    Ok(value) => match parse_data(value, ep_id) {
                        Ok(value) => Ok(value),
                        Err(_) => Err(()),
                    },
                    Err(_) => Err(()),
                }
            }
        },
        Err(_) => Err(()),
    }
}
