use super::request::{async_getwebpage, async_postwebpage, redis_get};
use super::types::ReportHealthData;
use super::types::{ReportMethod, SendHealthData};
use deadpool_redis::Pool;
use urlencoding::encode;

pub async fn send_report(
    redis_pool: &Pool,
    report_method: &ReportMethod,
    health_data: &SendHealthData,
) -> Result<(), ()> {
    // println!("[DEBUG] TEST PUSH, {:#?}", report_method);
    let health_report_data = generate_health_report_data(redis_pool).await;
    match report_method {
        // telegram bot
        ReportMethod::ReportConfigTgBot(config) => {
            let url = format!(
                "https://api.telegram.org/bot{}/sendMessage",
                config.tg_bot_token
            );
            let content = format!(
                "chat_id={}&text={}",
                config.tg_chat_id,
                health_report_data.generate_msg(report_method, health_data)
            );
            match async_postwebpage(
                &url,
                &content,
                &config.tg_proxy_api_open,
                &config.tg_proxy_url,
                "BiliRoaming-Rust-Server",
            ) // 虽然但是, 似乎也可以get, encode一下就可, 懒得改了
            .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(());
                }
            }
        }
        // pushplus
        ReportMethod::ReportConfigPushplus(config) => {
            let url = format!(
                "https://www.pushplus.plus/send/{}?topic={}&title={}&content={}&template=html",
                config.pushplus_secret.clone(),
                config.pushplus_group_id.clone(),
                encode(&config.pushplus_push_title),
                encode(&health_report_data.generate_msg(report_method, health_data))
            );
            // must encode params before getwebpage
            match async_getwebpage(&url, &false, "", "BiliRoaming-Rust-Server", "").await {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(());
                }
            }
        }
        // 自定义
        ReportMethod::ReportConfigCustom(config) => {
            match config.method {
                super::types::Method::Get => {
                    let url = config
                        .build_url(
                            &health_report_data.health_cn_playurl,
                            &health_report_data.health_hk_playurl,
                            &health_report_data.health_tw_playurl,
                            &health_report_data.health_th_playurl,
                            &health_report_data.health_cn_search,
                            &health_report_data.health_hk_search,
                            &health_report_data.health_tw_search,
                            &health_report_data.health_th_search,
                            &health_report_data.health_th_season,
                            &health_data.area_name(),
                            &health_data.data_type.to_string(),
                            &health_data.health_type.to_color_char(),
                        )
                        .unwrap();
                    match async_getwebpage(&url, &false, "", "BiliRoaming-Rust-Server", "").await {
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
                            &health_report_data.health_cn_playurl,
                            &health_report_data.health_hk_playurl,
                            &health_report_data.health_tw_playurl,
                            &health_report_data.health_th_playurl,
                            &health_report_data.health_cn_search,
                            &health_report_data.health_hk_search,
                            &health_report_data.health_tw_search,
                            &health_report_data.health_th_search,
                            &health_report_data.health_th_season,
                            &health_data.area_name(),
                            &health_data.data_type.to_string(),
                            &health_data.health_type.to_color_char(),
                        )
                        .unwrap();
                    let content = config
                        .build_content(
                            &health_report_data.health_cn_playurl,
                            &health_report_data.health_hk_playurl,
                            &health_report_data.health_tw_playurl,
                            &health_report_data.health_th_playurl,
                            &health_report_data.health_cn_search,
                            &health_report_data.health_hk_search,
                            &health_report_data.health_tw_search,
                            &health_report_data.health_th_search,
                            &health_report_data.health_th_season,
                            &health_data.area_name(),
                            &health_data.data_type.to_string(),
                            &health_data.health_type.to_color_char(),
                        )
                        .unwrap();
                    // println!("[Debug] content:{}", content);
                    match async_postwebpage(&url, &content, &false, "", "BiliRoaming-Rust-Server")
                        .await
                    {
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
        ReportMethod::ReportConfigNone => return Ok(()),
    }
}

async fn health_key_to_char(key: &str) -> String {
    match key {
        "0" => "🟢".to_string(),
        "1" => "🟡".to_string(),
        "2" => "🟠".to_string(),
        "3" => "🟠".to_string(),
        "4" => "🔴".to_string(),
        _ => "🔴".to_string(),
    }
}

async fn generate_health_report_data(redis_pool: &Pool) -> ReportHealthData {
    let health_cn_playurl = redis_get(redis_pool, "0111301")
        .await
        .unwrap_or("4".to_string());
    let health_hk_playurl = redis_get(redis_pool, "0121301")
        .await
        .unwrap_or("4".to_string());
    let health_tw_playurl = redis_get(redis_pool, "0131301")
        .await
        .unwrap_or("4".to_string());
    let health_th_playurl = redis_get(redis_pool, "0141301")
        .await
        .unwrap_or("4".to_string());
    let health_cn_search = redis_get(redis_pool, "0211301")
        .await
        .unwrap_or("4".to_string());
    let health_hk_search = redis_get(redis_pool, "0221301")
        .await
        .unwrap_or("4".to_string());
    let health_tw_search = redis_get(redis_pool, "0231301")
        .await
        .unwrap_or("4".to_string());
    let health_th_search = redis_get(redis_pool, "0241301")
        .await
        .unwrap_or("4".to_string());
    let health_th_season = redis_get(redis_pool, "0441301")
        .await
        .unwrap_or("4".to_string());
    ReportHealthData {
        health_cn_playurl: health_key_to_char(&health_cn_playurl).await,
        health_hk_playurl: health_key_to_char(&health_hk_playurl).await,
        health_tw_playurl: health_key_to_char(&health_tw_playurl).await,
        health_th_playurl: health_key_to_char(&health_th_playurl).await,
        health_cn_search: health_key_to_char(&health_cn_search).await,
        health_hk_search: health_key_to_char(&health_hk_search).await,
        health_tw_search: health_key_to_char(&health_tw_search).await,
        health_th_search: health_key_to_char(&health_th_search).await,
        health_th_season: health_key_to_char(&health_th_season).await,
    }
}
