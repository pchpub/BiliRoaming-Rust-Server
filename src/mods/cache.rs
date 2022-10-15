use super::request::{redis_get, redis_set};
use super::types::*;
use async_channel::Sender;
use async_channel::TrySendError;
use chrono::prelude::*;
use deadpool_redis::Pool;
use qstring::QString;
use std::sync::Arc;
/*
番剧区域缓存
*/
#[inline]
pub async fn get_cached_ep_area(
    params: &BiliPlayurlParams<'_>,
    redis_pool: &Pool,
) -> Result<GetEpAreaType, ()> {
    let ep_id = params.ep_id;
    let req_area_num = params.area_num as u8;
    let key = format!("e{ep_id}1401");
    let data_raw = redis_get(redis_pool, &key).await;
    if let Some(value) = data_raw {
        let mut ep_area_data: [u8; 4] = [2, 2, 2, 2];
        let mut is_all_available = true;
        for (index, char) in value.char_indices() {
            match char {
                '0' => {
                    ep_area_data[index] = 0; //0表示正常
                }
                '1' => {
                    ep_area_data[index] = 1; //非0不正常
                }
                '2' => {
                    is_all_available = false;
                }
                _ => {}
            }
        }

        if is_all_available {
            if req_area_num == 4 && ep_area_data[3] == 0 {
                return Ok(GetEpAreaType::Available(Area::Th));
            } else if ep_area_data[req_area_num as usize - 1] == 0 {
                return Ok(GetEpAreaType::Available(Area::new(req_area_num)));
            } else {
                if ep_area_data[1] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Hk));
                } else if ep_area_data[2] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Tw));
                } else if ep_area_data[3] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Th));
                } else if ep_area_data[0] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Cn));
                } else {
                    return Err(()); //不这样搞的话可能被攻击时会出大问题
                }
            }
        } else {
            if ep_area_data[req_area_num as usize - 1] == 0 {
                if req_area_num == 2 && ep_area_data[1] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Hk));
                } else {
                    return Ok(GetEpAreaType::Available(Area::new(req_area_num)));
                }
            } else {
                return Ok(GetEpAreaType::NoCurrentAreaData(key, value));
            }
        }
    } else {
        return Ok(GetEpAreaType::NoEpData(key));
    };
}

#[inline]
pub async fn update_area_cache(
    http_body: &str,
    params: &BiliPlayurlParams<'_>,
    key: &str,
    value: &str,
    redis_pool: &Pool,
) {
    let is_available = check_ep_available(http_body);
    let area_num = params.area_num as usize;
    let new_value = {
        if is_available {
            value[..area_num - 1].to_owned() + "0" + &value[area_num..]
        } else {
            value[..area_num - 1].to_owned() + "1" + &value[area_num..]
        }
    };
    let _ = redis_set(redis_pool, key, &new_value, 0).await;
}

#[inline]
fn check_ep_available(http_body: &str) -> bool {
    // 此处判断来自 @cxw620
    let http_body_json: serde_json::Value = serde_json::from_str(http_body).unwrap();
    let code = http_body_json["code"].as_i64().unwrap_or(233);
    let message = http_body_json["message"].as_str().unwrap_or("").clone();
    /*
        {"code":10015002,"message":"访问权限不足","ttl":1}
        {"code":-10403,"message":"大会员专享限制"}
        {"code":-10403,"message":"抱歉您所使用的平台不可观看！"}
        {"code":-10403,"message":"抱歉您所在地区不可观看！"}
        {"code":-400,"message":"请求错误"}
        {"code":-404,"message":"啥都木有"}
        {"code":-404,"message":"啥都木有","ttl":1}
    */
    match code {
        0 => return true,
        -10403 => {
            if message == "大会员专享限制" || message == "抱歉您所使用的平台不可观看！"
            {
                return true;
            } else {
                return false;
            }
        }
        10015002 => {
            if message == "访问权限不足" {
                return true;
            } else {
                return false;
            }
        }
        -10500 => {
            return true;
            // 万恶的米奇妙妙屋,不用家宽就 -10500
            // link: https://t.me/biliroaming_chat/1231065
            //       https://t.me/biliroaming_chat/1231113
        }
        -404 => {
            return false;
        }
        _ => return false,
    }
}

/*
用户信息缓存
*/
// blacklist info cache
// redis_set(redis, &key, &return_data.to_json(), 1 * 24 * 60 * 60).await;
pub async fn get_cached_user_info(access_key: &str, redis_pool: &Pool) -> Option<UserInfo> {
    match redis_get(redis_pool, &format!("{access_key}20501")).await {
        Some(value) => Some(serde_json::from_str(&value).unwrap()),
        None => None,
    }
}

pub async fn get_cached_blacklist_info(
    uid: &u64,
    redis_pool: &Pool,
) -> Option<UserCerinfo> {
    //turn to ver 02
    match redis_get(redis_pool, &format!("{uid}20602")).await {
        Some(value) => Some(serde_json::from_str(&value).unwrap()),
        None => None,
    }
}
// pub async fn set_cached_user_info(access_key: &str, cache: &mut BiliCache) -> AuthUserInfo {
//     let key  = format!("{access_key}20501");
//     let value = new_data.to_json();
//     let _ : () = redis_set(&redis,&key, &value,25*24*60*60).await.unwrap_or_default();
//     let _ : () = redis_set(&redis,&format!("u{}20501",new_data.uid), &access_key.to_owned(),25*24*60*60).await.unwrap_or_default();
// }

// async fn update_cached_user_info_redis() {

// }

/*
播放链接缓存
*/
pub async fn get_cached_playurl(
    params: &BiliPlayurlParams<'_>,
    bilisender: Arc<Sender<SendData>>,
    redis_pool: &Pool,
) -> Result<String, ()> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let key = get_plaurl_cache_key(
        params.ep_id,
        params.cid,
        params.area_num as u8,
        params.is_app,
        params.is_tv,
        params.is_vip,
    );

    let need_fresh: bool;
    let return_data;
    let cached_data = match redis_get(&redis_pool, &key).await {
        Some(value) => Some(value),
        None => None,
    };

    match cached_data {
        Some(value) => {
            let cached_data_expire_time = &value[..13].parse::<u64>().unwrap();
            if cached_data_expire_time - 1200000 > ts {
                need_fresh = false;
                return_data = value[13..].to_string();
            } else if cached_data_expire_time < &ts {
                return Err(());
            } else {
                need_fresh = true;
                return_data = value[13..].to_string();
            }
        }
        None => return Err(()),
    }
    if need_fresh {
        let playurl_to_fresh_data = SendData::Playurl(SendPlayurlData {
            access_key: params.access_key.to_string(),
            app_key: params.app_key.to_string(),
            app_sec: params.app_sec.to_string(),
            ep_id: params.ep_id.to_string(),
            cid: params.cid.to_string(),
            build: params.build.to_string(),
            device: params.device.to_string(),
            is_app: params.is_app,
            is_tv: params.is_tv,
            is_th: params.is_th,
            is_vip: params.is_vip,
            area: params.area.to_string(),
            area_num: params.area_num,
            user_agent: params.user_agent.to_string(),
        });
        tokio::spawn(async move {
            //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
            match bilisender.try_send(playurl_to_fresh_data) {
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
    Ok(return_data)
}

pub async fn update_cached_playurl(
    ep_id: &str,
    cid: &str,
    area_num: u8,
    is_app: bool,
    is_th: bool,
    is_tv: bool,
    is_vip: bool,
    body_data: &str,
    config: &BiliConfig,
    redis_pool: &Pool
) {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let key = get_plaurl_cache_key(ep_id, cid, area_num, is_app, is_tv, is_vip);

    let mut body_data_json: serde_json::Value = serde_json::from_str(body_data).unwrap();
    let code = body_data_json["code"].as_i64().unwrap();

    let playurl_type: PlayurlType;
    if is_th {
        playurl_type = PlayurlType::Thailand;
    } else if is_tv {
        playurl_type = PlayurlType::ChinaTv;
    } else if is_app {
        playurl_type = PlayurlType::ChinaApp;
    } else {
        playurl_type = PlayurlType::ChinaWeb;
    }

    let expire_time = match get_playurl_deadline(playurl_type, &mut body_data_json) {
        Ok(value) => value - ts / 1000,
        Err(_) => match config.cache.get(&code.to_string()) {
            Some(value) => value,
            None => config.cache.get("other").unwrap(),
        }
        .clone(),
    };
    let value = format!("{}{body_data}", ts + expire_time * 1000);
    let _: () = redis_set(&redis_pool, &key, &value, expire_time)
        .await
        .unwrap_or_default();
}

fn get_plaurl_cache_key(
    ep_id: &str,
    cid: &str,
    area_num: u8,
    is_app: bool,
    is_tv: bool,
    is_vip: bool,
) -> String {
    match is_app {
        true => {
            if is_tv {
                format!("e{}c{}v{is_vip}t1{area_num}0101", ep_id, cid)
            } else {
                format!("e{}c{}v{is_vip}t0{area_num}0101", ep_id, cid)
            }
        }
        false => format!("e{}c{}v{is_vip}t0{area_num}0701", ep_id, cid),
    }
}

fn get_playurl_deadline(
    playurl_type: PlayurlType,
    data: &mut serde_json::Value,
) -> Result<u64, ()> {
    fn get_query_string(url: &str) -> Result<&str, ()> {
        let mut index = 0;
        for char in url.chars() {
            if char == '?' {
                return Ok(&url[index..]);
            }
            index += 1;
        }
        Err(())
    }
    match playurl_type {
        PlayurlType::Thailand => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let items =
                    if let Some(value) = data["data"]["video_info"]["stream_list"].as_array_mut() {
                        value
                    } else {
                        return Err(());
                    };
                for item in items {
                    match item["dash_video"]["base_url"].as_str() {
                        Some(value) => {
                            let query_string = if let Ok(value) = get_query_string(value) {
                                value.replace(r#"\u0026"#, r#"\n"#)
                            } else {
                                return Err(());
                            };
                            let query = QString::from(&query_string[..]);
                            if let Some(value) = query.get("deadline") {
                                return Ok(value.parse::<u64>().unwrap());
                            }
                        }
                        None => (),
                    }
                }
                return Err(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaApp => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let items = if let Some(value) = data["dash"]["video"].as_array_mut() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    match item["base_url"].as_str() {
                        Some(value) => {
                            let query_string = if let Ok(value) = get_query_string(value) {
                                value
                            } else {
                                return Err(());
                            };
                            let query = QString::from(query_string);
                            if let Some(value) = query.get("deadline") {
                                return Ok(value.parse::<u64>().unwrap());
                            }
                        }
                        None => (),
                    }
                }
                return Err(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaWeb => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let items = if let Some(value) = data["result"]["dash"]["video"].as_array_mut() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    match item["base_url"].as_str() {
                        Some(value) => {
                            let query_string = if let Ok(value) = get_query_string(value) {
                                value
                            } else {
                                return Err(());
                            };
                            let query = QString::from(query_string);
                            if let Some(value) = query.get("deadline") {
                                return Ok(value.parse::<u64>().unwrap());
                            }
                        }
                        None => (),
                    }
                }
                return Err(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaTv => {
            return Err(());
        }
    }
}
