use super::{
    request::{async_getwebpage, redis_get, redis_set},
    types::{Area, BiliConfig, HealthReportType, BackgroundTaskType, HealthTask},
};
use async_channel::{Sender, TrySendError};
use deadpool_redis::Pool;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

pub async fn report_health(
    incident_attr: HealthReportType,
    config: &BiliConfig,
    redis_pool: &Pool,
    bilisender: &Arc<Sender<BackgroundTaskType>>,
) {
    let (_area_num, redis_key) = match &incident_attr {
        HealthReportType::Playurl(value) => {
            let redis_key_vec: Vec<u8> = vec![01, value.area_num, 13, 01];
            (value.area_num, String::from_utf8(redis_key_vec).unwrap())
        }
        HealthReportType::Search(value) => {
            let redis_key_vec: Vec<u8> = vec![02, value.area_num, 13, 01];
            (value.area_num, String::from_utf8(redis_key_vec).unwrap())
        }
        HealthReportType::ThSeason(value) => {
            let redis_key_vec: Vec<u8> = vec![04, value.area_num, 13, 01];
            (value.area_num, String::from_utf8(redis_key_vec).unwrap())
        }
        HealthReportType::Others(_) => return,
    };
    let bilisender_cl = Arc::clone(bilisender);
    if config.report_open {
        match redis_get(&redis_pool, &redis_key).await {
            Some(value) => {
                let err_num = value.parse::<u16>().unwrap();
                if err_num >= 4 {
                    redis_set(&redis_pool, &redis_key, "0", 0)
                        .await
                        .unwrap_or_default();
                    let data_to_send = BackgroundTaskType::HealthTask(HealthTask::HealthReport(incident_attr));
                    tokio::spawn(async move {
                        //debug!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                        match bilisender_cl.try_send(data_to_send) {
                            Ok(_) => (),
                            Err(TrySendError::Full(_)) => {
                                println!("[PUSH] channel is full");
                            }
                            Err(TrySendError::Closed(_)) => {
                                println!("[PUSH] channel is closed");
                            }
                        };
                    });
                } else if err_num != 0 {
                    redis_set(&redis_pool, &redis_key, "0", 0)
                        .await
                        .unwrap_or_default();
                }
            }
            None => {
                redis_set(&redis_pool, &redis_key, "0", 0)
                    .await
                    .unwrap_or_default();
                let data_to_send = BackgroundTaskType::HealthTask(HealthTask::HealthReport(incident_attr));
                tokio::spawn(async move {
                    //debug!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
                    match bilisender_cl.try_send(data_to_send) {
                        Ok(_) => (),
                        Err(TrySendError::Full(_)) => {
                            println!("[PUSH] channel is full");
                        }
                        Err(TrySendError::Closed(_)) => {
                            println!("[PUSH] channel is closed");
                        }
                    };
                });
            }
        }
    }
}

/*
* 主动检测上游代理状态
* now only check playurl proxy(I think it's enough)
*/
pub async fn check_proxy_health(
    area_num: u8,
    redis_pool: &Pool,
    config: &BiliConfig,
) -> Result<(), String> {
    let bili_user_status_api: &str = "https://api.bilibili.com/pgc/view/web/season/user/status";
    let season_id_cn_only = "42320	"; // 小林家的龙女仆 第二季 中配版
    let season_id_hk_only = "41550"; // "輝夜姬想讓人告白ー超級浪漫ー（僅限港澳地區）
    let season_id_tw_only = "33088"; // 輝夜姬想讓人告白？~天才們的戀愛頭腦戰~（僅限台灣地區）
    let _season_th_only = ""; // 东南亚区没有api, 无从直接得知area_limit状态
    let user_agent = "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
    let access_key = if let Some(value) = redis_get(redis_pool, "a1301").await {
        value
    } else {
        return Err("[ERROR] fail to get access_key".to_string());
    };
    // actually should always use struct Area to pass param area
    let area: Area = Area::new(area_num);
    // let area_num = area.num();
    let (url, proxy_open, proxy_url) = match area {
        Area::Cn => (
            format!("{bili_user_status_api}?access_key={access_key}&season_id={season_id_cn_only}"),
            &config.cn_proxy_playurl_open,
            &config.cn_proxy_playurl_url,
        ),

        Area::Hk => (
            format!("{bili_user_status_api}?access_key={access_key}&season_id={season_id_hk_only}"),
            &config.hk_proxy_playurl_open,
            &config.hk_proxy_playurl_url,
        ),

        Area::Tw => (
            format!("{bili_user_status_api}?access_key={access_key}&season_id={season_id_tw_only}"),
            &config.tw_proxy_playurl_open,
            &config.tw_proxy_playurl_url,
        ),
        Area::Th => {
            match get_server_ip_area(
                &config.tw_proxy_playurl_open,
                &config.tw_proxy_playurl_url,
                user_agent,
            )
            .await
            {
                Ok(value) => {
                    if value == 4 {
                        return Ok(());
                    } else {
                        return Err(format!(
                            "Zone {area_num} -> Detect Proxy Area Not Suitable, actual [{value}]"
                        ));
                    }
                }
                Err(code) => {
                    return Err(format!("Zone {area_num} -> Unknown Upstream Error {code}"))
                }
            }
        }
    };
    match async_getwebpage(&url, proxy_open, proxy_url, user_agent, "").await {
        Ok(value) => {
            let json_result =
                serde_json::from_str(&value).unwrap_or(json!({"code": -2333, "message": ""}));
            let code = json_result["code"]
                .as_str()
                .unwrap_or("233")
                .parse::<i64>()
                .unwrap_or(233);
            match code {
                0 => {
                    let result = json_result.get("result").unwrap();
                    if result["area_limit"].as_i64().unwrap() != 0 {
                        Err(format!("Zone {area_num} -> Detect Proxy Area Not Suitable"))
                    } else {
                        Ok(())
                    }
                }
                -2333 => Err(format!("Zone {area_num} -> Parse Json Error: {value}")),
                _ => Err(format!("Zone {area_num} -> Unknown Error {code}: {value}")),
            }
        }
        Err(_) => Err(format!("Zone {area_num} -> Detect Unavailable Proxy")),
    }
}

/*
* 地区检测
*/
pub async fn get_server_ip_area(
    proxy_open: &bool,
    proxy_url: &str,
    user_agent: &str,
) -> Result<u8, i64> {
    let area_api = "https://api.bilibili.com/x/web-interface/zone";
    let country_code_vec: Vec<(u16, u8)> = vec![
        (86, 1),  // 86 => 中国大陆
        (852, 2), // 852 => 香港特别行政区
        (886, 3), // 886 => 台湾地区
        (60, 4),  // 60 => 马来西亚
        (62, 4),  // 62 => 印度尼西亚
        (63, 4),  // 63 => 菲律宾
        (65, 4),  // 65 => 新加坡
        (66, 4),  // 66 => 泰国
        (4, 4),   // 84 => 越南
        (95, 4),  // 95 => 缅甸
        (673, 4), // 673 => 文莱
        (855, 4), // 855 => 柬埔寨
        (856, 4), // 856 => 老挝
    ];
    let country_code_map: HashMap<u16, u8> = country_code_vec.into_iter().collect();
    match async_getwebpage(area_api, proxy_open, proxy_url, user_agent, "").await {
        Ok(value) => {
            let json_result =
                serde_json::from_str(&value).unwrap_or(json!({"code": -2333, "message": ""}));
            let code = json_result["code"]
                .as_str()
                .unwrap_or("233")
                .parse::<i64>()
                .unwrap_or(233);
            match code {
                0 => {
                    let result = json_result.get("data").unwrap();
                    let country_code = result["country_code"].as_u64().unwrap_or(0) as u16;
                    Ok(*country_code_map.get(&country_code).unwrap_or(&(0 as u8)))
                }
                _ => Err(code),
            }
        }
        Err(_) => Err(-2333),
    }
}
// pub fn generate_health_page() {
//     // TODO: 生成状态页, 后续联动web panel
//     todo!()
// }


