use super::background_tasks::update_cached_ep_vip_status_background;
use super::request::async_getwebpage;
use super::types::{Area, BiliRuntime, CacheType, ReqType};
use super::upstream_res::get_upstream_bili_ep_info;
use log::{debug, error};
use serde_json::json;

/*
* `update_ep_vip_status_cache`
*/
pub async fn update_ep_vip_status_cache(
    ep_id: &str,
    need_vip: bool,
    bili_runtime: &BiliRuntime<'_>,
) {
    bili_runtime
        .update_cache(
            &CacheType::EpVipInfo(ep_id),
            &(need_vip as u8).to_string(),
            0,
        )
        .await;
    debug!("[GET EP_INFO][C] EP {ep_id} -> EP needVIP: {need_vip} 写入缓存成功");
}

pub async fn get_ep_need_vip(ep_id: &str, bili_runtime: &BiliRuntime<'_>) -> Option<u8> {
    let cache_type = CacheType::EpVipInfo(ep_id);
    match bili_runtime.get_cache(&cache_type).await {
        Some(value) => {
            let need_vip = value.parse::<u8>().unwrap_or(1);
            debug!("[GET EP_INFO][C] EP {ep_id} -> EP needVIP: {need_vip} 获取缓存成功");
            Some(need_vip)
        }
        None => {
            match get_upstream_bili_ep_info(ep_id, false, "", bili_runtime).await {
                Ok((value, ep_info_vec)) => {
                    update_cached_ep_vip_status_background(false, ep_info_vec, bili_runtime).await;
                    // update_ep_vip_status_cache(ep_id, value.need_vip, bili_runtime).await;
                    Some(value.need_vip as u8)
                }
                Err(_) => None,
            }
        }
    }
}

pub async fn get_ep_area_limit(ep_id: &str, area: Area, bili_runtime: &BiliRuntime<'_>) -> bool {
    let data_raw = bili_runtime.get_cache(&CacheType::EpArea(ep_id)).await;
    let mut ep_area_data: [u8; 4] = [2, 2, 2, 2];
    if let Some(value) = data_raw {
        for (index, char) in value.char_indices() {
            match char {
                '0' => {
                    ep_area_data[index] = 0; //0表示正常
                }
                '1' => {
                    ep_area_data[index] = 1; //非0不正常
                }
                _ => (),
            }
        }
    };
    let area_num = area.num() as usize;
    if ep_area_data[area_num - 1] == 1 {
        false
    } else {
        let bili_user_status_api: &str = "https://api.bilibili.com/pgc/view/web/season/user/status";
        let user_agent = "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
        // 暂时只借用带会员的来检测
        let access_key = if let Some(value) = bili_runtime.redis_get("av11301").await {
            value
        } else {
            error!("[GET AREA_LIMIT] fail to get access_key");
            return false;
        };
        let config = bili_runtime.config;
        let req_type = ReqType::Playurl(area, true);
        let url = format!("{bili_user_status_api}?access_key={access_key}&ep_id={ep_id}");
        let (proxy_open, proxy_url) = req_type.get_proxy(config);
        match async_getwebpage(&url, proxy_open, proxy_url, user_agent, "", None).await {
            Ok(value) => {
                let json_result = value
                    .json()
                    .unwrap_or(json!({"code": -2333, "message": ""}));
                let code = json_result["code"].as_i64().unwrap_or(-2333);
                match code {
                    0 => true,
                    _ => false,
                }
            }
            Err(_) => false,
        }
    }
}
