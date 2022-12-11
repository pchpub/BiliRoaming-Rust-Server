use super::cache::update_cached_playurl;
use super::ep_info::update_ep_vip_status_cache;
use super::health::*;
use super::push::send_report;
use super::request::async_getwebpage;
use super::tools::build_random_useragent;
use super::types::{
    Area, BackgroundTaskType, BiliRuntime, CacheTask, CacheType, EpInfo, HealthReportType,
    HealthTask, PlayurlParams, PlayurlParamsStatic, ReqType,
};
use super::upstream_res::*;
use super::user_info::{get_blacklist_info, get_user_info};
use log::{debug, error, info, trace};
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
    // 虽然看起来很114514, 懒得改了, 能用就行
    let background_task_data =
        BackgroundTaskType::Cache(CacheTask::PlayurlCacheRefresh(PlayurlParamsStatic {
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
            ep_need_vip: params.ep_need_vip,
            area: params.area.to_string(),
            area_num: params.area_num,
            user_agent: params.user_agent.to_string(),
        }));
    bili_runtime.send_task(background_task_data).await
}

pub async fn update_cached_area_background(
    params: &PlayurlParams<'_>,
    bili_runtime: &BiliRuntime<'_>,
) {
    trace!(
        "[BACKGROUND TASK] AREA {} | EP {} -> Accept Area Cache Refresh Task...",
        params.area.to_ascii_uppercase(),
        params.ep_id
    );
    let background_task_data = BackgroundTaskType::Cache(CacheTask::EpAreaCacheRefresh(
        params.ep_id.to_owned(),
        params.access_key.to_owned(),
    ));
    bili_runtime.send_task(background_task_data).await
}

pub async fn update_cached_user_info_background(
    access_key: String,
    bili_runtime: &BiliRuntime<'_>,
) {
    trace!("[BACKGROUND TASK] AK {access_key} -> Accept UserInfo Cache Refresh Task...");
    let background_task_data =
        BackgroundTaskType::Cache(CacheTask::UserInfoCacheRefresh(access_key));
    bili_runtime.send_task(background_task_data).await
}

pub async fn update_cached_ep_vip_status_background(
    force_update: bool,
    ep_info_vec: Vec<EpInfo>,
    bili_runtime: &BiliRuntime<'_>,
) {
    let background_task_data =
        BackgroundTaskType::Cache(CacheTask::EpInfoCacheRefresh(force_update, ep_info_vec));
    bili_runtime.send_task(background_task_data).await
}

pub async fn background_task_run(
    task: BackgroundTaskType,
    bili_runtime: &BiliRuntime<'_>,
) -> Result<(), String> {
    let config = bili_runtime.config;
    let redis_pool = bili_runtime.redis_pool;
    let report_config = &bili_runtime.config.report_config;
    match task {
        BackgroundTaskType::Health(value) => match value {
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
                    let max_num_of_err = 2u16;
                    let area_num_vec = ["", "1", "2", "3", "4"];
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
                        // 其他类型留给自定义推送
                        HealthReportType::Others(_) => {
                            send_report(redis_pool, report_config, &value)
                                .await
                                .unwrap();
                            return Ok(());
                        }
                    };
                    // debug!("[BACKGROUND TASK] HealthReport redis_key: {redis_key}");
                    let is_available = value.is_available();
                    if is_available {
                        match bili_runtime.redis_get(&redis_key).await {
                            Some(health_data) => {
                                let err_num = health_data.parse::<u16>().unwrap_or(max_num_of_err);
                                bili_runtime.redis_set(&redis_key, "0", 0).await;
                                if err_num >= max_num_of_err {
                                    send_report(&redis_pool, &report_config, &value)
                                        .await
                                        .unwrap_or_default();
                                }
                            }
                            None => {
                                bili_runtime.redis_set(&redis_key, "0", 0).await;
                                send_report(&redis_pool, &report_config, &value)
                                    .await
                                    .unwrap_or_default();
                            }
                        }
                    } else {
                        let num_of_err = bili_runtime
                            .redis_get(&redis_key)
                            .await
                            .unwrap_or("0".to_string())
                            .as_str()
                            .parse::<u16>()
                            .unwrap_or(max_num_of_err);
                        if num_of_err >= max_num_of_err {
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
                            // 超过max_num_of_err次请求失败即检测
                            if check_proxy_health(area_num, req_type, bili_runtime).await {
                                bili_runtime.redis_set(&redis_key, "0", 0).await;
                                return Ok(());
                            } else {
                                bili_runtime.redis_set(&redis_key, "1", 0).await;
                                send_report(&redis_pool, &report_config, &value)
                                    .await
                                    .unwrap_or_default();
                            }
                        } else {
                            bili_runtime
                                .redis_set(&redis_key, &(num_of_err + 1).to_string(), 0)
                                .await
                        }
                    }
                };
                Ok(())
            }
        },
        BackgroundTaskType::Cache(value) => match value {
            CacheTask::UserInfoCacheRefresh(access_key) => {
                let appkey = "1d8b6e7d45233436";
                let appsec = "560c52ccd288fed045859ed18bffd973";
                // let user_agent = "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
                let user_agent = build_random_useragent();
                match get_user_info(&access_key, appkey, appsec, &PlayurlParams {
                    is_app: true,
                    is_th: false,
                    user_agent: &user_agent,
                    ..Default::default()
                }, true, &bili_runtime).await {
                    Ok(new_user_info) => match get_blacklist_info(&new_user_info, bili_runtime).await {
                        Ok(_) => Ok(()),
                        Err(value) => Err(format!("[BACKGROUND TASK] UID {} | Refreshing blacklist info failed, ErrMsg: {}", new_user_info.uid, value.to_string())),
                    },
                    Err(value) => Err(format!("[BACKGROUND TASK] ACCESS_KEY {} | Refreshing blacklist info failed, ErrMsg: {}", access_key, value.to_string())),
                }
            }
            CacheTask::PlayurlCacheRefresh(params) => {
                match get_upstream_bili_playurl_background(&mut params.as_ref(), bili_runtime).await
                {
                    Ok(body_data) => {
                        update_cached_playurl(&mut params.as_ref(), &body_data, bili_runtime).await;
                        Ok(())
                    }
                    Err(value) => Err(format!(
                        "[BACKGROUND TASK] | Playurl cache refresh failed, ErrMsg: {}",
                        value.to_string()
                    )),
                }
            }
            CacheTask::EpInfoCacheRefresh(force_update, ep_info_vec) => {
                let new_ep_info_vec = if force_update {
                    // 可能有限免的, 一旦如此则强制清除之
                    let ep_id = ep_info_vec[0].ep_id;
                    if let Ok((_, value)) =
                        get_upstream_bili_ep_info(&format!("{ep_id}"), false, "", bili_runtime)
                            .await
                    {
                        value
                    } else {
                        return Err(
                            "[BACKGROUND TASK] EpInfo cache force refresh failed".to_string()
                        );
                    }
                } else {
                    ep_info_vec
                };
                for ep_info in new_ep_info_vec {
                    update_ep_vip_status_cache(
                        &ep_info.ep_id.to_string(),
                        ep_info.need_vip,
                        bili_runtime,
                    )
                    .await;
                }
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
                let user_agent = build_random_useragent();
                // let user_agent = "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
                let area_to_check = [
                    (
                        format!("{bili_user_status_api}?access_key={access_key}&ep_id={ep_id}"),
                        config.cn_proxy_playurl_open,
                        &config.cn_proxy_playurl_url,
                    ),
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
                ];
                let mut area_num: u8 = 0;
                let mut ep_area_data: [&str; 4] = ["2", "2", "2", "2"];
                for item in area_to_check {
                    area_num += 1;
                    let (url, proxy_open, proxy_url) = item;
                    match async_getwebpage(&url, proxy_open, proxy_url, &user_agent, "", None).await {
                            Ok(value) => {
                                let json_result = value.json()
                                    .unwrap_or(json!({"code": -2333, "message": ""}));
                                let code = json_result["code"]
                                    .as_i64()
                                    .unwrap_or(-2333);
                                match code {
                                    0 => {
                                        let result = json_result.get("result").unwrap();
                                        if result["area_limit"].as_i64().unwrap() != 0 {
                                            ep_area_data[(area_num - 1) as usize] = "1";
                                        } else {
                                            ep_area_data[3 as usize] = "1";
                                            ep_area_data[(area_num - 1) as usize] = "0";
                                        }
                                    }
                                    -404 => {
                                        // 东南亚区贼恶心...
                                        info!("[BACKGROUND TASK] EP {ep_id} | PROXY_OPEN {proxy_open} | PROXY_URL {proxy_url} -> Check EP available zone -404: maybe zone th");
                                        ep_area_data[(area_num - 1) as usize] = "1";
                                        ep_area_data[3 as usize] = "0";
                                    }
                                    -2333 => {
                                        error!("[BACKGROUND TASK] EP {ep_id} | PROXY_OPEN {proxy_open} | PROXY_URL {proxy_url} -> Check EP available zone failed: Json Parsing Error: {value}");
                                        continue;
                                    }
                                    _ => {
                                        ep_area_data[(area_num - 1) as usize] = "1";
                                        error!("[BACKGROUND TASK] EP {ep_id} | PROXY_OPEN {proxy_open} | PROXY_URL {proxy_url} -> Check EP available zone failed: Unknown Error Code {code}: {value}");
                                        continue;
                                    }
                                }
                            }
                            Err(_) => error!("[BACKGROUND TASK] EP {ep_id} | PROXY_OPEN {proxy_open} | PROXY_URL {proxy_url} -> Check EP available zone failed: Network Error"),
                        }
                }
                let ep_area_data = ep_area_data.concat();
                debug!("[BACKGROUND TASK] EP {ep_id} | Check EP available zone finished: {ep_area_data}");
                bili_runtime
                    .update_cache(&CacheType::EpArea(&ep_id), &ep_area_data, 0)
                    .await;
                Ok(())
            }
        },
    }
}
