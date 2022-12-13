use super::background_tasks::{update_cached_area_background, update_cached_playurl_background};
use super::tools::vec_to_string;
use super::types::*;
use chrono::prelude::*;
use log::debug;
use qstring::QString;

/*
番剧区域缓存
*/
#[inline]
pub async fn get_cached_ep_area(
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<Option<Area>, EType> {
    let ep_id = params.ep_id;
    let req_area_num = params.area_num as u8;
    // let key = format!("e{ep_id}1401");
    let data_raw = bili_runtime.get_cache(&CacheType::EpArea(ep_id)).await;
    let result = if let Some(value) = data_raw {
        let mut ep_area_data: [u8; 4] = [2, 2, 2, 2];
        // let mut is_all_available = true;
        for (index, char) in value.char_indices() {
            match char {
                '0' => {
                    ep_area_data[index] = 0; //0表示正常
                }
                '1' => {
                    ep_area_data[index] = 1; //非0不正常
                }
                // '2' => {
                //     // means has area which is never accessed
                //     is_all_available = false;
                // }
                _ => {}
            }
        }
        match ep_area_data[req_area_num as usize - 1] {
            0 => {
                if req_area_num == 3 && ep_area_data[1] == 0 {
                    Some(Area::Hk)
                } else {
                    Some(Area::new(req_area_num))
                }
            }
            1 => {
                if req_area_num != 4 {
                    for (i, item) in ep_area_data.iter().enumerate() {
                        if i < 3 && *item == 0 {
                            return Ok(Some(Area::new(i as u8 + 1)));
                        } else if *item == 2 {
                            update_cached_area_background(params, bili_runtime).await;
                            return Ok(None);
                        }
                    }
                    // cannot be all 111*
                    update_cached_area_background(params, bili_runtime).await;
                    return Err(EType::OtherError(-404, "非主站番剧"));
                } else {
                    return Err(EType::OtherError(-404, "非东南亚区番剧"));
                }
            }
            2 => {
                update_cached_area_background(params, bili_runtime).await;
                if req_area_num != 4 {
                    for (i, item) in ep_area_data.iter().enumerate() {
                        if i < 3 && *item == 0 {
                            return Ok(Some(Area::new(i as u8 + 1)));
                        }
                    }
                }
                None
            }
            _ => None,
        }

    //     if is_all_available {
    //         if req_area_num == 4 && ep_area_data[3] == 0 {
    //             Some(Area::Th)
    //         } else if ep_area_data[req_area_num as usize - 1] == 0 {
    //             Some(Area::new(req_area_num))
    //         } else {
    //             if ep_area_data[1] == 0 {
    //                 Some(Area::Hk)
    //             } else if ep_area_data[2] == 0 {
    //                 Some(Area::Tw)
    //             } else if ep_area_data[3] == 0 {
    //                 Some(Area::Th)
    //             } else if ep_area_data[0] == 0 {
    //                 Some(Area::Cn)
    //             } else {
    //                 None //不这样搞的话可能被攻击时会出大问题
    //             }
    //         }
    //     } else {
    //         if req_area_num == 4 && ep_area_data[3] == 1 {
    //             // fix zone th's playurl struct not eq to normal one
    //             None
    //         // here just for area hk priority
    //         } else if ep_area_data[req_area_num as usize - 1] == 0 {
    //             // if req_area == tw && hk_is_available
    //             if req_area_num == 2 && ep_area_data[1] == 0 {
    //                 Some(Area::Hk)
    //             } else {
    //                 Some(Area::new(req_area_num))
    //             }
    //         } else if ep_area_data[req_area_num as usize - 1] == 2 {
    //             update_cached_area_background(params, bili_runtime).await;
    //             None
    //         } else {
    //             None
    //         }
    //     }
    } else {
        update_cached_area_background(params, bili_runtime).await;
        None
    };
    Ok(result)
}

#[inline]
pub async fn update_area_cache(
    http_body_json: &serde_json::Value,
    params: &PlayurlParams<'_>,
    // key: &str,
    // value: &str,
    bili_runtime: &BiliRuntime<'_>,
) {
    let ep_id = params.ep_id;
    let is_available = check_ep_available(http_body_json);
    let area_num = params.area_num as usize;
    let cache_type = CacheType::EpArea(ep_id);
    let value = bili_runtime
        .get_cache(&cache_type)
        .await
        .unwrap_or("2222".to_string());
    let new_value = {
        if is_available {
            value[..area_num - 1].to_owned() + "0" + &value[area_num..]
        } else {
            value[..area_num - 1].to_owned() + "1" + &value[area_num..]
        }
    };
    debug!(
        "[UPDATE_CACHE] AREA {} | EP {} -> is available: {}. New area cache data: {}",
        params.area.to_ascii_uppercase(),
        params.ep_id,
        is_available,
        new_value
    );
    bili_runtime.update_cache(&cache_type, &new_value, 0).await;
}

#[inline]
pub fn check_ep_available(http_body_json: &serde_json::Value) -> bool {
    // 此处判断来自 @cxw620
    // let http_body_json: serde_json::Value = serde_json::from_str(http_body).unwrap();
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
pub async fn get_cached_user_info(
    access_key: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Option<UserInfo> {
    match bili_runtime
        .get_cache(&CacheType::UserInfo(access_key, 1145141919810))
        .await
    {
        // TODO: 处理expire_time, 主动刷新
        Some(value) => Some(serde_json::from_str(&value).unwrap()),
        None => None,
    }
}

/// `update_user_info_cache` 保存UserInfo信息到本地缓存
pub async fn update_user_info_cache(new_user_info: &UserInfo, bili_runtime: &BiliRuntime<'_>) {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let access_key = &new_user_info.access_key;
    let uid = new_user_info.uid;
    // let key = format!("{access_key}20501");
    let value = new_user_info.to_json();
    let expire_time = (new_user_info.expire_time - ts) / 1000;
    // let _: () = redis_set(redis_pool, &key, &value, expire_time)
    //     .await
    //     .unwrap_or_default();
    // let _: () = redis_set(
    //     redis_pool,
    //     &format!("u{}20501", new_user_info.uid),
    //     &access_key.to_owned(),
    //     expire_time,
    // )
    // .await
    debug!(
        "[UPDATE_CACHE] UID {} | AK {} -> is VIP: {}. New user_info cache data: {}",
        new_user_info.uid,
        new_user_info.access_key,
        new_user_info.is_vip(),
        value
    );
    bili_runtime
        .update_cache(&CacheType::UserInfo(access_key, uid), &value, expire_time)
        .await;
    // for health check
    if new_user_info.is_vip() {
        bili_runtime
            .redis_set("uv11301", &new_user_info.uid.to_string(), expire_time)
            .await;
        bili_runtime
            .redis_set("av11301", &new_user_info.access_key, expire_time)
            .await;
        // 此处保存vip用户的access_key到本地使用, 02版本号, 刷新access_token的方法比较麻烦
        bili_runtime.redis_set("a11102", &new_user_info.access_key, expire_time).await
    } else {
        bili_runtime
            .redis_set("uv01301", &new_user_info.uid.to_string(), expire_time)
            .await;
        bili_runtime
            .redis_set("av01301", &new_user_info.access_key, expire_time)
            .await;
    }
}

pub async fn get_cached_blacklist_info(
    user_info: &UserInfo,
    bili_runtime: &BiliRuntime<'_>,
) -> Option<UserCerinfo> {
    //turn to ver 02
    let uid = &user_info.uid;
    let access_key = &user_info.access_key;
    let cache_type = CacheType::UserCerInfo(access_key, *uid);
    if let Some(cached_value) = bili_runtime.get_cache(&cache_type).await {
        match serde_json::from_str(&cached_value) {
            Ok(user_cer_info) => {
                let user_cer_info: UserCerinfo = user_cer_info;
                debug!(
                    "[GET_CACHE][UserCerInfo] UID {} | AK {} -> white {} black {} ban_until {}",
                    user_info.uid,
                    user_info.access_key,
                    user_cer_info.white,
                    user_cer_info.black,
                    user_cer_info.ban_until
                );
                Some(user_cer_info)
            }
            Err(_) => None,
        }
    } else {
        None
    }
}
/// `update_blacklist_info_cache` 保存UserCerinfo信息到本地缓存
pub async fn update_blacklist_info_cache(
    user_info: &UserInfo,
    new_user_cer_info: &UserCerinfo,
    bili_runtime: &BiliRuntime<'_>,
) {
    debug!(
        "[UPDATE_CACHE][UserCerInfo] UID {} | AK {} -> white {} black {} ban_until {}",
        user_info.uid,
        user_info.access_key,
        new_user_cer_info.white,
        new_user_cer_info.black,
        new_user_cer_info.ban_until
    );
    let value = new_user_cer_info.to_json();
    let cache_type = CacheType::UserCerInfo(&user_info.access_key, user_info.uid);
    bili_runtime
        .update_cache(&cache_type, &value, 1 * 24 * 60 * 60)
        .await;
    // bili_runtime
    //     .redis_set(
    //         &format!("{}20602", &user_info.uid),
    //         &value,
    //         1 * 24 * 60 * 60,
    //     )
    //     .await;
    // bili_runtime
    //     .redis_set(
    //         &format!("a{}20602", &user_info.access_key),
    //         &value,
    //         1 * 24 * 60 * 60,
    //     )
    //     .await;
}

/*
播放链接缓存
*/
pub async fn get_cached_playurl(
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<String, ()> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let cache_type = CacheType::Playurl(params);
    let cached_data_expire_time;
    let need_fresh: bool;
    let return_data;
    let cached_data = bili_runtime.get_cache(&cache_type).await;

    match cached_data {
        Some(value) => {
            cached_data_expire_time = value[..13].parse::<u64>().unwrap();
            debug!(
                "[GET PLAYURL][C] AREA {} | EP {} -> is_app: {} is_tv: {} is_vip {} expire_time {} CacheKey: {} 获取缓存成功 ",
                params.area.to_ascii_uppercase(),
                params.ep_id,
                params.is_app,
                params.is_tv,
                params.is_vip,
                cached_data_expire_time,
                vec_to_string(&cache_type.gen_key(),"|")
            );
            if cached_data_expire_time - 1200000 > ts {
                need_fresh = false;
                return_data = value[13..].to_string();
            } else if cached_data_expire_time < ts {
                return Err(());
            } else {
                need_fresh = true;
                return_data = value[13..].to_string();
            }
        }
        None => return Err(()),
    }
    if need_fresh {
        update_cached_playurl_background(params, bili_runtime).await;
    }
    Ok(return_data)
}

pub async fn update_cached_playurl(
    params: &mut PlayurlParams<'_>,
    body_data: &str,
    bili_runtime: &BiliRuntime<'_>,
) {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;

    params.init_ep_need_vip(bili_runtime).await;
    let cache_type = CacheType::Playurl(params);

    let mut body_data_json: serde_json::Value = serde_json::from_str(body_data).unwrap();
    let code = body_data_json["code"].as_i64().unwrap();

    let playurl_type: PlayurlType;
    if params.is_th {
        playurl_type = PlayurlType::Thailand;
    } else if params.is_tv {
        playurl_type = PlayurlType::ChinaTv;
    } else if params.is_app {
        playurl_type = PlayurlType::ChinaApp;
    } else {
        playurl_type = PlayurlType::ChinaWeb;
    }

    let expire_time = match get_playurl_deadline(playurl_type, &mut body_data_json) {
        Ok(value) => value - ts / 1000,
        Err(_) => match bili_runtime.config.cache.get(&code.to_string()) {
            Some(value) => value,
            None => bili_runtime.config.cache.get("other").unwrap(),
        }
        .clone(),
    };
    let value = format!("{}{body_data}", ts + expire_time * 1000);
    bili_runtime
        .update_cache(&cache_type, &value, expire_time)
        .await;
    debug!(
        "[GET PLAYURL][C] AREA {} | EP {} -> is_app: {} is_tv: {} is_vip {} expire_time {} CacheKey: {} 写入缓存成功",
        params.area.to_ascii_uppercase(),
        params.ep_id,
        params.is_app,
        params.is_tv,
        params.is_vip,
        expire_time,
        vec_to_string(&cache_type.gen_key(),"|")
    );
}

#[inline]
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

/*
东南亚区season缓存
*/

pub async fn get_cached_th_season(
    season_id: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<String, ()> {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let redis_get_data: String;
    match bili_runtime
        .get_cache(&CacheType::ThSeason(season_id))
        .await
    {
        Some(value) => {
            debug!(
                "[GET TH_SEASON][C] AREA TH | SID {} -> CacheKey: {} 获取缓存成功 ",
                season_id,
                vec_to_string(&CacheType::ThSeason(season_id).gen_key(), "|")
            );
            let redis_get_data_expire_time = &value[..13].parse::<u64>().unwrap();
            if redis_get_data_expire_time > &ts {
                // TODO: add manual refresh
                redis_get_data = value[13..].to_string();
                Ok(redis_get_data)
            } else {
                Err(())
            }
        }
        None => Err(()),
    }
}

pub async fn update_th_season_cache(season_id: &str, data: &str, bili_runtime: &BiliRuntime<'_>) {
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let expire_time = match bili_runtime.config.cache.get(&"season".to_string()) {
        Some(value) => value,
        None => &1800,
    };
    let value = format!("{}{data}", ts + expire_time * 1000);
    bili_runtime
        .update_cache(&CacheType::ThSeason(season_id), &value, *expire_time)
        .await;
    debug!(
        "[GET TH_SEASON][C] AREA TH | SID {} -> CacheKey: {} 写入缓存成功",
        season_id,
        vec_to_string(&CacheType::ThSeason(season_id).gen_key(), "|")
    );
}

/*
* th subtitle 缓存
*/
pub async fn get_cached_th_subtitle(
    params: &PlayurlParams<'_>,
    // _raw_query: &str,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<String, bool> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    match bili_runtime
        .get_cache(&&CacheType::ThSubtitle(params.ep_id))
        .await
    {
        Some(value) => {
            debug!(
                "[GET TH_SUBTITLE][C] AREA TH | EP {} -> CacheKey: {} 获取缓存成功",
                params.ep_id,
                vec_to_string(&CacheType::ThSeason(params.ep_id).gen_key(), "|")
            );
            if &value[..13].parse::<u64>().unwrap() < &(ts * 1000) {
                Err(true)
            } else {
                Ok(value[13..].to_string())
            }
        }
        None => Err(true),
    }
}

pub async fn update_th_subtitle_cache(
    data: &str,
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let expire_time = bili_runtime.config.cache.get("thsub").unwrap_or(&14400);
    let value = format!("{}{data}", (ts + expire_time) * 1000);
    bili_runtime
        .update_cache(&CacheType::ThSubtitle(params.ep_id), &value, *expire_time)
        .await;
    debug!(
        "[GET TH_SUBTITLE][C] AREA TH | EP {} -> CacheKey: {} 写入缓存成功",
        params.ep_id,
        vec_to_string(&CacheType::ThSeason(params.ep_id).gen_key(), "|")
    );
}
