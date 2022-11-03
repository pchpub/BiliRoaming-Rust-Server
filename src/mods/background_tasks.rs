use super::cache::update_cached_playurl;
use super::health::*;
use super::push::send_report;
use super::request::async_getwebpage;
use super::types::{
    Area, BackgroundTaskType, BiliRuntime, CacheTask, HealthReportType, HealthTask,
    PlayurlParams, PlayurlParamsStatic, ReqType,
};
use super::upstream_res::*;
use super::user_info::{get_blacklist_info, get_user_info};
use log::{debug, trace};
use serde_json::json;

/*
* The following is for generate background task
*/

pub async fn update_cached_playurl_background(
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) {
    trace!(
        "[BACKGROUND TASK] AREA {} | EP {} -> Accept Playurl Cache Refresh Task...",
        params.area.to_ascii_uppercase(),
        params.ep_id
    );
    let background_task_data =
        BackgroundTaskType::CacheTask(CacheTask::PlayurlCacheRefresh(PlayurlParamsStatic {
            access_key: params.access_key.to_string(),
            appkey: params.appkey.to_string(),
            appsec: params.appsec.to_string(),
            ep_id: params.ep_id.to_string(),
            cid: params.cid.to_string(),
            season_id: params.season_id.to_string(),
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
    trace!(
        "[BACKGROUND TASK] AREA {} | EP {} -> Accept Area Cache Refresh Task...",
        params.area.to_ascii_uppercase(),
        params.ep_id
    );
    let background_task_data = BackgroundTaskType::CacheTask(CacheTask::EpAreaCacheRefresh(
        params.ep_id.to_owned(),
        params.access_key.to_owned(),
    ));
    bili_runtime.send_task(background_task_data).await
}

pub async fn update_cached_user_info_background(
    access_key: String,
    bili_runtime: &BiliRuntime<'_>,
) {
    trace!(
        "[BACKGROUND TASK] AK {access_key} -> Accept UserInfo Cache Refresh Task..."
    );
    let background_task_data =
        BackgroundTaskType::CacheTask(CacheTask::UserInfoCacheRefresh(access_key));
    bili_runtime.send_task(background_task_data).await
}

// pub async fn update_cached_ep_vip_status_background(
//     force_update: bool,
//     bili_runtime: &BiliRuntime<'_>,
// ) {
//     let background_task_data =
//         BackgroundTaskType::CacheTask(CacheTask::EpInfoCacheRefresh(force_update, ep_info_vec));
//     bili_runtime.send_task(background_task_data).await
// }

pub async fn background_task_run(
    task: BackgroundTaskType,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<(), String> {
    let config = bili_runtime.config;
    let redis_pool = bili_runtime.redis_pool;
    let report_config = &bili_runtime.config.report_config;
    match task {
        BackgroundTaskType::HealthTask(value) => match value {
            HealthTask::HealthCheck => {
                // 因为设置里面代理都是分开设置的, 感觉超麻烦的欸, 只检查playurl算了
                for area_num in [1 as u8, 2, 3, 4] {
                    if !check_proxy_health(
                        area_num,
                        ReqType::Playurl(Area::new(area_num), true),
                        bili_runtime,
                    )
                    .await
                    {
                        let redis_key = {
                            let area_num_vec = ["", "01", "02", "03", "04"];
                            let area_num_str = area_num_vec[area_num as usize];
                            ["01", area_num_str, "13", "01"].concat()
                        };
                        let num = bili_runtime
                            .redis_get(&redis_key)
                            .await
                            .unwrap_or("0".to_string())
                            .as_str()
                            .parse::<u32>()
                            .unwrap();
                        if num == 4 {
                            bili_runtime.redis_set(&redis_key, "1", 0).await
                        } else {
                            bili_runtime
                                .redis_set(&redis_key, &(num + 1).to_string(), 0)
                                .await
                        }
                    };
                }
                Ok(())
            }
            HealthTask::HealthReport(value) => {
                if config.report_open {
                    let area_num_vec = ["", "01", "02", "03", "04"];
                    let area_num;
                    let redis_key = match &value {
                        HealthReportType::Playurl(value) => {
                            area_num = area_num_vec[value.area_num as usize];
                            ["01", area_num, "13", "01"].concat()
                        }
                        HealthReportType::Search(value) => {
                            area_num = area_num_vec[value.area_num as usize];
                            ["02", area_num, "13", "01"].concat()
                        }
                        HealthReportType::ThSeason(value) => {
                            area_num = area_num_vec[value.area_num as usize];
                            ["04", area_num, "13", "01"].concat()
                        }
                        HealthReportType::Others(_) => {
                            send_report(redis_pool, report_config, &value)
                                .await
                                .unwrap();
                            return Ok(());
                        }
                    };
                    debug!("[BACKGROUND TASK] HealthReport redis_key: {redis_key}");
                    let is_available = value.is_available();
                    if is_available {
                        match bili_runtime.redis_get(&redis_key).await {
                            Some(value) => {
                                let err_num = value.parse::<u16>().unwrap_or(4);
                                if err_num >= 4 {
                                    bili_runtime.redis_set(&redis_key, "0", 0).await
                                } else if err_num != 0 {
                                    bili_runtime.redis_set(&redis_key, "0", 0).await
                                } else {
                                    return Ok(());
                                }
                            }
                            None => bili_runtime.redis_set(&redis_key, "0", 0).await,
                        }
                    } else {
                        let num = bili_runtime
                            .redis_get(&redis_key)
                            .await
                            .unwrap_or("0".to_string())
                            .as_str()
                            .parse::<u32>()
                            .unwrap();
                        if num == 4 {
                            let area_num = area_num.parse::<u8>().unwrap_or(2);
                            let req_type = match &value {
                                HealthReportType::Playurl(_) => {
                                    ReqType::Playurl(Area::new(area_num), true)
                                }
                                HealthReportType::Search(_) => {
                                    ReqType::Search(Area::new(area_num), true)
                                }
                                HealthReportType::ThSeason(_) => ReqType::ThSeason,
                                HealthReportType::Others(value) => ReqType::Other(
                                    value.upstream_reply.proxy_open,
                                    value.upstream_reply.proxy_url.clone(),
                                ),
                            };
                            // 超过四次请求失败即检测
                            if check_proxy_health(area_num, req_type, bili_runtime).await {
                                bili_runtime.redis_set(&redis_key, "0", 0).await;
                                return Ok(());
                            } else {
                                bili_runtime.redis_set(&redis_key, "1", 0).await
                            }
                        } else {
                            bili_runtime
                                .redis_set(&redis_key, &(num + 1).to_string(), 0)
                                .await
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
                let appkey = "1d8b6e7d45233436";
                let appsec = "560c52ccd288fed045859ed18bffd973";
                let user_agent =
                    "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
                match get_user_info(&access_key, appkey, appsec, user_agent, true, &bili_runtime).await {
                    Ok(new_user_info) => match get_blacklist_info(&new_user_info, bili_runtime).await {
                        Ok(_) => Ok(()),
                        Err(value) => Err(format!("[Background Task] UID {} | Refreshing blacklist info failed, ErrMsg: {}", new_user_info.uid, value.to_string())),
                    },
                    Err(value) => Err(format!("[Background Task] ACCESS_KEY {} | Refreshing blacklist info failed, ErrMsg: {}", access_key, value.to_string())),
                }
            }
            CacheTask::PlayurlCacheRefresh(params) => {
                match get_upstream_bili_playurl_background(&params, bili_runtime).await {
                    Ok(body_data) => {
                        update_cached_playurl(&params.as_ref(), &body_data, bili_runtime).await;
                        Ok(())
                    }
                    Err(value) => Err(format!(
                        "[Background Task] | Playurl cache refresh failed, ErrMsg: {value}"
                    )),
                }
            }
            CacheTask::EpInfoCacheRefresh(_force_update, _ep_info_vec) => {
                // let _new_ep_info_vec = if force_update {
                //     let ep_id = ep_info_vec[0].ep_id;
                //     if let Ok((_, value)) =
                //         get_upstream_bili_ep_info(&format!("{ep_id}"), false, "").await
                //     {
                //         value
                //     } else {
                //         return Err("[Background Task] ep info cache refresh failed".to_string());
                //     }
                // } else {
                //     ep_info_vec
                // };
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
                unimplemented!()
            }
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
                let _ = bili_runtime.redis_set(&key, &ep_area_data, 0).await;
                Ok(())
            }
        },
    }
}
