use crate::mods::background_tasks::update_cached_ep_vip_status_background;
use crate::mods::ep_info::get_ep_need_vip;
use crate::mods::tools::check_vip_status_from_playurl;

use super::background_tasks::update_cached_user_info_background;
use super::cache::{
    update_area_cache, update_blacklist_info_cache, update_cached_playurl, update_th_season_cache,
    update_th_subtitle_cache, update_user_info_cache,
};
use super::health::report_health;
use super::request::{async_getwebpage, async_postwebpage};
use super::tools::{get_mobi_app, remove_parameters_playurl};
use super::types::{
    Area, BiliRuntime, EType, EpInfo, HealthData, HealthReportType, PlayurlParams, ReqType,
    SearchParams, UpstreamReply, UserCerinfo, UserInfo, UserResignInfo,
};
use chrono::prelude::*;
use curl::easy::List;
use log::{debug, error};
use md5;
use qstring::QString;
use std::string::String;

pub async fn get_upstream_bili_account_info(
    access_key: &str,
    _appkey: &str,
    _appsec: &str,
    is_app: bool,
    // is_th: bool,
    user_agent: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<UserInfo, EType> {
    use rand::Rng;
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_min = dt.timestamp() as u64;
    let ts_min_string = ts_min.to_string();
    let appkey = { 
        // Android请求用Android端的appkey, 网页用的ios的appkey (https://github.com/SocialSisterYi/bilibili-API-collect/issues/393#issuecomment-1288749103)
        // 不知道有没有必要这样搞
        if is_app {
            "783bbb7264451d82"
        }else{
            "27eb53fc9058f8c3"
        }
    };
    
    let (appkey, appsec, mobi_app) = get_mobi_app(appkey);
    let rand_string_36 = {
        let words: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        (0..36)
        .map(|_| {
            let idx = rng.gen_range(0..words.len());
            words[idx] as char
        }).collect::<String>()
    };
    let mut req_vec = vec![ //以防万一，昨天抓了下包尽可能补全
        ("access_key", access_key),
        ("appkey", appkey),
        ("build", "6800300"),
        ("buvid", &rand_string_36),
        ("c_locale", "zh_CN"),
        ("channel", "master"),
        ("disable_rcmd", "0"),
        ("local_id",&rand_string_36),
        ("mobi_app",mobi_app),
        ("platform", "android"),
        ("s_locale","zh_CN"),
        ("statistics","%7B%22appId%22%3A1%2C%22platform%22%3A3%2C%22version%22%3A%226.80.0%22%2C%22abtest%22%3A%22%22%7D"),
        ("ts", &ts_min_string),
    ];
    req_vec.sort_by_key(|v| v.0);
    let req_params = qstring::QString::new(req_vec);

    let mut headers = List::new();
    headers.append("x-bili-aurora-zone: sh001").unwrap();

    let sign = md5::compute(req_params.to_string() + appsec);
    let url: String = format!(
        "https://app.bilibili.com/x/v2/account/myinfo?{}&sign={:x}",
        req_params.to_string(), sign
    );
    debug!(
        "[GET USER_INFO][U] AK {} | RAW QUERY -> APPKEY {} TS {} APPSEC {}",
        access_key, appkey, ts_min, appsec
    );
    debug!(
        "[GET USER_INFO][U] URL {}",
        url
    );
    let output = match async_getwebpage(
        &url,
        bili_runtime.config.cn_proxy_accesskey_open,
        &bili_runtime.config.cn_proxy_accesskey_url,
        user_agent,
        "",
        Some(headers),
    )
    .await
    {
        Ok(data) => data,
        Err(value) => {
            error!(
                "[GET USER_INFO][U] AK {} | Req failed. Network Problems. RAW QUERY -> APPKEY {} TS {} APPSEC {} Use Proxy {} - {}",
                access_key, appkey, ts_min, appsec, bili_runtime.config.cn_proxy_accesskey_open, &bili_runtime.config.cn_proxy_accesskey_url,
            );
            let health_report_type = HealthReportType::Others(HealthData {
                area_num: 0,
                is_200_ok: false,
                upstream_reply: UpstreamReply {
                    proxy_open: bili_runtime.config.cn_proxy_accesskey_open,
                    proxy_url: bili_runtime.config.cn_proxy_accesskey_url.clone(),
                    ..Default::default()
                },
                is_custom: true,
                custom_message: "[GET USERINFO][U] 致命错误! 获取用户信息失败!".to_owned(),
            });
            report_health(health_report_type, bili_runtime).await;
            return Err(value);
        }
    };

    debug!(
        "[GET USER_INFO][U] AK {} | Upstream Reply: {}",
        access_key, output
    );
    let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();
    let code = if let Some(value) = output_json["code"].as_i64() {
        value
    } else {
        debug!(
            "[GET USER_INFO][U] AK {} | Parsing Upstream reply failed, Upstream Reply -> {}",
            access_key, output
        );
        let health_report_type = HealthReportType::Others(HealthData {
            area_num: 0,
            is_200_ok: true,
            upstream_reply: UpstreamReply {
                proxy_open: bili_runtime.config.cn_proxy_accesskey_open,
                proxy_url: bili_runtime.config.cn_proxy_accesskey_url.clone(),
                ..Default::default()
            },
            is_custom: true,
            custom_message: format!("致命错误! 解析用户信息失败!\n上游返回: {output_json}"),
        });
        report_health(health_report_type, bili_runtime).await;
        return Err(EType::ServerGeneral);
    };
    match code {
        0 => {
            let output_struct = UserInfo {
                code: 0,
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
            update_user_info_cache(&output_struct, bili_runtime).await;
            Ok(output_struct)
        }
        -404 => {
            // 有几个unknown的appkey会导致 {"code":-404,"message":"啥都木有","ttl":1}, 直接返回invalidreq得了
            match appkey {
                "84956560bc028eb7" | "85eb6835b0a1034e" => {
                    // 还是迂回更新其用户信息试一下
                    update_cached_user_info_background(access_key.to_string(), bili_runtime).await;
                    Err(EType::OtherError(
                        -10403,
                        "不兼容的APPKEY, 请升级油猴脚本或其他你正在用的客户端!",
                    ))
                }
                _ => {
                    error!("[GET USER_INFO][U] AK {} -> Get UserInfo failed. Invalid APPKEY -> APPKEY {} | TS {} | APPSEC {}. Upstream Reply -> {}",
                        access_key, appkey, ts_min, appsec, output_json
                    );
                    let health_report_type = HealthReportType::Others(HealthData {
                        area_num: 0,
                        is_200_ok: true,
                        upstream_reply: UpstreamReply {
                            code,
                            message: output_json["message"].as_str().unwrap_or("null").to_owned(),
                            proxy_open: bili_runtime.config.cn_proxy_accesskey_open,
                            proxy_url: bili_runtime.config.cn_proxy_accesskey_url.clone(),
                        },
                        is_custom: true,
                        custom_message: format!(
                            "[GET USER_INFO][U] 疑似不能用于获取用户信息的APPKEY {appkey} - APPSEC {appsec}"
                        ),
                    });
                    report_health(health_report_type, bili_runtime).await;
                    Err(EType::InvalidReq)
                }
            }
        }
        -400 => {
            error!("[GET USER_INFO][U] AK {} | Get UserInfo failed -400. REQ Params -> APPKEY {} | TS {} | APPSEC {} | SIGN {:?}. Upstream Reply -> {}",
                access_key, appkey, ts_min, appsec, sign, output_json
            );
            Err(EType::OtherError(-400, "可能你用的不是手机"))
        }
        -101 => {
            let output_struct = UserInfo {
                code,
                expire_time: ts + 1 * 60 * 60 * 1000, // 未登录缓存1h,防止高频请求b站服务器
                ..Default::default()
            };
            update_user_info_cache(&output_struct, bili_runtime).await;
            error!(
                "[GET USER_INFO][U] AK {} | Get UserInfo failed -101. Upstream Reply -> {}",
                access_key, output_json
            );
            Err(EType::UserNotLoginedError)
        },
        61000 => { // 那先看成未登录
            let output_struct = UserInfo {
                code,
                expire_time: ts + 10 * 60 * 1000, // 看起来不是请求过快, 并且后续应该无法使用这个accesskey获取到用户信息了,所以先暂时缓存10m
                ..Default::default()
            };
            update_user_info_cache(&output_struct, bili_runtime).await;
            error!(
                "[GET USER_INFO][U] AK {} | Get UserInfo failed 61000. Maybe AK out of date. Upstream Reply -> {}",
                access_key, output_json
            );
            Err(EType::UserLoginInvalid)
        }
        -3 => {
            error!("[GET USER_INFO][U] AK {} | Get UserInfo failed -3. REQ Params -> APPKEY {} | TS {} | APPSEC {} | SIGN {:?}. Upstream Reply -> {}",
                access_key, appkey, ts_min, appsec, sign, output_json
            );
            Err(EType::ReqSignError)
        }
        -412 => {
            error!(
                "[GET USER_INFO][U] AK {} | Get UserInfo failed -412. Upstream Reply -> {}",
                access_key, output_json
            );
            let health_report_type = HealthReportType::Others(HealthData {
                area_num: 0,
                is_200_ok: true,
                upstream_reply: UpstreamReply {
                    code: -412,
                    message: output_json["message"].as_str().unwrap_or("null").to_owned(),
                    proxy_open: bili_runtime.config.cn_proxy_accesskey_open,
                    proxy_url: bili_runtime.config.cn_proxy_accesskey_url.clone(),
                },
                is_custom: true,
                custom_message: format!(
                    "[GET USER_INFO][U] 致命错误! 机子-412喵! \n上游返回: {output_json}"
                ),
            });
            report_health(health_report_type, bili_runtime).await;
            Err(EType::ServerFatalError)
        }
        -663 => {
            error!(
                "[GET USER_INFO][U] AK {} | Get UserInfo failed -663. Maybe req too often. Upstream Reply -> {}",
                access_key, output_json
            );
            update_cached_user_info_background(access_key.to_string(), bili_runtime).await;
            let health_report_type = HealthReportType::Others(HealthData {
                area_num: 0,
                is_200_ok: true,
                upstream_reply: UpstreamReply {
                    code,
                    message: output_json["message"].as_str().unwrap_or("null").to_owned(),
                    proxy_open: bili_runtime.config.cn_proxy_accesskey_open,
                    proxy_url: bili_runtime.config.cn_proxy_accesskey_url.clone(),
                },
                is_custom: true,
                custom_message: format!(
                    "[GET USER_INFO][U] -663错误! \n可能被鼠鼠限制请求频率了, 正在研究处理方法...\nDevice: {}, APPKEY: {}, AK: {}, TS: {}",
                    mobi_app, appkey, access_key, ts
                ),
            });
            report_health(health_report_type, bili_runtime).await;
            Err(EType::OtherError(
                -412,
                "服务器内部请求被鼠鼠限频了, 请等待若干秒后重试",
            ))
        }
        _ => {
            error!("[GET USER_INFO][U] AK {} -> Get UserInfo failed. REQ Params -> APPKEY {} | TS {} | APPSEC {} | SIGN {:?}. Upstream Reply -> {}",
                access_key, appkey, ts_min, appsec, sign, output_json
            );
            error!("[GET USER_INFO][U] URL {}",url);
            let health_report_type = HealthReportType::Others(HealthData {
                area_num: 0,
                is_200_ok: true,
                upstream_reply: UpstreamReply {
                    code,
                    message: output_json["message"].as_str().unwrap_or("null").to_owned(),
                    proxy_open: bili_runtime.config.cn_proxy_accesskey_open,
                    proxy_url: bili_runtime.config.cn_proxy_accesskey_url.clone(),
                },
                is_custom: true,
                custom_message: format!(
                    "[GET USER_INFO][U] 致命错误! 未知的错误码! \n上游返回: {output_json}"
                ),
            });
            report_health(health_report_type, bili_runtime).await;
            Err(EType::OtherUpstreamError(
                code,
                output_json["message"]
                    .as_str()
                    .unwrap_or("NULL")
                    .to_string(),
            ))
        }
    }
}

pub async fn get_upstream_blacklist_info(
    user_info: &UserInfo,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<UserCerinfo, EType> {
    // // currently upstream only support query using uid...
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let uid = user_info.uid;
    if uid == 0 {
        // 仅当出问题才会有uid=0
        return Err(EType::ServerGeneral);
    }
    //let user_cerinfo_str = String::new();
    let user_agent = format!("biliroaming-rust-server/{}", env!("CARGO_PKG_VERSION"));
    let api = match &bili_runtime.config.blacklist_config {
        super::types::BlackListType::OnlyOnlineBlackList(value) => &value.api,
        super::types::BlackListType::MixedBlackList(value) => &value.api,
        _ => return Err(EType::ServerGeneral),
    };
    let getwebpage_data =
        match async_getwebpage(&format!("{api}{uid}"), false, "", &user_agent, "", None).await {
            Ok(data) => data,
            Err(_) => {
                error!("[GET USER_CER_INFO][U] 服务器网络问题");
                let health_report_type = HealthReportType::Others(HealthData {
                    area_num: 0,
                    is_200_ok: false,
                    upstream_reply: UpstreamReply {
                        ..Default::default()
                    },
                    is_custom: true,
                    custom_message: format!(
                        "[GET USER_CER_INFO][U] 致命错误! 请求黑名单失败: 网络问题! "
                    ),
                });
                report_health(health_report_type, bili_runtime).await;
                return Err(EType::ServerNetworkError("鉴权失败了喵"));
            }
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
            error!("[GET USER_CER_INFO][U] 上游返回好像不是JSON... 是不是没接入公共黑名单?");
            debug!("[GET USER_CER_INFO][U] 解析上游返回数据错误: {getwebpage_data}");
            let health_report_type = HealthReportType::Others(HealthData {
                area_num: 0,
                is_200_ok: true,
                upstream_reply: UpstreamReply {
                    ..Default::default()
                },
                is_custom: true,
                custom_message: format!(
                    "[GET USER_CER_INFO][U] 致命错误! 解析Json数据失败: {getwebpage_data}"
                ),
            });
            report_health(health_report_type, bili_runtime).await;
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
        // debug!("[GET USER_CER_INFO][U] UID {} | Upstream UID {}", uid, return_data.uid);
        update_blacklist_info_cache(user_info, &return_data, bili_runtime).await;
        return Ok(return_data);
    } else {
        error!("[GET USER_CER_INFO][U] UID {uid} -> 鉴权失败: 上游返回 {getwebpage_data}");
        let health_report_type = HealthReportType::Others(HealthData {
            area_num: 0,
            is_200_ok: true,
            upstream_reply: UpstreamReply {
                code,
                ..Default::default()
            },
            is_custom: true,
            custom_message: format!("[GET USER_CER_INFO][U] 致命错误! 上游返回: {getwebpage_data}"),
        });
        report_health(health_report_type, bili_runtime).await;
        return Err(EType::ServerReqError(
            "鉴权失败了喵, Blacklist Server Error",
        ));
    }
}

pub async fn get_upstream_bili_playurl(
    // query: QString,
    params: &mut PlayurlParams<'_>,
    user_info: &UserInfo,
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
    if params.ep_id.is_empty() {
        return Err(EType::InvalidReq);
    } else {
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
        None,
    )
    .await
    {
        Ok(data) => data,
        Err(value) => {
            error!(
                "[GET PLAYURL][U] AREA {} | EP {} | PROXY_OPEN {} | PROXY_URL {} -> 获取播放链接失败: 网络问题",
                params.area.to_ascii_uppercase(), params.ep_id, proxy_open, proxy_url
            );
            report_health(
                HealthReportType::Playurl(HealthData::init(
                    Area::new(params.area_num),
                    false,
                    UpstreamReply {
                        proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                    params.ep_id,
                )),
                bili_runtime,
            )
            .await;
            return Err(value);
        }
    };
    let mut body_data_json: serde_json::Value = match serde_json::from_str(&body_data) {
        Ok(value) => value,
        Err(_) => {
            report_health(
                HealthReportType::Playurl(HealthData {
                    area_num: params.area_num,
                    is_200_ok: true,
                    upstream_reply: UpstreamReply {
                        proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                    is_custom: true,
                    custom_message: format!(
                        "[DEBUG] 请求链: APPKEY {} | APPSEC {} | TS {} | FINAL {}?{}\n实际返回信息: \n{}",
                        params.appkey, params.appsec, ts, api, signed_url, body_data
                    ),
                }),
                bili_runtime,
            )
            .await;
            error!(
                "[GET PLAYURL][U] AREA {} | EP {} | PROXY_OPEN {} | PROXY_URL {} -> 获取播放链接失败: 网络问题",
                params.area.to_ascii_uppercase(), params.ep_id, proxy_open, proxy_url
            );
            return Err(EType::ServerGeneral);
        }
    };
    let code = body_data_json["code"].as_i64().unwrap().clone();
    remove_parameters_playurl(&playurl_type, &mut body_data_json).unwrap_or_default();

    update_area_cache(&body_data_json, params, bili_runtime).await;
    // report health
    let message = body_data_json["message"]
        .as_str()
        .unwrap_or("Error on parsing Json Response")
        .to_string();
    report_health(
        HealthReportType::Playurl(HealthData::init(
            Area::new(params.area_num),
            true,
            UpstreamReply {
                code,
                message,
                proxy_open,
                proxy_url: String::from(proxy_url),
            },
            params.ep_id,
        )),
        bili_runtime,
    )
    .await;
    // check user's vip status
    if !params.is_vip {
        // TODO: add vip only feature here
        if let Ok(value) = check_vip_status_from_playurl(playurl_type, &body_data_json) {
            if value {
                update_cached_user_info_background(params.access_key.to_string(), bili_runtime)
                    .await;
                match get_ep_need_vip(params.ep_id, bili_runtime).await {
                    Some(ep_need_vip) => {
                        if ep_need_vip == 1 {
                            update_cached_ep_vip_status_background(
                                true,
                                vec![EpInfo {
                                    ep_id: params.ep_id.parse::<u64>().unwrap_or(233),
                                    ..Default::default()
                                }],
                                bili_runtime,
                            )
                            .await;
                        }
                        error!(
                            "[GET PLAYURL][U] UID {} | AK {} | AREA {} | EP {} -> 非大会员用户获取了大会员独享视频, 可能大会员状态变动或限免, 并且尝试更新ep_need_vip成功",
                            user_info.uid, user_info.access_key, params.area.to_ascii_uppercase(), params.ep_id
                        );
                    }
                    None => {
                        error!(
                            "[GET PLAYURL][U] UID {} | AK {} | AREA {} | EP {} -> 非大会员用户获取了大会员独享视频, 可能大会员状态变动或限免, 并且尝试更新ep_need_vip失败",
                            user_info.uid, user_info.access_key, params.area.to_ascii_uppercase(), params.ep_id
                        );
                    }
                }

                report_health(
                    HealthReportType::Playurl(HealthData {
                        area_num: params.area_num,
                        is_200_ok: true,
                        upstream_reply: UpstreamReply {
                            code,
                            proxy_open,
                            proxy_url: proxy_url.to_owned(),
                            ..Default::default()
                        },
                        is_custom: true,
                        custom_message: format!("[GET PLAYURL][U] EP {} -> 非大会员用户获取了大会员独享视频. 可能限免, 请人工核实...", params.ep_id),
                    }),
                    bili_runtime,
                )
                .await;
                return Err(EType::OtherError(
                    -10403,
                    "检测到可能刚刚买了带会员, 刷新缓存中, 请稍后重试喵",
                ));
            }
        }
        // TODO: add fallback check
    }
    // update playurl cache
    let final_data = body_data_json.to_string();
    update_cached_playurl(params, &final_data, bili_runtime).await;
    debug!(
        "[GET PLAYURL][U] UID {} | AK {} | AREA {} | EP {} -> 获取成功",
        user_info.uid,
        user_info.access_key,
        params.area.to_ascii_uppercase(),
        params.ep_id
    );
    Ok(final_data)
}

pub async fn get_upstream_bili_playurl_background(
    params: &mut PlayurlParams<'_>,
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
    if params.ep_id.is_empty() {
        return Err(EType::OtherError(-10403, "无EP_ID"));
    } else {
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
        None,
    )
    .await
    {
        Ok(data) => data,
        Err(value) => {
            error!(
                "[GET PLAYURL BACKGROUND][U] AREA {} | EP {} | PROXY_OPEN {} | PROXY_URL {} -> 获取播放链接失败: 网络问题",
                params.area.to_ascii_uppercase(), params.ep_id, proxy_open, proxy_url
            );
            report_health(
                HealthReportType::Playurl(HealthData::init(
                    Area::new(params.area_num),
                    false,
                    UpstreamReply {
                        proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                    params.ep_id,
                )),
                bili_runtime,
            )
            .await;
            return Err(value);
        }
    };
    let mut body_data_json: serde_json::Value = match serde_json::from_str(&body_data) {
        Ok(value) => value,
        Err(_) => {
            report_health(
                HealthReportType::Playurl(HealthData {
                    area_num: params.area_num,
                    is_200_ok: true,
                    upstream_reply: UpstreamReply {
                        proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                    is_custom: true,
                    custom_message: format!(
                        "[GET PLAYURL BACKGROUND] [DEBUG] 请求链: APPKEY {} | APPSEC {} | TS {} | FINAL {}?{}\n实际返回信息: \n{}",
                        params.appkey, params.appsec, ts, api, signed_url, body_data
                    ),
                }),
                bili_runtime,
            )
            .await;
            error!(
                "[GET PLAYURL BACKGROUND][U] AREA {} | EP {} | PROXY_OPEN {} | PROXY_URL {} -> 获取播放链接失败: 网络问题",
                params.area.to_ascii_uppercase(), params.ep_id, proxy_open, proxy_url
            );
            return Err(EType::ServerGeneral);
        }
    };
    let code = body_data_json["code"].as_i64().unwrap().clone();
    remove_parameters_playurl(&playurl_type, &mut body_data_json).unwrap_or_default();

    // update_area_cache(&body_data_json, params, bili_runtime).await;
    // report health
    let message = body_data_json["message"]
        .as_str()
        .unwrap_or("Error on parsing Json Response")
        .to_string();
    let upstream_reply = UpstreamReply {
        code,
        message,
        proxy_open,
        proxy_url: String::from(proxy_url),
    };
    if upstream_reply.is_available() {
        let final_data = body_data_json.to_string();
        update_cached_playurl(params, &final_data, bili_runtime).await;
        debug!(
            "[GET PLAYURL BACKGROUND][U] AREA {} | EP {} -> 获取成功",
            params.area.to_ascii_uppercase(),
            params.ep_id
        );
        Ok(final_data)
    } else {
        report_health(
            HealthReportType::Playurl(HealthData::init(
                Area::new(params.area_num),
                true,
                upstream_reply,
                params.ep_id,
            )),
            bili_runtime,
        )
        .await;
        Err(EType::OtherError(code, "上游错误, 刷新失败"))
    }
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
        None,
    )
    .await
    {
        Ok(data) => {
            let data_json: serde_json::Value = serde_json::from_str(&data).unwrap();
            let upstream_code = data_json["code"].as_i64().unwrap_or(233);
            let upstream_message = data_json["message"].as_str().unwrap_or("NULL");
            report_health(
                HealthReportType::Search(HealthData::init(
                    Area::new(params.area_num as u8),
                    true,
                    UpstreamReply {
                        code: upstream_code,
                        message: upstream_message.to_string(),
                        proxy_open,
                        proxy_url: String::from(proxy_url),
                    },
                    params.keyword,
                )),
                bili_runtime,
            )
            .await;
            if upstream_code == 0 {
                Ok(data_json)
            } else {
                error!(
                    "[GET SEARCH][U] AREA {} | PROXY_OPEN {} | PROXY_URL {} ->  Upstream ERROR {upstream_code}: {data_json}",
                    params.area.to_ascii_uppercase(), proxy_open, proxy_url
                );
                Err(EType::ServerReqError("上游错误"))
            }
        }
        Err(_) => {
            error!(
                "[GET SEARCH][U] AREA {} | PROXY_OPEN {} | PROXY_URL {} ->  Upstream ERROR: 网络问题",
                params.area.to_ascii_uppercase(),
                proxy_open,
                proxy_url
            );
            report_health(
                HealthReportType::Search(HealthData::init(
                    Area::new(params.area_num as u8),
                    false,
                    UpstreamReply {
                        proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                    params.keyword,
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
    // 硬编码APPSEC
    let appsec = params.appsec;
    let proxy_open = bili_runtime.config.th_proxy_subtitle_open;
    let proxy_url = &bili_runtime.config.th_proxy_subtitle_url;
    let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{unsigned_url}");
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{appsec}"))
    );
    let api = "https://app.biliintl.com/intl/gateway/v2/app/subtitle";
    match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        proxy_url,
        params.user_agent,
        "",
        None,
    )
    .await
    {
        Ok(value) => {
            update_th_subtitle_cache(&value, params, bili_runtime).await;
            Ok(value)
        }
        Err(value) => {
            // not intented to report_health
            error!(
                "[GET TH_SUBTITLE][U] AREA TH | PROXY_OPEN {} | PROXY_URL {} -> Upstream ERROR: 网络问题",
                proxy_open, proxy_url
            );
            Err(value)
        }
    }
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
        None,
    )
    .await
    {
        Ok(body_data) => {
            // println!("[Debug] ss_id:{}", season_id);
            // println!("[Debug] data:{}", data);
            let season_remake = move || async move {
                if config.th_app_season_sub_open || config.aid_replace_open {
                    let mut body_data_json: serde_json::Value =
                        serde_json::from_str(&body_data).unwrap();
                    let user_agent = params.user_agent;
                    if config.aid_replace_open {
                        let len_of_episodes = match body_data_json["result"]["modules"][0]["data"]
                            ["episodes"]
                            .as_array()
                        {
                            // 不这样做油猴脚本会有问题, 偶尔会panic
                            Some(value) => value.len(),
                            None => 0,
                        };
                        // {
                        //     Some(value) => value.len(),
                        //     None => {
                        //         println!("[Debug] error data: {}", body_data_json); //Debug
                        //         0
                        //     },
                        // };
                        let mut index = 0;
                        while index < len_of_episodes {
                            body_data_json["result"]["modules"][0]["data"]["episodes"][index]
                                .as_object_mut()
                                .unwrap()
                                .insert("aid".to_string(), serde_json::json!(&config.aid));
                            index += 1;
                        }
                    }

                    if config.th_app_season_sub_open {
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
                                if config.aid_replace_open {
                                    return serde_json::to_string(&body_data_json).unwrap();
                                } else {
                                    return body_data;
                                }
                            }
                            Some(_) => (),
                        }

                        let sub_replace_str = match async_getwebpage(
                            &format!("{}{}", &config.th_app_season_sub_api, season_id.unwrap()),
                            false,
                            "",
                            &user_agent,
                            "",
                            None,
                        )
                        .await
                        {
                            Ok(value) => value,
                            Err(_) => {
                                if config.aid_replace_open {
                                    return serde_json::to_string(&body_data_json).unwrap();
                                } else {
                                    return body_data;
                                }
                            }
                        };
                        let sub_replace_json: serde_json::Value =
                            if let Ok(value) = serde_json::from_str(&sub_replace_str) {
                                value
                            } else {
                                if config.aid_replace_open {
                                    return serde_json::to_string(&body_data_json).unwrap();
                                } else {
                                    return body_data;
                                }
                            };
                        match sub_replace_json["code"].as_i64().unwrap_or(233) {
                            0 => {
                                if body_data_json["result"]["modules"]
                                    .as_array_mut()
                                    .unwrap()
                                    .len()
                                    == 0
                                {
                                    if config.aid_replace_open {
                                        return serde_json::to_string(&body_data_json).unwrap();
                                    } else {
                                        return body_data;
                                    }
                                }
                            }
                            _ => {
                                if config.aid_replace_open {
                                    return serde_json::to_string(&body_data_json).unwrap();
                                } else {
                                    return body_data;
                                }
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
                    }

                    return serde_json::to_string(&body_data_json).unwrap();
                } else {
                    return body_data;
                }
            };
            let body_data = season_remake().await;
            update_th_season_cache(params.season_id, &body_data, &bili_runtime).await;
            report_health(
                HealthReportType::ThSeason(HealthData::init(
                    Area::Th,
                    true,
                    UpstreamReply {
                        code: 0,
                        proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                    params.season_id,
                )),
                bili_runtime,
            )
            .await;
            Ok(body_data)
        }
        Err(value) => {
            error!(
                "[GET TH_SEASON][U] AREA TH | PROXY_OPEN {} | PROXY_URL {} -> Upstream ERROR: 网络问题",
                proxy_open, proxy_url
            );
            report_health(
                HealthReportType::ThSeason(HealthData::init(
                    Area::Th,
                    false,
                    UpstreamReply {
                        proxy_open,
                        proxy_url: String::from(proxy_url),
                        ..Default::default()
                    },
                    params.season_id,
                )),
                bili_runtime,
            )
            .await;
            Err(value)
        }
    }
}

pub async fn get_upstream_bili_ep_info(
    ep_id: &str,
    proxy_open: bool,
    proxy_url: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<(EpInfo, Vec<EpInfo>), ()> {
    // 获取番剧信息
    // 1 season_id for later use
    // 2 ep need vip
    fn parse_data(value: String, ep_id: &str) -> Result<(EpInfo, Vec<EpInfo>), i64> {
        let value_json = serde_json::from_str(&value).unwrap_or(serde_json::json!({"code":-2333}));
        let mut ep_info_vec: Vec<EpInfo> = vec![];
        let mut current_ep_info: EpInfo = EpInfo {
            ..Default::default()
        };
        let upstream_code = value_json["code"].as_i64().unwrap_or(-2333);
        if upstream_code == 0 {
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
            for episode in episodes.as_array().unwrap() {
                let episode = episode.as_object().unwrap();
                let episode_ep_id = episode["ep_id"].as_u64().unwrap_or(0);
                let episode_need_vip = {
                    if episode.contains_key("badge") || episode.contains_key("badge_type") {
                        // DEBUG
                        debug!(
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
            return Err(upstream_code);
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
        None,
    )
    .await
    {
        Ok(value) => match parse_data(value, ep_id) {
            Ok(value) => Ok(value),
            Err(upstream_code_hidden) => {
                match async_getwebpage(&bili_season_api, proxy_open, proxy_url, user_agent, "", None)
                    .await
                {
                    Ok(value) => match parse_data(value, ep_id) {
                        Ok(value) => Ok(value),
                        Err(upstream_code) => {
                            if upstream_code_hidden == -404 && upstream_code == -404 {
                                // both -404 means zone th
                                Ok((
                                    EpInfo {
                                        need_vip: false,
                                        ..Default::default()
                                    },
                                    vec![],
                                ))
                            } else {
                                error!("[GET EP_INFO] EP {ep_id} -> 获取番剧信息失败: 上游问题 E {upstream_code_hidden}/{upstream_code}");
                                Err(())
                            }
                        }
                    },
                    Err(_) => {
                        report_health(
                            HealthReportType::Others(HealthData {
                                area_num: 0,
                                is_200_ok: false,
                                upstream_reply: UpstreamReply {
                                    proxy_open,
                                    proxy_url: proxy_url.to_string(),
                                    ..Default::default()
                                },
                                is_custom: true,
                                custom_message: String::from(
                                    "[GET EP_INFO] 获取番剧信息失败! 网络问题!",
                                ),
                            }),
                            bili_runtime,
                        )
                        .await;
                        Err(())
                    }
                }
            }
        },
        Err(_) => {
            // hidden_bili_season_api failed then try bili_season_api
            match async_getwebpage(&bili_season_api, proxy_open, proxy_url, user_agent, "", None).await {
                Ok(value) => match parse_data(value, ep_id) {
                    Ok(value) => Ok(value),
                    Err(upstream_code) => {
                        error!("[GET EP_INFO] EP {ep_id} -> 获取番剧信息失败: 上游问题 E -500/{upstream_code}");
                        Err(())
                    }
                },
                Err(_) => {
                    report_health(
                        HealthReportType::Others(HealthData {
                            area_num: 0,
                            is_200_ok: false,
                            upstream_reply: UpstreamReply {
                                proxy_open,
                                proxy_url: proxy_url.to_string(),
                                ..Default::default()
                            },
                            is_custom: true,
                            custom_message: String::from(
                                "[GET EP_INFO] 获取番剧信息失败! 网络问题!",
                            ),
                        }),
                        bili_runtime,
                    )
                    .await;
                    Err(())
                }
            }
        }
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
            // area_num: 4,
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
            // area_num: 1,
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
        .resign_from_api_open
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
        let webgetpage_data = if let Ok(data) = async_getwebpage(&url, false, "", "", "", None).await {
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
            // area_num: *area_num as i32,
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
