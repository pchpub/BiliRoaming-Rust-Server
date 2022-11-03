use super::{
    request::async_getwebpage,
    types::{
        Area, BackgroundTaskType, BiliRuntime, HealthData, HealthReportType, HealthTask, ReqType,
        UpstreamReply,
    },
};
use serde_json::json;
use std::collections::HashMap;

/// `report_health` 后台任务: 报告健康信息
pub async fn report_health(health_report_type: HealthReportType, bili_runtime: &BiliRuntime<'_>) {
    let background_task_data =
        BackgroundTaskType::HealthTask(HealthTask::HealthReport(health_report_type));
    bili_runtime.send_task(background_task_data).await;
}

/// `check_health_background` 后台任务: 检测代理可用性
/// - 将检测所有区域的代理可用性
/// - 仅检测playurl的代理
pub async fn check_health_background(bili_runtime: &BiliRuntime<'_>) {
    let background_task_data = BackgroundTaskType::HealthTask(HealthTask::HealthCheck);
    bili_runtime.send_task(background_task_data).await;
}

/*
* 主动检测上游代理状态
* now only check playurl proxy(I think it's enough)
*/
pub async fn check_proxy_health(
    area_num: u8,
    req_type: ReqType,
    bili_runtime: &BiliRuntime<'_>,
) -> bool {
    let config = bili_runtime.config;
    let bili_user_status_api: &str = "https://api.bilibili.com/pgc/view/web/season/user/status";
    let season_id_cn_only = "42320	"; // 小林家的龙女仆 第二季 中配版
    let season_id_hk_only = "41550"; // "輝夜姬想讓人告白ー超級浪漫ー（僅限港澳地區）
    let season_id_tw_only = "33088"; // 輝夜姬想讓人告白？~天才們的戀愛頭腦戰~（僅限台灣地區）
    let _season_th_only = ""; // 东南亚区没有api, 无从直接得知area_limit状态
    let user_agent = "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011)";
    // 暂时只借用带会员的来检测
    let access_key = if let Some(value) = bili_runtime.redis_get("av11301").await {
        value
    } else {
        println!("[CHECK_PROXY_HEALTH] fail to get access_key");
        return true;
    };
    // actually should always use struct Area to pass param area
    // let area: Area = Area::new(area_num);
    // let area_num = area.num();
    // let req_type = ReqType::Playurl(area, true);
    // ohhhhh 我的老天爷, 什么勾巴处理方式
    // 为了热点路径的性能, 不管了
    let (proxy_open, proxy_url) = match req_type {
        ReqType::Other(proxy_open, proxy_url) => (proxy_open, proxy_url),
        _ => {
            let (proxy_open, proxy_url) = req_type.get_proxy(config);
            (proxy_open, proxy_url.to_owned())
        }
    };

    let url = format!("{bili_user_status_api}?access_key={access_key}&season_id=")
        + match Area::new(area_num) {
            Area::Cn => season_id_cn_only,
            Area::Hk => season_id_hk_only,
            Area::Tw => season_id_tw_only,
            Area::Th => {
                if let Some(value) = match get_server_ip_area(
                    &config.th_proxy_playurl_open,
                    &config.th_proxy_playurl_url,
                    user_agent,
                )
                .await
                {
                    Ok(value) => {
                        if value != 4 {
                            Some(
                                "Zone {area_num} -> Detect Proxy Area Not Suitable, actual [{value}]"
                            )
                        } else {
                            None
                        }
                    }
                    Err(code) => match code {
                        2333 => Some("Zone {area_num} -> ISP Banned!"),
                        _ => Some("Zone {area_num} -> Unknown Upstream Error {code}"),
                    },
                } {
                    println!("[CHECK_PROXY_HEALTH] {value}");
                    let health_report_type = HealthReportType::Others(HealthData {
                        area_num: 0,
                        is_200_ok: false,
                        upstream_reply: UpstreamReply {
                            ..Default::default()
                        },
                        is_custom: true,
                        custom_message: format!("[CHECK_PROXY_HEALTH] {value}"),
                    });
                    report_health(health_report_type, bili_runtime).await;
                    return false;
                } else {
                    return true;
                }
            }
        };
    if let Some(value) = match async_getwebpage(&url, proxy_open, &proxy_url, user_agent, "").await
    {
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
                        Some("Zone {area_num} -> Detect Proxy Area Not Suitable")
                    } else {
                        None
                    }
                }
                -2333 => Some("Zone {area_num} -> Parse Json Error: {value}"),
                _ => Some("Zone {area_num} -> Unknown Error {code}: {value}"),
            }
        }
        Err(_) => Some("Zone {area_num} -> Detect Unavailable Proxy"),
    } {
        println!("[CHECK_PROXY_HEALTH] {value}");
        let health_report_type = HealthReportType::Others(HealthData {
            area_num,
            is_200_ok: false,
            upstream_reply: UpstreamReply {
                proxy_open,
                proxy_url: proxy_url.to_owned(),
                ..Default::default()
            },
            is_custom: true,
            custom_message: format!("[CHECK_PROXY_HEALTH] {value}"),
        });
        report_health(health_report_type, bili_runtime).await;
        return false;
    } else {
        return true;
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
    match async_getwebpage(area_api, *proxy_open, proxy_url, user_agent, "").await {
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
                    let isp = result["isp"].as_str().unwrap_or("NULL");
                    // some isp is forbidden!
                    match isp {
                        "ovh.com" => Err(2333),
                        _ => Ok(*country_code_map.get(&country_code).unwrap_or(&(0 as u8))),
                    }
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
