use crate::mods::cache::update_blacklist_info_cache;

use super::cache::{get_cached_blacklist_info, get_cached_user_info, update_user_info_cache};
use super::request::{async_getwebpage, async_postwebpage};
use super::types::{BiliRuntime, EType, PlayurlParams, UserInfo, UserResignInfo};
use super::upstream_res::{get_upstream_bili_account_info, get_upstream_blacklist_info};
use chrono::prelude::*;

// general
#[inline]
pub async fn get_user_info(
    access_key: &str,
    appkey: &str,
    appsec: &str,
    user_agent: &str,
    force_update: bool,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<UserInfo, EType> {
    // mixed with blacklist function
    if force_update {
        match get_upstream_bili_account_info(access_key, appkey, appsec, user_agent, bili_runtime)
            .await
        {
            Ok(value) => {
                update_user_info_cache(&value, bili_runtime).await;
                Ok(value)
            }
            Err(value) => Err(value),
        }
    } else {
        match get_cached_user_info(access_key, bili_runtime).await {
            Some(value) => Ok(value),
            None => match get_upstream_bili_account_info(
                access_key,
                appkey,
                appsec,
                user_agent,
                bili_runtime,
            )
            .await
            {
                Ok(value) => {
                    update_user_info_cache(&value, bili_runtime).await;
                    Ok(value)
                }
                Err(value) => {
                    let user_info = UserInfo {
                        access_key: access_key.to_owned(),
                        uid: 0,
                        vip_expire_time: 0,
                        expire_time: 0,
                    };
                    match get_blacklist_info(&user_info, bili_runtime).await {
                        Ok(_) => Ok(user_info),
                        Err(_) => Err(value),
                    }
                }
            },
        }
    }
}

#[inline]
pub async fn get_blacklist_info(
    user_info: &UserInfo,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<bool, EType> {
    // fn timestamp_to_time(timestamp: &u64) -> String {
    //     let dt = Utc
    //         .timestamp(*timestamp as i64, 0)
    //         .with_timezone(&FixedOffset::east(8 * 3600));
    //     dt.format(r#"%Y年%m月%d日 %H:%M解封\n请耐心等待"#)
    //         .to_string()
    // }
    // let uid = &user_info.uid;
    // let access_key = &user_info.access_key;
    match &bili_runtime.config.blacklist_config {
        super::types::BlackListType::OnlyLocalBlackList => {
            match bili_runtime
                .config
                .local_wblist
                .get(&user_info.uid.to_string())
            {
                Some(value) => {
                    if value.1 {
                        return Ok(true);
                    } else if value.0 {
                        return Err(EType::UserBlacklistedError(0));
                    }
                    {
                        Ok(false)
                    }
                }
                None => Err(EType::UserWhitelistedError),
            }
        }
        super::types::BlackListType::OnlyOnlineBlackList(_) => {
            let dt = Local::now();
            let ts = dt.timestamp() as u64;
            let data = match get_cached_blacklist_info(user_info, bili_runtime).await {
                Some(value) => {
                    if value.status_expire_time < ts {
                        match get_upstream_blacklist_info(&user_info.uid, &bili_runtime).await {
                            Ok(value) => {
                                update_blacklist_info_cache(user_info, &value, bili_runtime).await;
                                value
                            }
                            Err(value) => return Err(value),
                        }
                    } else {
                        value
                    }
                }
                None => match get_upstream_blacklist_info(&user_info.uid, &bili_runtime).await {
                    Ok(value) => {
                        update_blacklist_info_cache(user_info, &value, bili_runtime).await;
                        value
                    }
                    Err(value) => return Err(value),
                },
            };
            if data.white {
                Ok(true)
            } else if data.black {
                Err(EType::UserBlacklistedError(data.ban_until as i64))
            } else {
                Ok(false)
            }
        }
        super::types::BlackListType::MixedBlackList(_) => {
            match bili_runtime
                .config
                .local_wblist
                .get(&user_info.uid.to_string())
            {
                Some(value) => {
                    if value.1 {
                        return Ok(true);
                    } else if value.0 {
                        return Err(EType::UserBlacklistedError(0));
                    } else {
                        ()
                    }
                }
                None => (),
            }
            let dt = Local::now();
            let ts = dt.timestamp() as u64;
            let data = match get_cached_blacklist_info(user_info, bili_runtime).await {
                Some(value) => {
                    if value.status_expire_time < ts {
                        match get_upstream_blacklist_info(&user_info.uid, &bili_runtime).await {
                            Ok(value) => {
                                update_blacklist_info_cache(user_info, &value, bili_runtime).await;
                                value
                            }
                            Err(value) => return Err(value),
                        }
                    } else {
                        value
                    }
                }
                None => match get_upstream_blacklist_info(&user_info.uid, &bili_runtime).await {
                    Ok(value) => {
                        update_blacklist_info_cache(user_info, &value, bili_runtime).await;
                        value
                    }
                    Err(value) => return Err(value),
                },
            };
            if data.white {
                Ok(true)
            } else if data.black {
                Err(EType::UserBlacklistedError(data.ban_until as i64))
            } else {
                Ok(false)
            }
        }
    }
}

pub async fn resign_user_info(
    white: bool,
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<Option<(bool, String)>, EType> {
    let config = bili_runtime.config;
    let new_access_key;
    if params.is_th {
        if *config.resign_open.get("4").unwrap_or(&false)
            && (white || *config.resign_pub.get("4").unwrap_or(&false))
        {
            (new_access_key, _) = get_resigned_access_key(&4, &params.user_agent, bili_runtime)
                .await
                .unwrap_or((params.access_key.to_string(), 1));
            Ok(Some((true, new_access_key)))
        } else {
            Ok(None)
        }
    } else {
        if *config.resign_open.get("4").unwrap_or(&false)
            && (white
                || *config
                    .resign_pub
                    .get(&params.area_num.to_string())
                    .unwrap_or(&false))
        {
            (new_access_key, _) = get_resigned_access_key(&4, &params.user_agent, bili_runtime)
                .await
                .unwrap_or((params.access_key.to_string(), 1));
            let user_info = match get_user_info(
                params.access_key,
                params.appkey,
                params.appsec,
                params.user_agent,
                false,
                bili_runtime,
            )
            .await
            {
                Ok(value) => value,
                Err(value) => {
                    return Err(value);
                }
            };
            Ok(Some((user_info.is_vip(), new_access_key)))
        } else {
            Ok(None)
        }
    }
}
pub async fn get_resigned_access_key(
    area_num: &u8,
    user_agent: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Option<(String, u64)> {
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
        let data = if let Ok(data) = async_getwebpage(&url, false, "", "", "").await {
            data
        } else {
            println!("[Error] 从非官方接口处获取accesskey失败");
            return None;
        };
        let webgetpage_data_json: serde_json::Value = if let Ok(value) = serde_json::from_str(&data)
        {
            value
        } else {
            println!("[Error] json解析失败: {}", data);
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
        let area_num = match area_num {
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
            let sub_area_num: u8 = match area_num {
                4 => 4,
                _ => 1,
            };
            get_accesskey_from_token(sub_area_num, user_agent, bili_runtime).await
        }
    }
}

async fn get_accesskey_from_token(
    sub_area_num: u8,
    user_agent: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Option<(String, u64)> {
    let config = bili_runtime.config;
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let resign_info = to_resign_info(
        &bili_runtime
            .redis_get(&format!("a{sub_area_num}1101"))
            .await
            .unwrap(),
    )
    .await;
    let access_key = resign_info.access_key;
    let refresh_token = resign_info.refresh_token;
    let (url, content, proxy_open, proxy_url) = match sub_area_num {
        4 => (
            "https://passport.biliintl.com/x/intl/passport-login/oauth2/refresh_token",
            format!("access_token={access_key}&refresh_token={refresh_token}"),
            &config.th_proxy_token_open,
            &config.th_proxy_token_url,
        ),
        1 => {
            let unsign_request_body = format!(
                "access_token={access_key}&appkey=1d8b6e7d45233436&refresh_token={refresh_token}&ts={ts}"
            );
            (
                "https://passport.bilibili.com/x/passport-login/oauth2/refresh_token",
                format!(
                    "{unsign_request_body}&sign={:x}",
                    md5::compute(format!(
                        "{unsign_request_body}560c52ccd288fed045859ed18bffd973"
                    ))
                ),
                &config.cn_proxy_token_open,
                &config.cn_proxy_token_url,
            )
        }
        _ => return None,
    };
    let getpost_string =
        match async_postwebpage(&url, &content, *proxy_open, proxy_url, user_agent).await {
            Ok(value) => value,
            Err(_) => return None,
        };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    let resign_info = UserResignInfo {
        area_num: sub_area_num as i32,
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
        .redis_set(&format!("a{sub_area_num}1101"), &resign_info.to_json(), 0)
        .await;
    Some((resign_info.access_key, resign_info.expire_time))
}

async fn to_resign_info(resin_info_str: &str) -> UserResignInfo {
    serde_json::from_str(resin_info_str).unwrap()
}

// // background task
// pub async fn get_user_info_background(
//     access_key: &str,
//     appkey: &str,
//     appsec: &str,
//     user_agent: &str,
//     bili_runtime: &BiliRuntime<'_>,
// ) -> Result<UserInfo, EType> {
//     // mixed with blacklist function
//     match get_cached_user_info(access_key, bili_runtime).await {
//         Some(value) => Ok(value),
//         None => {
//             match get_upstream_bili_account_info(
//                 access_key,
//                 appkey,
//                 appsec,
//                 user_agent,
//                 bili_runtime,
//             )
//             .await
//             {
//                 Ok(value) => Ok(value),
//                 Err(value) => Err(value),
//             }
//         }
//     }
// }
