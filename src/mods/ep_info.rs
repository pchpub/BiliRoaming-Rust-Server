use super::types::{BiliRuntime, CacheType};
use super::upstream_res::get_upstream_bili_ep_info;
use log::debug;

/** `update_ep_vip_status_cache`
 * 目前仅用于记录EP是否为大会员专享
*/
pub async fn update_ep_vip_status_cache(
    ep_id: &str,
    need_vip: bool,
    bili_runtime: &BiliRuntime<'_>,
) {
    debug!("[UPDATE_CACHE] EP {ep_id} -> EP needVIP: {need_vip}");
    bili_runtime
        .update_cache(
            &CacheType::EpVipInfo(ep_id),
            &(need_vip as u8).to_string(),
            0,
        )
        .await;
}

pub async fn get_ep_need_vip(ep_id: &str, bili_runtime: &BiliRuntime<'_>) -> Option<u8> {
    let cache_type = CacheType::EpVipInfo(ep_id);
    match bili_runtime.get_cache(&cache_type).await {
        Some(value) => {
            let need_vip = value.parse::<u8>().unwrap_or(1);
            debug!("[GET_CACHE] EP {ep_id} -> EP needVIP: {value}");
            Some(need_vip)
        }
        None => {
            debug!("[GET_CACHE] EP {ep_id} -> No cached EP needVIP data");
            match get_upstream_bili_ep_info(ep_id, false, "").await {
                Ok((value, _)) => {
                    debug!(
                        "[GET_CACHE][U] EP {ep_id} -> EP needVIP: {}",
                        value.need_vip
                    );
                    update_ep_vip_status_cache(ep_id, value.need_vip, bili_runtime).await;
                    Some(value.need_vip as u8)
                }
                Err(_) => None,
            }
        }
    }
}
