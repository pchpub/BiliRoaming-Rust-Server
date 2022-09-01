use super::get_user_info::{appkey_to_sec, auth_user, getuser_list};
use super::request::{async_getwebpage, redis_get, redis_set, async_postwebpage};
use super::tools::remove_parameters_playurl;
use super::types::{BiliConfig, PlayurlType, ResignInfo, SendData, SendPlayurlData, SendHealthData, SesourceType, HealthType};
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use async_channel::Sender;
use async_channel::TrySendError;
use chrono::prelude::*;
use deadpool_redis::Pool;
use md5;
use pcre2::bytes::Regex;
use qstring::QString;
use serde_json::{self, json};
use std::sync::Arc;
use std::thread::spawn;

pub async fn get_playurl(req: &HttpRequest, is_app: bool, is_th: bool) -> HttpResponse {
    let (pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<SendData>>)>()
        .unwrap();
    let bilisender_cl = Arc::clone(bilisender);
    match req.headers().get("user-agent") {
        Option::Some(_ua) => (),
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":-1403,\"message\":\"草,没ua你看个der\"}");
        }
    }
    let user_agent = format!(
        "{}",
        req.headers().get("user-agent").unwrap().to_str().unwrap()
    );
    if is_app && config.limit_biliroaming_version_open{
        match req.headers().get("build") {
            Some(value) => {
                let version: u16 = value.to_str().unwrap_or("0").parse().unwrap_or(0);
                if version < config.limit_biliroaming_version_min || version > config.limit_biliroaming_version_max {
                    return HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .body("{\"code\":-1403,\"message\":\"什么旧版本魔人,升下级\"}");
                }
            },
            None => (),
        }
    }
    
    let query_string = req.query_string();
    let query = QString::from(query_string);

    let mut appkey = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436",
    };

    let mut appsec = match appkey_to_sec(appkey) {
        Ok(value) => value,
        Err(()) => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":-3403,\"message\":\"未知设备\"}");
        }
    };

    if is_app || is_th {
        if query_string.len() <= 39
            || (format!(
                "{:x}",
                md5::compute(format!(
                    "{}{appsec}",
                    &query_string[..query_string.len() - 38]
                ))
            ) != &query_string[query_string.len() - 32..])
        {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":-0403,\"message\":\"校验失败\"}");
        }
    }

    let mut access_key = match query.get("access_key") {
        Option::Some(key) => key.to_string(),
        _ => {
            return HttpResponse::Ok().content_type(ContentType::json()).body(
                "{\"code\":-2403,\"message\":\"草,没登陆你看个der,让我凭空拿到你账号是吧\"}",
            );
        }
    };

    if access_key.len() == 0 {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("{\"code\":-2403,\"message\":\"没有accesskey,你b站和漫游需要换个版本\"}");
    }

    let area = match query.get("area") {
        Option::Some(area) => area,
        _ => {
            if is_th {
                "th"
            } else {
                "hk"
            }
        }
    };

    let area_num: u8 = match area {
        "cn" => 1,
        "hk" => 2,
        "tw" => 3,
        "th" => 4,
        _ => 2,
    };

    let ep_id = match query.get("ep_id") {
        Option::Some(key) => Some(key),
        _ => None,
    };

    let cid = match query.get("cid") {
        Option::Some(key) => Some(key),
        _ => None,
    };

    let user_info = match getuser_list(pool, &access_key, appkey, &appsec, &user_agent).await {
        Ok(value) => value,
        Err(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(format!("{{\"code\":-4403,\"message\":\"{value}\"}}"));
        }
    };

    let (black, white) = match auth_user(pool, &user_info.uid, &config).await {
        Ok(value) => value,
        Err(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(value);
        }
    };
    if black {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("{\"code\":-4403,\"message\":\"黑名单用户,建议换号重开\"}");
    }
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let mut is_vip = 0;
    if is_th {
        is_vip = 0;
        if *config.resign_open.get("4").unwrap_or(&false)
            && (white || *config.resign_pub.get("4").unwrap_or(&false))
        {
            (access_key, _) = get_resign_accesskey(pool, &4, &user_agent, &config)
                .await
                .unwrap_or((access_key, 1));
            is_vip = 1;
        }
    } else {
        if user_info.vip_expire_time >= ts {
            is_vip = 1;
        } else if *config.resign_open.get("4").unwrap_or(&false)
            && (white
                || *config
                    .resign_pub
                    .get(&area_num.to_string())
                    .unwrap_or(&false))
        {
            (access_key, _) = get_resign_accesskey(pool, &area_num, &user_agent, &config)
                .await
                .unwrap_or((access_key, 1));
            let user_info =
                match getuser_list(pool, &access_key, appkey, &appsec, &user_agent).await {
                    Ok(value) => value,
                    Err(value) => {
                        return HttpResponse::Ok()
                            .content_type(ContentType::json())
                            .body(format!("{{\"code\":-5403,\"message\":\"{value}\"}}"));
                    }
                };
            if user_info.vip_expire_time >= ts {
                is_vip = 1;
            }
        }
    }

    let is_tv: bool;
    match query.get("fnval") {
        Some(value) => match value {
            "130" => is_tv = true,
            "0" => is_tv = true,
            "2" => is_tv = true,
            _ => is_tv = false,
        },
        None => is_tv = false,
    }

    let key = match is_app {
        true => {
            if is_tv {
                format!(
                    "e{}c{}v{is_vip}t1{area_num}0101",
                    ep_id.unwrap_or(""),
                    cid.unwrap_or("")
                )
            } else {
                format!(
                    "e{}c{}v{is_vip}t0{area_num}0101",
                    ep_id.unwrap_or(""),
                    cid.unwrap_or("")
                )
            }
        }
        false => format!(
            "e{}c{}v{is_vip}t0{area_num}0701",
            ep_id.unwrap_or(""),
            cid.unwrap_or("")
        ),
    };

    //查询数据+地区（1位）+类型（2位）+版本（2位）
    //查询数据 a asscesskey
    //        e epid
    //        c cid
    //        v is_vip
    //        t is_tv
    //地区 cn 1
    //     hk 2
    //     tw 3
    //     th 4
    //     default 2
    //类型 app playurl 01
    //     app search 02
    //     app subtitle 03
    //     app season 04
    //     user_info 05
    //     user_cerinfo 06
    //     web playurl 07
    //     web search 08
    //     web subtitle 09
    //     web season 10
    //     resign_info 11
    //     api 12
    //     health 13 eg. 0141301 = playurl th health ver.1
    //版本 ：用于处理版本更新后导致的格式变更
    //     now 01
    let is_expire: bool;
    let need_flash: bool;
    let mut redis_get_data = String::new();
    match redis_get(&pool, &key).await {
        Some(value) => {
            let redis_get_data_expire_time = &value[..13].parse::<u64>().unwrap();
            if redis_get_data_expire_time - 1200000 > ts {
                need_flash = false;
                is_expire = false;
                redis_get_data = value[13..].to_string();
            } else if redis_get_data_expire_time < &ts {
                need_flash = true;
                is_expire = true;
            } else {
                need_flash = true;
                is_expire = false;
                redis_get_data = value[13..].to_string();
            }
        }
        None => {
            need_flash = true;
            is_expire = true;
        }
    };
    let response_body: String;
    if is_expire || need_flash {
        let ts_string = ts.to_string();
        let mut query_vec: Vec<(&str, &str)>;
        if is_tv {
            query_vec = vec![
                ("access_key", &access_key[..]),
                ("appkey", appkey),
                ("build", query.get("build").unwrap_or("6800300")),
                ("device", query.get("device").unwrap_or("android")),
                ("fnval", "130"),
                ("fnver", "0"),
                ("fourk", "1"),
                ("platform", "android"),
                //("qn", query.get("qn").unwrap_or("112")), //720P 64 1080P高码率 112
                ("qn", "112"),//测试了下,没会员会回落到下一档,所以没必要区分 DLNA投屏就最高一档好了,支持其他档没必要,还增加服务器负担
                ("ts", &ts_string),
            ];
        } else {
            query_vec = vec![
                ("access_key", &access_key[..]),
                ("appkey", appkey),
                ("build", query.get("build").unwrap_or("6800300")),
                ("device", query.get("device").unwrap_or("android")),
                ("fnval", "4048"),
                ("fnver", "0"),
                ("fourk", "1"),
                ("platform", "android"),
                ("qn", "125"),
                ("ts", &ts_string),
            ];
        }

        match ep_id {
            Some(value) => query_vec.push(("ep_id", value)),
            None => (),
        }
        match cid {
            Some(value) => query_vec.push(("cid", value)),
            None => (),
        }
        match area_num {
            4 => {
                appkey = "7d089525d3611b1c";
                appsec = appkey_to_sec(&appkey).unwrap();
                query_vec.push(("s_locale", "zh_SG"));
            }
            _ => (),
        }
        query_vec.sort_by_key(|v| v.0);
        let unsigned_url = qstring::QString::new(query_vec);
        let unsigned_url = format!("{unsigned_url}");
        let signed_url = format!(
            "{unsigned_url}&sign={:x}",
            md5::compute(format!("{unsigned_url}{appsec}"))
        );
        let proxy_open = match area_num {
            1 => &config.cn_proxy_playurl_open,
            2 => &config.hk_proxy_playurl_open,
            3 => &config.tw_proxy_playurl_open,
            4 => &config.th_proxy_playurl_open,
            _ => &config.tw_proxy_playurl_open,
        };
        let proxy_url = match area_num {
            1 => &config.cn_proxy_playurl_url,
            2 => &config.hk_proxy_playurl_url,
            3 => &config.tw_proxy_playurl_url,
            4 => &config.th_proxy_playurl_url,
            _ => &config.tw_proxy_playurl_url,
        };
        let api = match is_app {
            true => match area_num {
                1 => &config.cn_app_playurl_api,
                2 => &config.hk_app_playurl_api,
                3 => &config.tw_app_playurl_api,
                4 => &config.th_app_playurl_api,
                _ => &config.tw_app_playurl_api,
            },
            false => match area_num {
                1 => &config.cn_web_playurl_api,
                2 => &config.hk_web_playurl_api,
                3 => &config.tw_web_playurl_api,
                4 => &config.th_web_playurl_api,
                _ => &config.tw_web_playurl_api,
            },
        };
        if is_expire {
            let mut body_data = match async_getwebpage(
                &format!("{api}?{signed_url}"),
                proxy_open,
                proxy_url,
                &user_agent,
            )
            .await
            {
                Ok(data) => data,
                Err(_) => {
                    if config.telegram_report && redis_get(&pool, &format!("01{}1301",area_num)).await.unwrap_or("0".to_string()).as_str() == "0" {
                        redis_set(&pool, &format!("01{}1301",area_num), "1", 0).await.unwrap_or_default();
                        let senddata = SendData::Health(SendHealthData{
                            area_num,
                            data_type: SesourceType::PlayUrl,
                            health_type: HealthType::Offline,
                        });
                        spawn(move || {
                            //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                            match bilisender_cl.try_send(senddata) {
                                Ok(_) => (),
                                Err(TrySendError::Full(_)) => {
                                    println!("[Error] channel is full");
                                }
                                Err(TrySendError::Closed(_)) => {
                                    println!("[Error] channel is closed");
                                }
                            };
                        });
                    }
                    return HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .body("{\"code\":-6404,\"message\":\"获取播放地址失败喵\"}");
                }
            };
            let mut body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
            if area_num == 4 {
                remove_parameters_playurl(PlayurlType::Thailand, &mut body_data_json)
                    .unwrap_or_default();
            } else {
                remove_parameters_playurl(PlayurlType::China, &mut body_data_json)
                    .unwrap_or_default();
            }
            let mut code = body_data_json["code"].as_i64().unwrap().clone();
            let backup_policy = match area_num {
                1 => &config.cn_proxy_playurl_backup_policy,
                2 => &config.hk_proxy_playurl_backup_policy,
                3 => &config.tw_proxy_playurl_backup_policy,
                4 => &config.th_proxy_playurl_backup_policy,
                _ => &false,
            };
            if code == -10500 as i64 && *backup_policy {
                let api = match is_app {
                    true => match area_num {
                        1 => &config.cn_app_playurl_backup_api,
                        2 => &config.hk_app_playurl_backup_api,
                        3 => &config.tw_app_playurl_backup_api,
                        4 => &config.th_app_playurl_backup_api,
                        _ => &config.tw_app_playurl_backup_api,
                    },
                    false => match area_num {
                        1 => &config.cn_web_playurl_backup_api,
                        2 => &config.hk_web_playurl_backup_api,
                        3 => &config.tw_web_playurl_backup_api,
                        4 => &config.th_web_playurl_backup_api,
                        _ => &config.tw_web_playurl_backup_api,
                    },
                };
                let proxy_open = match area_num {
                    1 => &config.cn_proxy_playurl_backup_open,
                    2 => &config.hk_proxy_playurl_backup_open,
                    3 => &config.tw_proxy_playurl_backup_open,
                    4 => &config.th_proxy_playurl_backup_open,
                    _ => &config.tw_proxy_playurl_backup_open,
                };
                let proxy_url = match area_num {
                    1 => &config.cn_proxy_playurl_backup_url,
                    2 => &config.hk_proxy_playurl_backup_url,
                    3 => &config.tw_proxy_playurl_backup_url,
                    4 => &config.th_proxy_playurl_backup_url,
                    _ => &config.tw_proxy_playurl_backup_url,
                };
                body_data = match async_getwebpage(
                    &format!("{api}?{signed_url}"),
                    proxy_open,
                    proxy_url,
                    &user_agent,
                )
                .await
                {
                    Ok(data) => data,
                    Err(_) => {
                        if config.telegram_report && redis_get(&pool, &format!("01{}1301",area_num)).await.unwrap_or("0".to_string()).as_str() == "0" {
                            redis_set(&pool, &format!("01{}1301",area_num), "1", 0).await.unwrap_or_default();
                            let senddata = SendData::Health(SendHealthData{
                                area_num,
                                data_type: SesourceType::PlayUrl,
                                health_type: HealthType::Offline,
                            });
                            spawn(move || {
                                //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                                match bilisender_cl.try_send(senddata) {
                                    Ok(_) => (),
                                    Err(TrySendError::Full(_)) => {
                                        println!("[Error] channel is full");
                                    }
                                    Err(TrySendError::Closed(_)) => {
                                        println!("[Error] channel is closed");
                                    }
                                };
                            });
                        }
                        return HttpResponse::Ok()
                            .content_type(ContentType::json())
                            .body("{\"code\":-7404,\"message\":\"获取播放地址失败喵\"}");
                    }
                };
                let body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
                code = body_data_json["code"].as_i64().unwrap();
            }

            let expire_time = match config.cache.get(&code.to_string()) {
                Some(value) => value,
                None => config.cache.get("other").unwrap(),
            };
            let value = format!("{}{body_data}", ts + expire_time * 1000);
            let _: () = redis_set(&pool, &key, &value, *expire_time)
                .await
                .unwrap_or_default();
            if config.telegram_report {
                match redis_get(&pool, &format!("01{}1301",area_num)).await {
                    Some(value) => {
                        if &value == "1" {
                            redis_set(&pool, &format!("01{}1301",area_num), "0", 0).await.unwrap_or_default();
                            let senddata = SendData::Health(SendHealthData{
                                area_num,
                                data_type: SesourceType::PlayUrl,
                                health_type: HealthType::Online,
                            });
                            spawn(move || {
                                //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                                match bilisender_cl.try_send(senddata) {
                                    Ok(_) => (),
                                    Err(TrySendError::Full(_)) => {
                                        println!("[Error] channel is full");
                                    }
                                    Err(TrySendError::Closed(_)) => {
                                        println!("[Error] channel is closed");
                                    }
                                };
                            });
                        }
                    },
                    None => {
                        redis_set(&pool, &format!("01{}1301",area_num), "0", 0).await.unwrap_or_default();
                        let senddata = SendData::Health(SendHealthData{
                            area_num,
                            data_type: SesourceType::PlayUrl,
                            health_type: HealthType::Online,
                        });
                        spawn(move || {
                            //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                            match bilisender_cl.try_send(senddata) {
                                Ok(_) => (),
                                Err(TrySendError::Full(_)) => {
                                    println!("[Error] channel is full");
                                }
                                Err(TrySendError::Closed(_)) => {
                                    println!("[Error] channel is closed");
                                }
                            };
                        });
                    },
                }
            }
            response_body = body_data;
        } else {
            let senddata = SendData::Playurl(SendPlayurlData {
                key,
                url: format!("{api}?{signed_url}"),
                proxy_open: proxy_open.clone(),
                proxy_url: proxy_url.to_string(),
                user_agent,
                area_num,
            });
            spawn(move || {
                //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                match bilisender_cl.try_send(senddata) {
                    Ok(_) => (),
                    Err(TrySendError::Full(_)) => {
                        println!("[Error] channel is full");
                    }
                    Err(TrySendError::Closed(_)) => {
                        println!("[Error] channel is closed");
                    }
                };
            });
            response_body = redis_get_data;
        }
    } else {
        response_body = redis_get_data;
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("From", "biliroaming-rust-server"))
        .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
        .insert_header(("Access-Control-Allow-Credentials", "true"))
        .insert_header(("Access-Control-Allow-Methods", "GET"))
        .body(response_body)
}

pub async fn get_playurl_background(
    redis: &Pool,
    receive_data: &SendPlayurlData,
    anti_speedtest_cfg: &BiliConfig,
) -> Result<(), String> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let body_data = match async_getwebpage(
        &receive_data.url,
        &receive_data.proxy_open,
        &receive_data.proxy_url,
        &receive_data.user_agent,
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
    if receive_data.area_num == 4 {
        remove_parameters_playurl(PlayurlType::Thailand, &mut body_data_json).unwrap_or_default();
    } else {
        remove_parameters_playurl(PlayurlType::China, &mut body_data_json).unwrap_or_default();
    }
    let expire_time = match anti_speedtest_cfg
        .cache
        .get(&body_data_json["code"].as_i64().unwrap_or_default().to_string())
    {
        Some(value) => value,
        None => anti_speedtest_cfg.cache.get("other").unwrap_or(&1380),
    };
    let value = format!("{}{body_data}", ts + expire_time * 1000);
    match redis_set(&redis, &receive_data.key, &value, *expire_time).await {
        Some(_) => return Ok(()),
        None => return Err("[Error] fn get_playurl_background redis set error".to_string()),
    }
}

pub async fn get_search(req: &HttpRequest, is_app: bool, is_th: bool) -> HttpResponse {
    let (pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<SendData>>)>()
        .unwrap();
    let bilisender_cl = Arc::clone(bilisender);
    match req.headers().get("user-agent") {
        Option::Some(_ua) => (),
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":-1403,\"message\":\"草,没ua你看个der\"}");
        }
    }

    let user_agent = format!(
        "{}",
        req.headers().get("user-agent").unwrap().to_str().unwrap()
    );
    let query = QString::from(req.query_string());

    let access_key: &str;
    if is_app && (!is_th) {
        access_key = match query.get("access_key") {
            Option::Some(key) => key,
            _ => {
                return HttpResponse::Ok().content_type(ContentType::json()).body(
                    "{\"code\":-2403,\"message\":\"草,没登陆你搜个der,让我凭空拿到你账号是吧\"}",
                );
            }
        };
    } else {
        access_key = "";
    }

    let mut appkey = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436", //为了应对新的appkey,应该设定默认值
    };

    let keyword = match query.get("keyword") {
        Option::Some(key) => key,
        _ => "",
    };

    let area = match query.get("area") {
        Option::Some(area) => area,
        _ => {
            if is_th {
                "th"
            } else {
                "hk"
            }
        }
    };

    let area_num = match area {
        "cn" => 1,
        "hk" => 2,
        "tw" => 3,
        "th" => {
            appkey = "7d089525d3611b1c";
            4
        }
        _ => 2,
    };

    let appsec = match appkey_to_sec(appkey) {
        Ok(value) => value,
        Err(()) => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":-3403,\"message\":\"未知设备\"}");
        }
    };

    if is_app && (!is_th) {
        let user_info = match getuser_list(pool, access_key, appkey, &appsec, &user_agent).await {
            Ok(value) => value,
            Err(value) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(format!("{{\"code\":-4403,\"message\":\"{value}\"}}"));
            }
        };

        let (_, _) = match auth_user(pool, &user_info.uid, &config).await {
            Ok(value) => value,
            Err(_) => (false, false),
        };
    }

    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(String, String)>;
    if is_th {
        query_vec = vec![
            // ("access_key".to_string(), access_key.to_string()),
            ("appkey".to_string(), appkey.to_string()),
            (
                "build".to_string(),
                query.get("build").unwrap_or("1080003").to_string(),
            ),
            ("c_locale".to_string(), "zh_SG".to_string()),
            ("channel".to_string(), "master".to_string()),
            (
                "device".to_string(),
                query.get("device").unwrap_or("android").to_string(),
            ),
            ("disable_rcmd".to_string(), "0".to_string()),
            (
                "fnval".to_string(),
                query.get("fnval").unwrap_or("976").to_string(),
            ),
            ("fnver".to_string(), "0".to_string()),
            ("fourk".to_string(), "1".to_string()),
            ("highlight".to_string(), "1".to_string()),
            ("keyword".to_string(), keyword.to_string()),
            ("lang".to_string(), "hans".to_string()),
            ("mobi_app".to_string(), "bstar_a".to_string()),
            ("platform".to_string(), "android".to_string()),
            ("pn".to_string(), query.get("pn").unwrap_or("1").to_string()),
            ("ps".to_string(), "20".to_string()),
            ("qn".to_string(), "120".to_string()),
            ("s_locale".to_string(), "zh_SG".to_string()),
            ("sim_code".to_string(), "52004".to_string()),
            ("ts".to_string(), ts_string.to_string()),
            ("type".to_string(), "7".to_string()),
        ];
        match query.get("access_key") {
            Option::Some(value) => {
                query_vec.push(("access_key".to_string(), value.to_string()));
            }
            _ => (),
        };
        match query.get("statistics") {
            Some(value) => {
                query_vec.push(("statistics".to_string(), value.to_string()));
            }
            _ => (),
        }
    } else {
        if is_app {
            query_vec = vec![
                ("access_key".to_string(), access_key.to_string()),
                ("appkey".to_string(), appkey.to_string()),
                (
                    "build".to_string(),
                    query.get("build").unwrap_or("6400000").to_string(),
                ),
                ("c_locale".to_string(), "zh_CN".to_string()),
                ("channel".to_string(), "master".to_string()),
                (
                    "device".to_string(),
                    query.get("device").unwrap_or("android").to_string(),
                ),
                ("disable_rcmd".to_string(), "0".to_string()),
                ("fnval".to_string(), "4048".to_string()),
                ("fnver".to_string(), "0".to_string()),
                ("fourk".to_string(), "1".to_string()),
                ("highlight".to_string(), "1".to_string()),
                ("keyword".to_string(), keyword.to_string()),
                ("mobi_app".to_string(), "android".to_string()),
                ("platform".to_string(), "android".to_string()),
                ("pn".to_string(), query.get("pn").unwrap_or("1").to_string()),
                ("ps".to_string(), "20".to_string()),
                ("qn".to_string(), "120".to_string()),
                ("s_locale".to_string(), "zh_CN".to_string()),
                ("ts".to_string(), ts_string.to_string()),
                ("type".to_string(), "7".to_string()),
            ];
            match query.get("statistics") {
                Some(value) => {
                    query_vec.push(("statistics".to_string(), value.to_string()));
                }
                _ => (),
            }
        } else {
            query_vec = query.clone().into_pairs();
        }
    }

    query_vec.sort_by_key(|v| v.0.clone());
    //let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{}", qstring::QString::new(query_vec));
    let signed_url = format!(
        "{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{appsec}"))
    );
    let api = match (area_num, is_app) {
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

    let proxy_open = match area_num {
        1 => &config.cn_proxy_search_open,
        2 => &config.hk_proxy_search_open,
        3 => &config.tw_proxy_search_open,
        4 => &config.th_proxy_search_open,
        _ => &config.hk_proxy_search_open,
    };

    let proxy_url = match area_num {
        1 => &config.cn_proxy_search_url,
        2 => &config.hk_proxy_search_url,
        3 => &config.tw_proxy_search_url,
        4 => &config.th_proxy_search_url,
        _ => &config.hk_proxy_search_url,
    };

    let body_data = match async_getwebpage(
        &format!("{api}?{signed_url}"),
        proxy_open,
        &proxy_url,
        &user_agent,
    )
    .await
    {
        Ok(data) => data,
        Err(_) => {
            if config.telegram_report && redis_get(&pool, &format!("02{}1301",area_num)).await.unwrap_or("0".to_string()).as_str() == "0" {
                redis_set(&pool, &format!("02{}1301",area_num), "1", 0).await.unwrap_or_default();
                let senddata = SendData::Health(SendHealthData{
                    area_num,
                    data_type: SesourceType::Search,
                    health_type: HealthType::Offline,
                });
                spawn(move || {
                    //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                    match bilisender_cl.try_send(senddata) {
                        Ok(_) => (),
                        Err(TrySendError::Full(_)) => {
                            println!("[Error] channel is full");
                        }
                        Err(TrySendError::Closed(_)) => {
                            println!("[Error] channel is closed");
                        }
                    };
                });
            }
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":-5404,\"message\":\"获取失败喵\"}");
        }
    };

    // if !is_app {
    //     return HttpResponse::Ok()
    //         .content_type(ContentType::json())
    //         .insert_header(("From", "biliroaming-rust-server"))
    //         .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
    //         .insert_header(("Access-Control-Allow-Credentials", "true"))
    //         .insert_header(("Access-Control-Allow-Methods", "GET"))
    //         .body(body_data);
    // }

    let host = match req.headers().get("Host") {
        Some(host) => host.to_str().unwrap(),
        _ => match req.headers().get("authority") {
            Some(host) => host.to_str().unwrap(),
            _ => "",
        },
    };
    if query.get("pn").unwrap_or("1") != "1" {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body(body_data);
    }
    let search_remake_date = {
        if is_app {
            if let Some(value) = config.appsearch_remake.get(host) {
                value
            } else {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials", "true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(body_data);
            }
        } else {
            if let Some(value) = config.websearch_remake.get(host) {
                value
            } else {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials", "true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(body_data);
            }
        }
    };
    let mut body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
    if body_data_json["code"].as_i64().unwrap_or(233) != 0
        && body_data_json["code"].as_str().unwrap_or("233") != "0"
    {
        if config.telegram_report && redis_get(&pool, &format!("02{}1301",area_num)).await.unwrap_or("0".to_string()).as_str() == "0" {
            redis_set(&pool, &format!("02{}1301",area_num), "1", 0).await.unwrap_or_default();
            let senddata = SendData::Health(SendHealthData{
                area_num,
                data_type: SesourceType::Search,
                health_type: HealthType::Offline,
            });
            spawn(move || {
                //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                match bilisender_cl.try_send(senddata) {
                    Ok(_) => (),
                    Err(TrySendError::Full(_)) => {
                        println!("[Error] channel is full");
                    }
                    Err(TrySendError::Closed(_)) => {
                        println!("[Error] channel is closed");
                    }
                };
            });
        }
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("{\"code\":-6404,\"message\":\"获取失败喵\"}");
    }
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
    if config.telegram_report {
        match redis_get(&pool, &format!("02{}1301",area_num)).await {
            Some(value) => {
                if &value == "1" {
                    redis_set(&pool, &format!("02{}1301",area_num), "0", 0).await.unwrap_or_default();
                    let senddata = SendData::Health(SendHealthData{
                        area_num,
                        data_type: SesourceType::Search,
                        health_type: HealthType::Online,
                    });
                    spawn(move || {
                        //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                        match bilisender_cl.try_send(senddata) {
                            Ok(_) => (),
                            Err(TrySendError::Full(_)) => {
                                println!("[Error] channel is full");
                            }
                            Err(TrySendError::Closed(_)) => {
                                println!("[Error] channel is closed");
                            }
                        };
                    });
                }
            },
            None => {
                redis_set(&pool, &format!("02{}1301",area_num), "0", 0).await.unwrap_or_default();
                let senddata = SendData::Health(SendHealthData{
                    area_num,
                    data_type: SesourceType::Search,
                    health_type: HealthType::Online,
                });
                spawn(move || {
                    //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                    match bilisender_cl.try_send(senddata) {
                        Ok(_) => (),
                        Err(TrySendError::Full(_)) => {
                            println!("[Error] channel is full");
                        }
                        Err(TrySendError::Closed(_)) => {
                            println!("[Error] channel is closed");
                        }
                    };
                });
            },
        }
    }
    let body_data = body_data_json.to_string();
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("From", "biliroaming-rust-server"))
        .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
        .insert_header(("Access-Control-Allow-Credentials", "true"))
        .insert_header(("Access-Control-Allow-Methods", "GET"))
        .body(body_data);
}

pub async fn get_season(req: &HttpRequest, _is_app: bool, _is_th: bool) -> HttpResponse {
    let (pool, config, bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<SendData>>)>()
        .unwrap();
    let bilisender_cl = Arc::clone(bilisender);
    match req.headers().get("user-agent") {
        Option::Some(_ua) => (),
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":-1403,\"message\":\"草,没ua你看个der\"}");
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
            return HttpResponse::Ok().content_type(ContentType::json()).body(
                "{\"code\":-2403,\"message\":\"草,没登陆你搜个der,让我凭空拿到你账号是吧\"}",
            );
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

    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();

    let season_id = query.get("season_id").unwrap_or("114514");
    let key = format!("s{}41001", season_id);
    let is_expire: bool;
    let redis_get_data: String;
    match redis_get(&pool, &key).await {
        Some(value) => {
            let redis_get_data_expire_time = &value[..13].parse::<u64>().unwrap();
            if redis_get_data_expire_time > &ts {
                is_expire = false;
                redis_get_data = value[13..].to_string();
            } else {
                is_expire = true;
                redis_get_data = "".to_string();
            }
        }
        None => {
            is_expire = true;
            redis_get_data = "".to_string();
        }
    };

    if is_expire {
        let mut query_vec = vec![
            ("access_key", access_key),
            ("appkey", "7d089525d3611b1c"),
            ("build", query.get("build").unwrap_or("1080003")),
            ("mobi_app", "bstar_a"),
            ("season_id", season_id),
            ("s_locale", "zh_SG"),
            ("ts", &ts_string),
        ];

        query_vec.sort_by_key(|v| v.0);
        //let unsigned_url = qstring::QString::new(query_vec);
        let unsigned_url = format!("{}", qstring::QString::new(query_vec));
        let appsec = match appkey_to_sec("7d089525d3611b1c") {
            Ok(value) => value,
            _ => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(format!(
                        "{{\"code\":-3403,\"message\":\"没有对应的appsec\"}}"
                    ));
            }
        };
        let signed_url = format!(
            "{unsigned_url}&sign={:x}",
            md5::compute(format!("{unsigned_url}{appsec}"))
        );
        let proxy_open = &config.th_proxy_playurl_open;
        let proxy_url = &config.th_proxy_playurl_url;
        let api = &config.th_app_season_api;
        let body_data = match async_getwebpage(
            &format!("{api}?{signed_url}"),
            proxy_open,
            &proxy_url,
            &user_agent,
        )
        .await
        {
            Ok(data) => data,
            Err(_) => {
                if config.telegram_report && redis_get(&pool, "0441301").await.unwrap_or("0".to_string()).as_str() == "0" {
                    redis_set(&pool, "0441301", "1", 0).await.unwrap_or_default();
                    let senddata = SendData::Health(SendHealthData{
                        area_num: 4,
                        data_type: SesourceType::Season,
                        health_type: HealthType::Offline,
                    });
                    spawn(move || {
                        //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                        match bilisender_cl.try_send(senddata) {
                            Ok(_) => (),
                            Err(TrySendError::Full(_)) => {
                                println!("[Error] channel is full");
                            }
                            Err(TrySendError::Closed(_)) => {
                                println!("[Error] channel is closed");
                            }
                        };
                    });
                }
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body("{\"code\":-4404,\"message\":\"获取失败喵\"}");
            }
        };
        let season_remake = move || async move {
            if config.th_app_season_sub_open {
                let mut body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
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
                )
                .await
                {
                    Ok(value) => value,
                    Err(_) => {
                        return body_data;
                    }
                };
                let sub_replace_json: serde_json::Value =
                    serde_json::from_str(&sub_replace_str).unwrap();
                match sub_replace_json["code"].as_i64().unwrap() {
                    0 => (),
                    _ => {
                        return body_data;
                    }
                }
                let mut index_of_replace_json = 0;
                let len_of_replace_json = sub_replace_json["data"].as_array().unwrap().len();
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
                        body_data_json["result"]["modules"][0]["data"]["episodes"][ep]["subtitles"]
                            .as_array_mut()
                            .unwrap()
                            .insert(0, serde_json::from_str(&element).unwrap());
                    }
                    index_of_replace_json += 1;
                }
    
                if config.aid_replace_open {
                    let len_of_episodes = body_data_json["result"]["modules"][0]["data"]["episodes"]
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
        let expire_time = match config.cache.get(&"season".to_string()) {
            Some(value) => value,
            None => &1800,
        };
        let value = format!("{}{body_data}", ts + expire_time * 1000);
        redis_set(&pool, &key, &value, *expire_time).await;
        if config.telegram_report {
            match redis_get(&pool, "0441301").await {
                Some(value) => {
                    if &value == "1" {
                        redis_set(&pool, "0441301", "0", 0).await.unwrap_or_default();
                        let senddata = SendData::Health(SendHealthData{
                            area_num: 4,
                            data_type: SesourceType::PlayUrl,
                            health_type: HealthType::Online,
                        });
                        spawn(move || {
                            //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                            match bilisender_cl.try_send(senddata) {
                                Ok(_) => (),
                                Err(TrySendError::Full(_)) => {
                                    println!("[Error] channel is full");
                                }
                                Err(TrySendError::Closed(_)) => {
                                    println!("[Error] channel is closed");
                                }
                            };
                        });
                    }
                },
                None => {
                    redis_set(&pool, "0441301", "0", 0).await.unwrap_or_default();
                    let senddata = SendData::Health(SendHealthData{
                        area_num: 4,
                        data_type: SesourceType::PlayUrl,
                        health_type: HealthType::Online,
                    });
                    spawn(move || {
                        //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                        match bilisender_cl.try_send(senddata) {
                            Ok(_) => (),
                            Err(TrySendError::Full(_)) => {
                                println!("[Error] channel is full");
                            }
                            Err(TrySendError::Closed(_)) => {
                                println!("[Error] channel is closed");
                            }
                        };
                    });
                },
            }
        }
        return  HttpResponse::Ok()
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
                let resign_info_json: ResignInfo = serde_json::from_str(&value).unwrap();
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
        let webgetpage_data = if let Ok(data) = async_getwebpage(&url, &false, "", "").await {
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
        let resign_info = ResignInfo {
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
        let resign_info_json: ResignInfo = serde_json::from_str(&resign_info_str).unwrap();
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
    let getpost_string = match async_postwebpage(&url, &content, proxy_open, proxy_url, user_agent).await{
        Ok(value) => value,
        Err(_) => return None,
    };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    let resign_info = ResignInfo {
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
    let getpost_string = match async_postwebpage(&url, &content, proxy_open, proxy_url, user_agent).await{
        Ok(value) => value,
        Err(_) => return None,
    };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    let resign_info = ResignInfo {
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

async fn to_resign_info(resin_info_str: &str) -> ResignInfo {
    serde_json::from_str(resin_info_str).unwrap()
}

pub async fn get_subtitle_th(req: &HttpRequest, _: bool, _: bool) -> HttpResponse {
    let (pool, config, _bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<SendData>>)>()
        .unwrap();
    match req.headers().get("user-agent") {
        Option::Some(_ua) => (),
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("{\"code\":-1403,\"message\":\"草,没ua你看个der\"}");
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
    //查询数据+地区（1位）+类型（2位）+版本（2位）
    //地区 cn 1
    //     hk 2
    //     tw 3
    //     th 4 （不打算支持，切割泰区，没弹幕我为什么不看nc-raw?）
    //     default 2
    //类型 app playurl 01
    //     app search 02
    //     app subtitle 03
    //     app season 04 (留着备用)
    //     user_info 05
    //     user_cerinfo 06
    //     web playurl 07
    //     web search 08
    //     web subtitle 09
    //     web season 10
    //     token 11
    //     th subtitle 12
    //版本 ：用于处理版本更新后导致的格式变更
    //     now 01
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
        let appsec = appkey_to_sec("7d089525d3611b1c").unwrap();
        let proxy_open = &config.th_proxy_subtitle_open;
        let proxy_url = &config.th_proxy_subtitle_url;
        let unsigned_url = qstring::QString::new(query_vec);
        let unsigned_url = format!("{unsigned_url}");
        let signed_url = format!(
            "{unsigned_url}&sign={:x}",
            md5::compute(format!("{unsigned_url}{appsec}"))
        );
        let api = "https://app.biliintl.com/intl/gateway/v2/app/subtitle";
        let body_data = match async_getwebpage(
            &format!("{api}?{signed_url}"),
            proxy_open,
            proxy_url,
            &user_agent,
        )
        .await
        {
            Ok(data) => data,
            Err(_) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body("{\"code\":-2404,\"message\":\"获取字幕失败喵\"}");
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
