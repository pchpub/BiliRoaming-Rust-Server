use deadpool_redis::Pool;

use super::{
    request::{getwebpage, postwebpage},
    tools::health_key_to_char,
    types::{ReportConfig, SendHealthData},
};

pub async fn send_report(
    redis: &Pool,
    config: &mut ReportConfig,
    health_data: &SendHealthData,
) -> Result<(), ()> {
    let health_cn_playurl = health_key_to_char(redis, "0111301").await;
    let health_hk_playurl = health_key_to_char(redis, "0121301").await;
    let health_tw_playurl = health_key_to_char(redis, "0131301").await;
    let health_th_playurl = health_key_to_char(redis, "0141301").await;
    let health_cn_search = health_key_to_char(redis, "0211301").await;
    let health_hk_search = health_key_to_char(redis, "0221301").await;
    let health_tw_search = health_key_to_char(redis, "0231301").await;
    let health_th_search = health_key_to_char(redis, "0241301").await;
    let health_th_season = health_key_to_char(redis, "0441301").await;

    match config.method {
        super::types::Method::Get => {
            let url = config
                .build_url(
                    &health_cn_playurl,
                    &health_hk_playurl,
                    &health_tw_playurl,
                    &health_th_playurl,
                    &health_cn_search,
                    &health_hk_search,
                    &health_tw_search,
                    &health_th_search,
                    &health_th_season,
                    &health_data.area_name(),
                    &health_data.data_type.to_string(),
                    &health_data.health_type.to_color_char(),
                )
                .unwrap();
            match getwebpage(
                url,
                false,
                "".to_string(),
                "BiliRoaming-Rust-Server".to_string(),
                "".to_string(),
            ) {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(());
                }
            }
        }
        super::types::Method::Post => {
            let url = config
                .build_url(
                    &health_cn_playurl,
                    &health_hk_playurl,
                    &health_tw_playurl,
                    &health_th_playurl,
                    &health_cn_search,
                    &health_hk_search,
                    &health_tw_search,
                    &health_th_search,
                    &health_th_season,
                    &health_data.area_name(),
                    &health_data.data_type.to_string(),
                    &health_data.health_type.to_color_char(),
                )
                .unwrap();
            let content = config
                .build_content(
                    &health_cn_playurl,
                    &health_hk_playurl,
                    &health_tw_playurl,
                    &health_th_playurl,
                    &health_cn_search,
                    &health_hk_search,
                    &health_tw_search,
                    &health_th_search,
                    &health_th_season,
                    &health_data.area_name(),
                    &health_data.data_type.to_string(),
                    &health_data.health_type.to_color_char(),
                )
                .unwrap();
            // println!("[Debug] content:{}", content);
            match postwebpage(
                url,
                content,
                false,
                "".to_string(),
                "BiliRoaming-Rust-Server".to_string(),
            ) {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(());
                }
            }
        }
    }
}
