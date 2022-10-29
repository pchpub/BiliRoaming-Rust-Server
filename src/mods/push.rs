use super::request::{async_getwebpage, async_postwebpage, redis_get};
use super::types::HealthReportType;
use super::types::{ReportConfig, ReportHealthData};
use deadpool_redis::Pool;
use urlencoding::encode;

pub async fn send_report(
    redis_pool: &Pool,
    report_config: &ReportConfig,
    health_report_type: &HealthReportType,
) -> Result<(), String> {
    // println!("[DEBUG] TEST PUSH, {:#?}", report_method);
    let report_health_data = generate_health_report_data(redis_pool).await;
    let (area_name, data_type) = health_report_type.incident_attr();
    let color_char = health_report_type.status_color_char();
    match report_config {
        ReportConfig::TgBot(report_config_tg_bot) => {
            let url = format!(
                "https://api.telegram.org/bot{}/sendMessage",
                report_config_tg_bot.tg_bot_token
            );
            let content = format!(
                "chat_id={}&text={}",
                report_config_tg_bot.tg_chat_id,
                report_health_data.generate_msg(report_config, health_report_type)
            );
            match async_postwebpage(
                &url,
                &content,
                report_config_tg_bot.tg_proxy_api_open,
                &report_config_tg_bot.tg_proxy_url,
                "BiliRoaming-Rust-Server",
            )
            .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(String::new());
                }
            }
        }
        ReportConfig::PushPlus(report_config_push_plus) => {
            let url = format!(
                "https://www.pushplus.plus/send/{}?topic={}&title={}&content={}&template=html",
                report_config_push_plus.pushplus_secret.clone(),
                report_config_push_plus.pushplus_group_id.clone(),
                encode(&report_config_push_plus.pushplus_push_title),
                encode(&report_health_data.generate_msg(report_config, health_report_type))
            );
            // must encode params before getwebpage
            match async_getwebpage(&url, false, "", "BiliRoaming-Rust-Server", "").await {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(String::new());
                }
            }
        }
        ReportConfig::Custom(report_config_custom) => {
            match report_config_custom.method {
                super::types::ReportRequestMethod::Get => {
                    let url = report_config_custom
                        .build_url(&report_health_data, &area_name, &data_type, &color_char)
                        .unwrap();
                    match async_getwebpage(&url, false, "", "BiliRoaming-Rust-Server", "").await {
                        Ok(_) => {
                            return Ok(());
                        }
                        Err(_) => {
                            return Err(String::new());
                        }
                    }
                }
                super::types::ReportRequestMethod::Post => {
                    let url = report_config_custom
                        .build_url(&report_health_data, &area_name, &data_type, &color_char)
                        .unwrap();
                    let content = report_config_custom
                        .build_url(&report_health_data, &area_name, &data_type, &color_char)
                        .unwrap();
                    // println!("[Debug] content:{}", content);
                    match async_postwebpage(&url, &content, false, "", "BiliRoaming-Rust-Server")
                        .await
                    {
                        Ok(_) => {
                            return Ok(());
                        }
                        Err(_) => {
                            return Err(String::new());
                        }
                    }
                }
            }
        }
    }
}

async fn health_key_to_char(key: &str) -> String {
    match key {
        "0" => "🟢".to_owned(),
        "1" => "🟡".to_owned(),
        "2" => "🟠".to_owned(),
        "3" => "🟠".to_owned(),
        "4" => "🔴".to_owned(),
        _ => "🔴".to_owned(),
    }
}

async fn generate_health_report_data(redis_pool: &Pool) -> ReportHealthData {
    let health_cn_playurl_string = redis_get(redis_pool, "0111301")
        .await
        .unwrap_or("4".to_string());
    let health_hk_playurl_string = redis_get(redis_pool, "0121301")
        .await
        .unwrap_or("4".to_string());
    let health_tw_playurl_string = redis_get(redis_pool, "0131301")
        .await
        .unwrap_or("4".to_string());
    let health_th_playurl_string = redis_get(redis_pool, "0141301")
        .await
        .unwrap_or("4".to_string());
    let health_cn_search_string = redis_get(redis_pool, "0211301")
        .await
        .unwrap_or("4".to_string());
    let health_hk_search_string = redis_get(redis_pool, "0221301")
        .await
        .unwrap_or("4".to_string());
    let health_tw_search_string = redis_get(redis_pool, "0231301")
        .await
        .unwrap_or("4".to_string());
    let health_th_search_string = redis_get(redis_pool, "0241301")
        .await
        .unwrap_or("4".to_string());
    let health_th_season_string = redis_get(redis_pool, "0441301")
        .await
        .unwrap_or("4".to_string());
    ReportHealthData {
        health_cn_playurl: health_key_to_char(&health_cn_playurl_string).await,
        health_hk_playurl: health_key_to_char(&health_hk_playurl_string).await,
        health_tw_playurl: health_key_to_char(&health_tw_playurl_string).await,
        health_th_playurl: health_key_to_char(&health_th_playurl_string).await,
        health_cn_search: health_key_to_char(&health_cn_search_string).await,
        health_hk_search: health_key_to_char(&health_hk_search_string).await,
        health_tw_search: health_key_to_char(&health_tw_search_string).await,
        health_th_search: health_key_to_char(&health_th_search_string).await,
        health_th_season: health_key_to_char(&health_th_season_string).await,
    }
}
