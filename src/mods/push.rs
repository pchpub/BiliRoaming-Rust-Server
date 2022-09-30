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
            println!("[Debug] url:{}", url);
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
            println!("[Debug] url:{}", url);
            println!("[Debug] content:{}", content);
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
    // let msg = format!(
    //                     "大陆 Playurl:              {}\n香港 Playurl:              {}\n台湾 Playurl:              {}\n泰区 Playurl:              {}\n大陆 Search:              {}\n香港 Search:              {}\n台湾 Search:              {}\n泰区 Search:              {}\n泰区 Season:              {}\n\n变动: {} {} -> {}",
    //                     health_cn_playurl,
    //                     health_hk_playurl,
    //                     health_tw_playurl,
    //                     health_th_playurl,
    //                     health_cn_search,
    //                     health_hk_search,
    //                     health_tw_search,
    //                     health_th_search,
    //                     health_th_season,
    //                     health_data.area_name(),
    //                     health_data.data_type,
    //                     health_data.health_type.to_color_char()
    //                 );
    // let url = format!(
    //     "https://api.telegram.org/bot{}/sendMessage",
    //     &anti_speedtest_cfg.telegram_token
    // );
    // let content = format!(
    //     "chat_id={}&text={msg}",
    //     &anti_speedtest_cfg.telegram_chat_id
    // );
    // if let Err(_) =
    //     async_postwebpage(&url, &content, &false, "", "BiliRoaming-Rust-Server")
    //         .await
    // {
    //     println!("[Error] 发送监控状态失败");
    // };
}
