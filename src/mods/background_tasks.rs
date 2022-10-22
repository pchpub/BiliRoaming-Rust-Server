use std::sync::Arc;

use super::health::*;
use super::push::send_report;
use super::request::redis_set;
use super::request::{async_getwebpage, redis_get};
use super::types::{BackgroundTaskType, BiliConfig, CacheTask, HealthTask, ReportConfig};
use super::upstream_res::*;
use super::user_info::{get_blacklist_info, get_user_info};
use deadpool_redis::Pool;
use serde_json::json;

pub async fn background_task_init() {}

pub async fn background_task_run(
    task: BackgroundTaskType,
    config: Arc<BiliConfig>,
    report_config: Arc<ReportConfig>,
    redis_pool: Arc<Pool>,
) -> Result<(), String> {
    match task {
        BackgroundTaskType::HealthTask(value) => match value {
            HealthTask::HealthCheck(value) => {
                for area_num in &value.need_check_area {
                    if let Err(value) = check_proxy_health(*area_num, &redis_pool, &config).await {
                        println!("[Background Task] Proxy health check failed: {value}");
                    }
                }
                Ok(())
            }
            HealthTask::HealthReport(value) => {
                send_report(&redis_pool, &report_config, &value).await
            }
        },
        BackgroundTaskType::CacheTask(value) => match value {
            CacheTask::UserInfoCacheRefresh(user_info) => {
                let access_key = user_info.access_key;
                let app_key = "1d8b6e7d45233436";
                let app_sec = "560c52ccd288fed045859ed18bffd973";
                let user_agent =
                    "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
                match get_user_info(&access_key, app_key, app_sec, user_agent, true, &config, &redis_pool).await {
                    Ok(new_user_info) => match get_blacklist_info(&new_user_info.uid, &config, &redis_pool).await {
                        Ok(_) => Ok(()),
                        Err(value) => Err(format!("[Background Task] UID {} | Refreshing blacklist info failed, ErrMsg: {value}", user_info.uid)),
                    },
                    Err(value) => Err(format!("[Background Task] UID {} | Refreshing blacklist info failed, ErrMsg: {value}", user_info.uid)),
                }
            }
            CacheTask::PlayurlCacheRefresh(value) => {
                match get_upstream_bili_playurl_background(&value, &config).await {
                    Ok(_) => Ok(()),
                    Err(value) => Err(format!(
                        "[Background Task] | Playurl cache refresh failed, ErrMsg: {value}"
                    )),
                }
            }
            CacheTask::EpInfoCacheRefresh => todo!(),
            CacheTask::EpAreaCacheRefresh(ep_id) => {
                // only for area cn, hk, tw, area th not intend to support
                // // 没弹幕/评论区还不如去看RC-RAWS
                let ep_id = &ep_id;
                let bili_user_status_api: &str =
                        "https://api.bilibili.com/pgc/view/web/season/user/status";
                    let user_agent =
                        "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
                    let access_key = if let Some(value) = redis_get(&redis_pool, "a1301").await {
                        value
                    } else {
                        return Err("[ERROR] fail to get access_key".to_string());
                    };
                    let area_to_check = [
                        (
                            format!("{bili_user_status_api}?access_key={access_key}&ep_id={ep_id}"),
                            &config.hk_proxy_playurl_open,
                            &config.hk_proxy_playurl_url,
                        ),
                        (
                            format!("{bili_user_status_api}?access_key={access_key}&ep_id={ep_id}"),
                            &config.tw_proxy_playurl_open,
                            &config.tw_proxy_playurl_url,
                        ),
                        (
                            format!("{bili_user_status_api}?access_key={access_key}&ep_id={ep_id}"),
                            &config.cn_proxy_playurl_open,
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
            },
        },
    }
}
