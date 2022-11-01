use super::health::report_health;
use super::request::{async_getwebpage, async_postwebpage};
use super::tools::{check_playurl_need_vip, remove_parameters_playurl};
use super::types::{
    Area, BiliRuntime, EType, EpInfo, HealthData, HealthReportType, PlayurlParams,
    PlayurlParamsStatic, ReqType, SearchParams, UpstreamReply, UserCerinfo, UserInfo,
    UserResignInfo,
};
use chrono::prelude::*;
use md5;
use qstring::QString;
use std::string::String;

pub async fn get_upstream_bili_account_info(
    access_key: &str,
    appkey: &str,
    appsec: &str,
    user_agent: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<UserInfo, EType> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_min = dt.timestamp() as u64;
    let sign = md5::compute(format!(
        "access_key={}&appkey={}&ts={}{}",
        access_key, appkey, ts_min, appsec
    ));
    let url: String = format!(
        "https://app.bilibili.com/x/v2/account/myinfo?access_key={}&appkey={}&ts={}&sign={:x}",
        access_key, appkey, ts_min, sign
    );
    //println!("{}",url);
    let output = match async_getwebpage(
        &url,
        bili_runtime.config.cn_proxy_accesskey_open,
        &bili_runtime.config.cn_proxy_accesskey_url,
        user_agent,
        "",
    )
    .await
    {
        Ok(data) => data,
        Err(value) => {
            println!("getuser_list函数寄了 url:{}",url);
            // TODO: add error report
            return Err(value);
        }
    };

    //println!("{}",output);
    let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();
    let code = if let Some(value) = output_json["code"].as_i64() {
        value
    } else {
        println!("[USER INFO] Parsing Upstream reply failed, Upstream Reply -> {}", output);
        return Err(EType::ServerGeneral);
    };
    match code {
        0 => {
            let output_struct = UserInfo {
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
            };
            // TODO: Add Cache: update_cached_user_info
            Ok(output_struct)
        }
        -400 => {
            println!("[USER INFO] AK {} | Get UserInfo failed -400. REQ Params -> APPKEY {} | TS {} | APP_SEC {} | SIGN {:?}. Upstream Reply -> {}",
                access_key, appkey, ts_min, appsec, sign, output_json
            );
            Err(EType::OtherError(-400, "可能你用的不是手机"))
        }
        -101 => {
            println!(
                "[USER INFO] AK {} | Get UserInfo failed -101. Upstream Reply -> {}",
                access_key, output_json
            );
            Err(EType::UserNotLoginedError)
        }
        -3 => {
            println!("[USER INFO] AK {} | Get UserInfo failed -3. REQ Params -> APPKEY {} | TS {} | APP_SEC {} | SIGN {:?}. Upstream Reply -> {}",
                access_key, appkey, ts_min, appsec, sign, output_json
            );
            Err(EType::ReqSignError)
        }
        -412 => {
            println!(
                "[USER INFO] AK {} | Get UserInfo failed -412. Upstream Reply -> {}",
                access_key, output_json
            );
            Err(EType::ServerFatalError)
        }
        _ => {
            println!("[USER INFO] AK {} | Get UserInfo failed. REQ Params -> APPKEY {} | TS {} | APP_SEC {} | SIGN {:?}. Upstream Reply -> {}",
                access_key, appkey, ts_min, appsec, sign, output_json
            );
            Err(EType::OtherUpstreamError(
                code,
                // //我写的什么勾巴代码...
                output_json["message"].as_str().unwrap_or("NULL").to_string(),
            ))
        }
    }
}

pub async fn get_upstream_blacklist_info(
    uid: &u64,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<UserCerinfo, EType> {
    // // currently upstream only support query using uid...
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    //let user_cerinfo_str = String::new();
    let user_agent = format!("biliroaming-rust-server/{}", env!("CARGO_PKG_VERSION"));
    let api = match &bili_runtime.config.blacklist_config {
        super::types::BlackListType::OnlyOnlineBlackList(value) => &value.api,
        super::types::BlackListType::MixedBlackList(value) => &value.api,
        _ => return Err(EType::ServerGeneral),
    };
    let getwebpage_data =
        match async_getwebpage(&format!("{api}{uid}"), false, "", &user_agent, "").await {
            Ok(data) => data,
            Err(_) => return Err(EType::ServerNetworkError("鉴权失败了喵")),
        };
    let getwebpage_json: serde_json::Value = match serde_json::from_str(&getwebpage_data) {
        Ok(value) => value,
        Err(_) => {
            // let return_data = UserCerinfo {
            //     uid: uid.clone(),
            //     black: true,
            //     white: false,
            //     ban_until: 0,
            //     status_expire_time: 0,
            // };
            println!("[Error] 请接入在线黑名单");
            return Err(EType::ServerReqError(
                "Blacklist Server Internal Error Json",
            ));
        }
    };
    let code = getwebpage_json["code"].as_i64().unwrap_or(233);
    if code == 0 {
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
        // TODO: add cache here
        return Ok(return_data);
    } else {
        println!("鉴权失败: UID {uid}, 上游返回 {getwebpage_data}");
        return Err(EType::ServerReqError("鉴权失败了喵, Blacklist Server Error"));
    }
}

pub async fn get_upstream_bili_playurl(
    // query: QString,
    params: &mut PlayurlParams<'_>,
    _user_info: &UserInfo,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<String, EType> {
    // let bilisender_cl = Arc::clone(bilisender);
    // generate api info & proxy_info, for later adding proxy balance
    let config = bili_runtime.config;
    let req_type = ReqType::Playurl(Area::new(params.area_num), params.is_app);
    let api = req_type.get_api(config);
    let (proxy_open, proxy_url) = req_type.get_proxy(config);
    let playurl_type = params.get_playurl_type();
    // generate req params
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(&str, &str)>;
    if params.is_tv {
        query_vec = vec![
            ("access_key", &params.access_key[..]),
            ("appkey", params.appkey),
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
            ("appkey", params.appkey),
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
    if !params.ep_id.is_empty() {
        query_vec.push(("ep_id", params.ep_id));
    }
    if !params.cid.is_empty() {
        query_vec.push(("cid", params.cid));
    }
    if params.is_th {
        query_vec.push(("s_locale", "zh_SG"));
    }

    query_vec.sort_by_key(|v| v.0);
    let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{unsigned_url}");
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{}", params.appsec))
    );
    // finish generating req params
    let body_data = match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        proxy_url,
        params.user_agent,
        "",
    )
    .await
    {
        Ok(data) => data,
        Err(value) => return Err(value),
    };
    let mut body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
    let code = body_data_json["code"].as_i64().unwrap().clone();
    remove_parameters_playurl(&playurl_type, &mut body_data_json).unwrap_or_default();

    // cache playurl
    // update_cached_playurl(params, &body_data, redis_pool, bilisender);
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
            proxy_open,
            // .clone used here may do harm to perf for such func is used frequently
            // as biliconfig lives much longer, why not use String::from to create a new String?
            // proxy_url: String::from(proxy_url.as_str()),
            proxy_url: proxy_url.to_string(),
        },
    ));
    report_health(health_report_data, bili_runtime).await;
    // check user's vip status
    if !params.is_vip {
        // TODO: add vip only feature here
        if let Ok(value) = check_playurl_need_vip(playurl_type, &body_data_json) {
            if value {
                // let bilisender_cl = Arc::clone(bilisender);
                // update_cached_user_info_background(params.access_key.to_string(), bilisender_cl)
                //     .await
            }
        }
        // TODO: add fallback check
    }
    Ok(body_data_json.to_string())
}

pub async fn get_upstream_bili_playurl_background(
    params: &PlayurlParamsStatic,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<String, String> {
    let config = bili_runtime.config;
    let req_type = ReqType::Playurl(Area::new(params.area_num), params.is_app);
    let api = req_type.get_api(config);
    let (proxy_open, proxy_url) = req_type.get_proxy(config);
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(&str, &str)>;
    let playurl_type = params.get_playurl_type();
    if params.is_tv {
        query_vec = vec![
            ("access_key", &params.access_key[..]),
            ("appkey", &params.appkey),
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
            ("appkey", &params.appkey),
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
    //         // appkey = "7d089525d3611b1c";
    //         // appsec = appkey_to_sec(&appkey).unwrap();
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
        md5::compute(format!("{unsigned_url}{}", params.appsec))
    );
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
    bili_runtime: &BiliRuntime<'_>,
) -> Result<serde_json::Value, EType> {
    let config = bili_runtime.config;
    let req_type = ReqType::Search(Area::new(params.area_num as u8), params.is_app);
    let api = req_type.get_api(config);
    let (proxy_open, proxy_url) = req_type.get_proxy(config);
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(String, String)>;
    if params.is_th {
        query_vec = vec![
            // ("access_key".to_string(), access_key.to_string()),
            ("appkey".to_string(), params.appkey.to_string()),
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
                ("appkey".to_string(), params.appkey.to_string()),
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
        md5::compute(format!("{unsigned_url}{}", params.appsec))
    );
    match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        proxy_url,
        params.user_agent,
        params.cookie,
    )
    .await
    {
        Ok(data) => {
            // TODO: 有时候, 上游啥都没返回, 程序却还是正常插入search_remake返回了, 待排查原因
            let data_json: serde_json::Value = serde_json::from_str(&data).unwrap();
            let upstream_code = data_json["code"]
                .as_i64()
                .unwrap_or(233);
            if upstream_code == 0 {
                Ok(data_json)
            } else {
                let upstream_message = data_json["message"].as_str().unwrap_or("NULL");
                println!("[SEARCH] Upstream ERROR {upstream_code}: {data_json}");
                report_health(
                    HealthReportType::Search(HealthData::init(
                        Area::new(params.area_num as u8),
                        true,
                        UpstreamReply {
                            code: upstream_code,
                            message: upstream_message.to_string(),
                            proxy_open: proxy_open,
                            proxy_url: String::from(proxy_url),
                            ..Default::default()
                        },
                    )),
                    bili_runtime,
                )
                .await;
                Err(EType::ServerReqError("上游错误"))
            }
        }
        Err(_) => {
            report_health(
                HealthReportType::Search(HealthData::init(
                    Area::new(params.area_num as u8),
                    false,
                    UpstreamReply {
                        proxy_open: proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                )),
                bili_runtime,
            )
            .await;
            Err(EType::ServerNetworkError("连接上游失败"))
        }
    }
}

pub async fn get_upstream_bili_subtitle(
    params: &PlayurlParams<'_>,
    raw_query: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<String, EType> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let mut query = QString::from(raw_query);
    query.add_str(&format!(
        "&appkey={}&mobi_app=bstar_a&s_locale=zh_SG&ts={ts}",
        params.appkey
    ));
    let mut query_vec = query.to_pairs();
    query_vec.sort_by_key(|v| v.0);
    // 硬编码app_sec
    let app_sec = params.appsec;
    let proxy_open = bili_runtime.config.th_proxy_subtitle_open;
    let proxy_url = &bili_runtime.config.th_proxy_subtitle_url;
    let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{unsigned_url}");
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{app_sec}"))
    );
    let api = "https://app.biliintl.com/intl/gateway/v2/app/subtitle";
    return async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        proxy_url,
        params.user_agent,
        "",
    )
    .await;
}

pub async fn get_upstream_bili_season(
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<String, EType> {
    let config = bili_runtime.config;
    let req_type = ReqType::ThSeason;
    let api = req_type.get_api(config);
    let (proxy_open, proxy_url) = req_type.get_proxy(config);

    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec = vec![
        ("access_key", params.access_key),
        ("appkey", params.appkey),
        ("build", params.build),
        ("mobi_app", "bstar_a"),
        ("season_id", params.season_id),
        ("s_locale", "zh_SG"),
        ("ts", &ts_string),
    ];

    query_vec.sort_by_key(|v| v.0);
    //let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{}", qstring::QString::new(query_vec));
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{}", params.appsec))
    );

    match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        proxy_url,
        params.user_agent,
        "",
    )
    .await
    {
        Ok(data) => {
            // println!("[Debug] ss_id:{}", season_id);
            // println!("[Debug] data:{}", data);
            Ok(data)
        }
        Err(value) => {
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
            Err(value)
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
        proxy_open,
        proxy_url,
        user_agent,
        "",
    )
    .await
    {
        Ok(value) => match parse_data(value, ep_id) {
            Ok(value) => Ok(value),
            Err(_) => {
                match async_getwebpage(&bili_season_api, proxy_open, proxy_url, user_agent, "")
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

pub async fn get_upstream_resigned_access_key(
    area_num: &u8,
    user_agent: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Option<(String, u64)> {
    async fn get_accesskey_from_token_th(
        user_agent: &str,
        bili_runtime: &BiliRuntime<'_>,
    ) -> Option<(String, u64)> {
        let dt = Local::now();
        let ts = dt.timestamp() as u64;
        let resign_info =
            to_resign_info(&bili_runtime.redis_get(&format!("a41101")).await.unwrap()).await;
        let access_key = resign_info.access_key;
        let refresh_token = resign_info.refresh_token;
        let url = "https://passport.biliintl.com/x/intl/passport-login/oauth2/refresh_token";
        let content = format!("access_token={access_key}&refresh_token={refresh_token}");
        let proxy_open = bili_runtime.config.th_proxy_token_open;
        let proxy_url = &bili_runtime.config.th_proxy_token_url;
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
        bili_runtime
            .redis_set("a41101", &resign_info.to_json(), 0)
            .await;
        Some((resign_info.access_key, resign_info.expire_time))
    }

    async fn get_accesskey_from_token_cn(
        _area_num: &u8,
        user_agent: &str,
        bili_runtime: &BiliRuntime<'_>,
    ) -> Option<(String, u64)> {
        let dt = Local::now();
        let ts = dt.timestamp() as u64;
        let resign_info =
            to_resign_info(&bili_runtime.redis_get(&format!("a11101")).await.unwrap()).await;
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
        let proxy_open = bili_runtime.config.cn_proxy_token_open;
        let proxy_url = &bili_runtime.config.cn_proxy_token_url;
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
        bili_runtime
            .redis_set("a11101", &resign_info.to_json(), 0)
            .await;
        Some((resign_info.access_key, resign_info.expire_time))
    }

    async fn to_resign_info(resin_info_str: &str) -> UserResignInfo {
        serde_json::from_str(resin_info_str).unwrap()
    }

    let config = bili_runtime.config;
    if *config
        .resign_api_policy
        .get(&area_num.to_string())
        .unwrap_or(&false)
    {
        let key = format!("a{area_num}1201");
        let dt = Local::now();
        let ts = dt.timestamp() as u64;
        match bili_runtime.redis_get(&key).await {
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
        let webgetpage_data = if let Ok(data) = async_getwebpage(&url, false, "", "", "").await {
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

        bili_runtime
            .redis_set(&key, &resign_info.to_json(), 3600)
            .await;
        return Some((access_key, resign_info.expire_time));
    } else {
        let area_num: u8 = match area_num {
            4 => 4,
            _ => 1,
        };
        let resign_info_str = match bili_runtime.redis_get(&format!("a{area_num}1101")).await {
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
                4 => get_accesskey_from_token_th(user_agent, bili_runtime).await,
                _ => get_accesskey_from_token_cn(&area_num, user_agent, bili_runtime).await,
            }
        }
    }
}
