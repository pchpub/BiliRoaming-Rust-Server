use async_channel::Sender;
use deadpool_redis::Pool;
use std::sync::Arc;

use super::cache::update_cached_ep_info_background;
use super::request::redis_get;
use super::types::{BackgroundTaskType, BiliRuntime};
use super::upstream_res::get_upstream_bili_ep_info;

// pub async fn get_ep_info(ep_id: &str, redis_pool: &Pool) -> Option<EpInfo> {
//     match get_cached_ep_info(ep_id, redis_pool).await {
//         Ok(value) => Some(value),
//         Err(_) => match get_upstream_bili_ep_info(ep_id, false, "").await {
//             Ok((value, _)) => Some(value),
//             Err(_) => todo!(),
//         },
//     }
// }

pub async fn get_ep_need_vip(
    ep_id: &str,
    bili_runtime: &BiliRuntime<'_>
) -> Option<bool> {
    let key = format!("e{ep_id}150101");
    // data stucture: {ep_id},{0},{title},{season_id}
    match bili_runtime.redis_get(&key).await {
        Some(value) => {
            let need_vip = value.parse::<u8>().unwrap_or(1);
            Some(need_vip == 1)
        }
        None => {
            // println!("[EP INFO] EP {ep_id} | No cached data");
            match get_upstream_bili_ep_info(ep_id, false, "").await {
                Ok((value, ep_info_vec)) => {
                    update_cached_ep_info_background(false, ep_info_vec, bili_runtime).await;
                    Some(value.need_vip)
                }
                Err(_) => {
                    // if error then try to force update cache
                    update_cached_ep_info_background(true, vec![], bili_runtime).await;
                    None
                }
            }
        }
    }
}

pub async fn get_ep_need_vip_background(
    ep_id: &str,
    redis_pool: &Pool,
) -> Option<bool> {
    let key = format!("e{ep_id}150101");
    // data stucture: {ep_id},{0},{title},{season_id}
    match redis_get(redis_pool, &key).await {
        Some(value) => {
            let need_vip = value.parse::<u8>().unwrap_or(1);
            Some(need_vip == 1)
        }
        None => {
            // println!("[EP INFO] EP {ep_id} | No cached data");
            match get_upstream_bili_ep_info(ep_id, false, "").await {
                Ok((value, ep_info_vec)) => {
                    Some(value.need_vip)
                }
                Err(_) => {
                    // if error then try to force update cache
                    None
                }
            }
        }
    }
}