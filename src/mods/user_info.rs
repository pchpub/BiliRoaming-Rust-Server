use super::cache::{get_cached_blacklist_info, get_cached_user_info};
use super::request::{async_getwebpage, async_postwebpage, redis_get, redis_set};
use super::types::{BiliConfig, ResignInfo, UserInfo};
use super::upstream_res::{get_upstream_bili_account_info, get_upstream_blacklist_info};
use chrono::prelude::*;
use deadpool_redis::Pool;

pub async fn get_user_info(
    access_key: &str,
    app_key: &str,
    app_sec: &str,
    user_agent: &str,
    config: &BiliConfig,
    redis_pool: &Pool,
) -> Result<UserInfo, String> {
    // mixed with blacklist function
    match get_cached_user_info(access_key, redis_pool).await {
        Some(value) => Ok(value),
        None => match get_upstream_bili_account_info(
            access_key, app_key, app_sec, user_agent, config, redis_pool
        )
        .await
        {
            Ok(value) => Ok(value),
            Err(value) => Err(value),
        },
    }
}

pub async fn get_blacklist_info(
    uid: &u64,
    config: &BiliConfig,
    redis_pool: &Pool,
) -> Result<(bool, bool), String> {
    match &config.blacklist_config {
        super::types::BlackListType::OnlyLocalBlackList => {
            match config.local_wblist.get(&uid.to_string()) {
                Some(value) => {
                    return Ok((value.0, value.1));
                }
                None => {
                    return Ok((true, false));
                }
            }
        }
        super::types::BlackListType::OnlyOnlineBlackList(online_blacklist_config) => {
            let dt = Local::now();
            let ts = dt.timestamp() as u64;
            let data = match get_cached_blacklist_info(uid, redis_pool).await {
                Some(value) => {
                    if value.status_expire_time < ts {
                        match get_upstream_blacklist_info(online_blacklist_config, uid).await {
                            Some(value) => value,
                            None => return Err("鉴权失败了喵".to_string()),
                        }
                    } else {
                        value
                    }
                }
                None => match get_upstream_blacklist_info(online_blacklist_config, uid).await {
                    Some(value) => value,
                    None => return Err("鉴权失败了喵".to_string()),
                },
            };
            return Ok((data.black, data.white));
        }
        super::types::BlackListType::MixedBlackList(online_blacklist_config) => {
            match config.local_wblist.get(&uid.to_string()) {
                Some(value) => {
                    return Ok((value.0, value.1));
                }
                None => (),
            }
            let dt = Local::now();
            let ts = dt.timestamp() as u64;
            let data = match get_cached_blacklist_info(uid, redis_pool).await {
                Some(value) => {
                    if value.status_expire_time < ts {
                        match get_upstream_blacklist_info(online_blacklist_config, uid).await {
                            Some(value) => value,
                            None => return Err("鉴权失败了喵".to_string()),
                        }
                    } else {
                        value
                    }
                }
                None => match get_upstream_blacklist_info(online_blacklist_config, uid).await {
                    Some(value) => value,
                    None => return Err("鉴权失败了喵".to_string()),
                },
            };
            return Ok((data.black, data.white));
        }
    }
}

pub async fn get_resigned_access_key(
    area_num: &u8,
    user_agent: &str,
    config: &BiliConfig,
    redis_pool: &Pool,
) -> Option<(String, u64)> {
    if *config
        .resign_api_policy
        .get(&area_num.to_string())
        .unwrap_or(&false)
    {
        let key = format!("a{area_num}1201");
        let dt = Local::now();
        let ts = dt.timestamp() as u64;
        match redis_get(redis_pool, &key).await {
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
        let data = if let Ok(data) = async_getwebpage(&url, &false, "", "", "").await {
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
        let resign_info = ResignInfo {
            area_num: *area_num as i32,
            access_key: access_key.clone(),
            refresh_token: "".to_string(),
            expire_time: webgetpage_data_json["expire_time"]
                .as_u64()
                .unwrap_or(ts + 3600),
        };

        redis_set(redis_pool, &key, &resign_info.to_json(), 3600).await;
        return Some((access_key, resign_info.expire_time));
    } else {
        let area_num = match area_num {
            4 => 4,
            _ => 1,
        };
        let resign_info_str = match redis_get(redis_pool, &format!("a{area_num}1101")).await {
            Some(value) => value,
            None => return None,
        };
        let resign_info_json: ResignInfo = serde_json::from_str(&resign_info_str).unwrap();
        let dt = Local::now();
        let ts = dt.timestamp() as u64;
        if resign_info_json.expire_time > ts {
            return Some((resign_info_json.access_key, resign_info_json.expire_time));
        } else {
            let sub_area_num: u8 = match area_num {
                4 => 4,
                _ => 1,
            };
            get_accesskey_from_token(redis_pool, sub_area_num, user_agent, config).await
        }
    }
}

async fn get_accesskey_from_token(
    pool: &Pool,
    sub_area_num: u8,
    user_agent: &str,
    config: &BiliConfig,
) -> Option<(String, u64)> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let resign_info = to_resign_info(
        &redis_get(pool, &format!("a{sub_area_num}1101"))
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
        match async_postwebpage(&url, &content, proxy_open, proxy_url, user_agent).await {
            Ok(value) => value,
            Err(_) => return None,
        };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    let resign_info = ResignInfo {
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
    redis_set(
        pool,
        &format!("a{sub_area_num}1101"),
        &resign_info.to_json(),
        0,
    )
    .await;
    Some((resign_info.access_key, resign_info.expire_time))
}

async fn to_resign_info(resin_info_str: &str) -> ResignInfo {
    serde_json::from_str(resin_info_str).unwrap()
}
