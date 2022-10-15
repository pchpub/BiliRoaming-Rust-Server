use super::request::async_getwebpage;
use super::tools::remove_parameters_playurl;
use super::types::{
    BiliConfig, BiliPlayurlParams, OnlineBlackListConfig, PlayurlType,
    UserCerinfo, UserInfo, SendPlayurlData
};
use chrono::prelude::*;
use deadpool_redis::Pool;
use md5;
use std::string::String;

pub async fn get_upstream_bili_account_info(
    access_key: &str,
    app_key: &str,
    app_sec: &str,
    user_agent: &str,
    config: &BiliConfig,
    redis_pool: &Pool,
) -> Result<UserInfo, String> {
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
        Err(_) => {
            // println!("getuser_list函数寄了 url:{}",url);
            return Err("emmmm解析服务器的网络问题".to_string());
        }
    };

    //println!("{}",output);
    let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();
    let output_struct: UserInfo;
    let code = if let Some(value) = output_json["code"].as_i64() {
        value
    } else {
        println!("{}", output);
        return Err("服务器内部错误".to_string());
    };
    if code == 0 {
        output_struct = UserInfo {
            access_key: String::from(access_key),
            uid: output_json["data"]["mid"].as_u64().unwrap(),
            vip_expire_time: output_json["data"]["vip"]["due_date"].as_u64().unwrap(),
            expire_time: ts + 25 * 24 * 60 * 60 * 1000, //用户状态25天强制更新
        };
    } else if code == -400 {
        //println!("getuser_list函数寄了 output_json:{}",output_json);
        return Err("可能你用的不是手机".to_string());
    } else if code == -101 {
        //println!("getuser_list函数寄了 output_json:{}",output_json);
        return Err("账号未登录喵(b站api说的,估计你access_key过期了)".to_string());
    } else if code == -3 {
        //println!("{}",url); //Debug
        //println!("getuser_list函数寄了 output_json:{}",output_json);
        return Err("可能我sign参数算错了,非常抱歉喵".to_string());
    } else if code == -412 {
        //println!("getuser_list函数寄了 output_json:{}",output_json);
        return Err("被草到风控了.....".to_string());
    } else {
        //println!("getuser_list函数寄了 output_json:{}",output_json);
        return Err(format!("鼠鼠说:{}", output_json["code"].as_i64().unwrap()));
    }

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
    //     resign_info 11
    //     api 12
    //版本 ：用于处理版本更新后导致的格式变更
    //     now 01
    return Ok(output_struct);
}

pub async fn get_upstream_blacklist_info(
    config: &OnlineBlackListConfig,
    uid: &u64,
) -> Option<UserCerinfo> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    //let user_cerinfo_str = String::new();
    let getwebpage_data =
        match async_getwebpage(&format!("{}{uid}", config.api), &false, "", "", "").await {
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
        };
        //println!("[Debug] uid:{}", return_data.uid);
        return Some(return_data);
    } else {
        return None;
    }
}

pub async fn get_upstream_bili_playurl(
    // query: QString,
    params: &BiliPlayurlParams<'_>,
    config: &BiliConfig,
) -> Result<String, String> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(&str, &str)>;
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
    let mut code = body_data_json["code"].as_i64().unwrap().clone();
    if params.area_num == 4 {
        remove_parameters_playurl(PlayurlType::Thailand, &mut body_data_json).unwrap_or_default();
    } else {
        if params.is_app {
            remove_parameters_playurl(PlayurlType::ChinaApp, &mut body_data_json)
                .unwrap_or_default();
        } else {
            remove_parameters_playurl(PlayurlType::ChinaWeb, &mut body_data_json)
                .unwrap_or_default();
        }
    }
    let backup_policy = match params.area_num {
        1 => &config.cn_proxy_playurl_backup_policy,
        2 => &config.hk_proxy_playurl_backup_policy,
        3 => &config.tw_proxy_playurl_backup_policy,
        4 => &config.th_proxy_playurl_backup_policy,
        _ => &false,
    };
    if code == -10500 as i64 && *backup_policy {
        let api = match params.is_app {
            true => match params.area_num {
                1 => &config.cn_app_playurl_backup_api,
                2 => &config.hk_app_playurl_backup_api,
                3 => &config.tw_app_playurl_backup_api,
                4 => &config.th_app_playurl_backup_api,
                _ => &config.tw_app_playurl_backup_api,
            },
            false => match params.area_num {
                1 => &config.cn_web_playurl_backup_api,
                2 => &config.hk_web_playurl_backup_api,
                3 => &config.tw_web_playurl_backup_api,
                4 => &config.th_web_playurl_backup_api,
                _ => &config.tw_web_playurl_backup_api,
            },
        };
        let proxy_open = match params.area_num {
            1 => &config.cn_proxy_playurl_backup_open,
            2 => &config.hk_proxy_playurl_backup_open,
            3 => &config.tw_proxy_playurl_backup_open,
            4 => &config.th_proxy_playurl_backup_open,
            _ => &config.tw_proxy_playurl_backup_open,
        };
        let proxy_url = match params.area_num {
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
            params.user_agent,
            "",
        )
        .await
        {
            Ok(data) => data,
            Err(_) => return Err("{\"code\":-404,\"message\":\"获取播放地址失败喵\"}".to_string()),
        };
        body_data_json = serde_json::from_str(&body_data).unwrap();
        code = body_data_json["code"].as_i64().unwrap();
    }
    Ok(body_data_json.to_string())
}

pub async fn get_upstream_bili_playurl_background(
    params: &SendPlayurlData,
    config: &BiliConfig,
) -> Result<String, String> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(&str, &str)>;
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
    if params.area_num == 4 {
        remove_parameters_playurl(PlayurlType::Thailand, &mut body_data_json).unwrap_or_default();
    } else {
        if params.is_app {
            remove_parameters_playurl(PlayurlType::ChinaApp, &mut body_data_json)
                .unwrap_or_default();
        } else {
            remove_parameters_playurl(PlayurlType::ChinaWeb, &mut body_data_json)
                .unwrap_or_default();
        }
    }
    Ok(body_data)
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