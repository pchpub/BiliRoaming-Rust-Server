use super::health::*;
use super::push::send_report;
use super::request::redis_set;
use super::request::{async_getwebpage, redis_get};
use super::types::{
    BackgroundTaskType, BiliRuntime, CacheTask, HealthReportType, HealthTask, PlayurlParams,
    PlayurlParamsStatic,
};
use super::upstream_res::*;
use super::user_info::{get_blacklist_info, get_user_info};
use async_channel::{Sender, TrySendError};
use serde_json::json;
use std::sync::Arc;

/*
* The following is for generate background task
*/

pub async fn update_cached_playurl_background(
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) {
    let background_task_data =
        BackgroundTaskType::CacheTask(CacheTask::PlayurlCacheRefresh(PlayurlParamsStatic {
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
        }));
    bili_runtime.send_task(background_task_data).await
}

pub async fn update_area_cache_background(
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) {
    let background_task_data =
        BackgroundTaskType::CacheTask(CacheTask::EpAreaCacheRefresh(params.ep_id.to_owned(), params.access_key.to_owned()));
    bili_runtime.send_task(background_task_data).await
}

pub async fn update_cached_user_info_background(
    access_key: String,
    bilisender: Arc<Sender<BackgroundTaskType>>,
) {
    let background_task_data =
        BackgroundTaskType::CacheTask(CacheTask::UserInfoCacheRefresh(access_key));
    tokio::spawn(async move {
        //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
        match bilisender.try_send(background_task_data) {
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

pub async fn background_task_run(
    task: BackgroundTaskType,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<(), String> {
    let config = bili_runtime.config;
    let redis_pool = bili_runtime.redis_pool;
    let report_config = &bili_runtime.config.report_config;
    match task {
        BackgroundTaskType::HealthTask(value) => match value {
            HealthTask::HealthCheck(value) => {
                for area_num in &value.need_check_area {
                    if let Err(value) = check_proxy_health(*area_num, bili_runtime).await {
                        println!("[Background Task] Proxy health check failed: {value}");
                    }
                }
                Ok(())
            }
            HealthTask::HealthReport(value) => {
                if config.report_open {
                    let redis_key = match &value {
                        HealthReportType::Playurl(value) => {
                            let redis_key_vec: Vec<u8> = vec![01, value.area_num, 13, 01];
                            String::from_utf8(redis_key_vec).unwrap()
                        }
                        HealthReportType::Search(value) => {
                            let redis_key_vec: Vec<u8> = vec![02, value.area_num, 13, 01];
                            String::from_utf8(redis_key_vec).unwrap()
                        }
                        HealthReportType::ThSeason(value) => {
                            let redis_key_vec: Vec<u8> = vec![04, value.area_num, 13, 01];
                            String::from_utf8(redis_key_vec).unwrap()
                        }
                        HealthReportType::Others(_) => return Ok(()),
                    };
                    let is_available = value.is_available();
                    if is_available {
                        match redis_get(&redis_pool, &redis_key).await {
                            Some(value) => {
                                let err_num = value.parse::<u16>().unwrap_or(4);
                                if err_num >= 4 {
                                    redis_set(&redis_pool, &redis_key, "0", 0)
                                        .await
                                        .unwrap_or_default();
                                } else if err_num != 0 {
                                    redis_set(&redis_pool, &redis_key, "0", 0)
                                        .await
                                        .unwrap_or_default();
                                }
                            }
                            None => {
                                redis_set(&redis_pool, &redis_key, "0", 0)
                                    .await
                                    .unwrap_or_default();
                            }
                        }
                    } else {
                        let num = redis_get(&redis_pool, &redis_key)
                            .await
                            .unwrap_or("0".to_string())
                            .as_str()
                            .parse::<u32>()
                            .unwrap();
                        if num == 4 {
                            redis_set(&redis_pool, &redis_key, "1", 0)
                                .await
                                .unwrap_or_default();
                        } else {
                            redis_set(&redis_pool, &redis_key, &(num + 1).to_string(), 0)
                                .await
                                .unwrap_or_default();
                        }
                    }
                    send_report(&redis_pool, &report_config, &value).await
                } else {
                    Ok(())
                }
            }
        },
        BackgroundTaskType::CacheTask(value) => match value {
            CacheTask::UserInfoCacheRefresh(access_key) => {
                let app_key = "1d8b6e7d45233436";
                let app_sec = "560c52ccd288fed045859ed18bffd973";
                let user_agent =
                    "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
                match get_user_info(&access_key, app_key, app_sec, user_agent, true, &bili_runtime).await {
                    Ok(new_user_info) => match get_blacklist_info(&new_user_info, bili_runtime).await {
                        Ok(_) => Ok(()),
                        Err(value) => Err(format!("[Background Task] UID {} | Refreshing blacklist info failed, ErrMsg: {}", new_user_info.uid, value.err_json())),
                    },
                    Err(value) => Err(format!("[Background Task] ACCESS_KEY {} | Refreshing blacklist info failed, ErrMsg: {}", access_key, value.err_json())),
                }
            }
            CacheTask::PlayurlCacheRefresh(params) => {
                match get_upstream_bili_playurl_background(&params, bili_runtime).await {
                    Ok(_value) => {
                        // update_cached_playurl_background(params, &value, &redis_pool, &config)
                        // .await;
                        todo!()
                    }
                    Err(value) => Err(format!(
                        "[Background Task] | Playurl cache refresh failed, ErrMsg: {value}"
                    )),
                }
            }
            CacheTask::EpInfoCacheRefresh(force_update, ep_info_vec) => {
                let new_ep_info_vec = if force_update {
                    let ep_id = ep_info_vec[0].ep_id;
                    if let Ok((_, value)) =
                        get_upstream_bili_ep_info(&format!("{ep_id}"), false, "").await
                    {
                        value
                    } else {
                        return Err("[Background Task] ep info cache refresh failed".to_string());
                    }
                } else {
                    ep_info_vec
                };
                // TODO TOFIX
                // for ep_info in new_ep_info_vec {
                //     let redis_pool_cl = Arc::clone(&redis_pool);
                //     tokio::spawn(async move {
                //         update_cached_ep_info_redis(ep_info, &redis_pool_cl).await
                //     });
                // }
                Ok(())
            }
            CacheTask::ProactivePlayurlCacheRefresh => {

                todo!()
            },
            CacheTask::EpAreaCacheRefresh(ep_id, access_key) => {
                // only for area cn, hk, tw, area th not intend to support
                // // 没弹幕/评论区还不如去看RC-RAWS
                let bili_user_status_api: &str =
                    "https://api.bilibili.com/pgc/view/web/season/user/status";
                let user_agent =
                    "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
                let area_to_check = [
                    (
                        format!("{bili_user_status_api}?access_key={access_key}&ep_id={ep_id}"),
                        config.hk_proxy_playurl_open,
                        &config.hk_proxy_playurl_url,
                    ),
                    (
                        format!("{bili_user_status_api}?access_key={access_key}&ep_id={ep_id}"),
                        config.tw_proxy_playurl_open,
                        &config.tw_proxy_playurl_url,
                    ),
                    (
                        format!("{bili_user_status_api}?access_key={access_key}&ep_id={ep_id}"),
                        config.cn_proxy_playurl_open,
                        &config.cn_proxy_playurl_url,
                    ),
                ];
                let mut area_num: u8 = 0;
                let mut ep_area_data: [u8; 4] = [2, 2, 2, 2];
                for item in area_to_check {
                    area_num += 1;
                    let (url, proxy_open, proxy_url) = item;
                    match async_getwebpage(&url, proxy_open, proxy_url, user_agent, "").await {
                            Ok(value) => {
                                let json_result = serde_json::from_str(&value)
                                    .unwrap_or(json!({"code": -2333, "message": ""}));
                                let code = json_result["code"]
                                    .as_str()
                                    .unwrap_or("233")
                                    .parse::<i64>()
                                    .unwrap_or(233);
                                match code {
                                    0 => {
                                        let result = json_result.get("result").unwrap();
                                        if result["area_limit"].as_i64().unwrap() != 0 {
                                            ep_area_data[(area_num - 1) as usize] = 1;
                                        } else {
                                            ep_area_data[(area_num - 1) as usize] = 0;
                                        }
                                    }
                                    -2333 => {
                                        println!("Check EP {ep_id} available zone using proxy {proxy_open}-{proxy_url} failed -> Json Parsing Error: {value}");
                                        continue;
                                    }
                                    _ => {
                                        ep_area_data[(area_num - 1) as usize] = 1;
                                        println!("Check EP {ep_id} available zone using proxy {proxy_open}-{proxy_url} failed -> Unknown Error Code {code}: {value}");
                                        continue;
                                    }
                                }
                            }
                            Err(_) => println!("Check EP {ep_id} available zone using proxy {proxy_open}-{proxy_url} failed -> Network Error"),
                        }
                }
                let key = format!("e{ep_id}1401");
                let ep_area_data = &format!(
                    "{}{}{}{}",
                    ep_area_data[0], ep_area_data[1], ep_area_data[2], ep_area_data[3],
                );
                let _ = redis_set(&redis_pool, &key, &ep_area_data, 0).await;
                Ok(())
            }
            
        },
    }
}
