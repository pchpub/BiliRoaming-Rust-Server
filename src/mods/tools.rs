use log::{debug, error};

use super::{
    request::{download, getwebpage},
    types::PlayurlType,
};
use std::env;
use std::path::PathBuf;
use std::thread;

#[inline]
pub fn check_vip_status_from_playurl(
    playurl_type: PlayurlType,
    data: &serde_json::Value,
) -> Result<bool, ()> {
    match playurl_type {
        PlayurlType::Thailand => Err(()),
        PlayurlType::ChinaApp => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let items = if let Some(value) = data["support_formats"].as_array() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    if item["need_vip"].as_bool().unwrap_or(false) {
                        return Ok(true);
                    }
                }

                match data["vip_status"].as_i64().unwrap_or(2) {
                    1 => {
                        return Ok(true);
                    }
                    0 => {
                        return Ok(false);
                    }
                    value => {
                        error!("[VIP STATUS] 发现无法处理的 vip_status: {value}");
                        error!(
                            "[VIP STATUS] 相关信息 data: {}",
                            serde_json::to_string(data).unwrap_or_default()
                        );
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaWeb => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let items = if let Some(value) = data["result"]["support_formats"].as_array() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    if item["need_vip"].as_bool().unwrap_or(false) {
                        return Ok(true);
                    }
                }

                match data["result"]["vip_status"].as_i64().unwrap_or(2) {
                    1 => {
                        return Ok(true);
                    }
                    0 => {
                        return Ok(false);
                    }
                    value => {
                        error!("[VIP STATUS] 发现无法处理的 vip_status: {value}");
                        error!(
                            "[VIP STATUS] 相关信息 data: {}",
                            serde_json::to_string(data).unwrap_or_default()
                        );
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaTv => Err(()), //过年回家的时候抓包看看（宿舍没电视机）
    }
}

#[inline]
pub fn remove_parameters_playurl(
    playurl_type: &PlayurlType,
    data: &mut serde_json::Value,
) -> Result<(), ()> {
    match playurl_type {
        PlayurlType::Thailand => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let items =
                    if let Some(value) = data["data"]["video_info"]["stream_list"].as_array_mut() {
                        value
                    } else {
                        return Err(());
                    };
                for item in items {
                    item["stream_info"]["need_vip"] = serde_json::Value::Bool(false);
                    item["stream_info"]["need_login"] = serde_json::Value::Bool(false);
                }
                return Ok(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaApp => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let items = if let Some(value) = data["support_formats"].as_array_mut() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    //item["need_login"] = serde_json::Value::Bool(false);
                    item.as_object_mut().unwrap().remove("need_login");
                    item.as_object_mut().unwrap().remove("need_vip");
                }
                return Ok(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaWeb => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let items = if let Some(value) = data["result"]["support_formats"].as_array_mut() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    //item["need_login"] = serde_json::Value::Bool(false);
                    item.as_object_mut().unwrap().remove("need_login");
                    item.as_object_mut().unwrap().remove("need_vip");
                }
                return Ok(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaTv => {
            return Ok(());
        }
    }
}

// TODO: 大会员获取非大会员专享视频时, 且缓存为非大会员时: 去除大会员专享清晰度(不去可能会被叔叔发律师函, 存在较大风险)
// 对非大会员不太友好
// 课好满(晚上继续咕咕咕)
// 要考试, 咕咕咕
#[inline]
pub async fn remove_viponly_clarity<'a>(
    playurl_type: &'a PlayurlType,
    data: &'a str,
) -> Option<String> {
    let mut new_return_data = String::with_capacity(data.len() + 32);
    match playurl_type {
        PlayurlType::Thailand => {
            // 东南亚区直接返回None, 影响不大
            return None;
        }
        PlayurlType::ChinaApp => {
            // 判断是否有带会员独享清晰度
            if data.contains(r#""need_vip":true"#) {
                // 处理
                let expire_time = &data[..13];
                let mut data_json: serde_json::Value = if let Ok(value) = serde_json::from_str(&data[13..])
                {
                    value
                } else {
                    debug!("[TOOLS] 解析JSON失败: {data}");
                    return None;
                };
                let mut accept_description_to_del: Vec<String> = vec![];
                // 移除support_formats里的need_vip内容
                let support_formats = data_json["support_formats"].as_array_mut().unwrap();
                support_formats.retain(|support_format| {
                    if support_format.as_object().unwrap().contains_key("need_vip")
                        && support_format["need_vip"].as_bool().unwrap_or(true)
                    {
                        accept_description_to_del
                            .push(support_format["description"].as_str().unwrap().to_owned());
                        false
                    } else {
                        true
                    }
                });
                // 上一步记录了description
                let accept_description = data_json["accept_description"].as_array_mut().unwrap();
                accept_description.retain(|accept_description_item| {
                    for accept_description_to_del_item in &accept_description_to_del {
                        if accept_description_item.as_str().unwrap_or("")
                            == accept_description_to_del_item
                        {
                            return false;
                        }
                    }
                    true
                });
                new_return_data.push_str(expire_time);
                new_return_data.push_str(&data_json.to_string());
            } else {
                return None;
            }
        }
        PlayurlType::ChinaWeb => {
            // 判断是否有带会员独享清晰度, web没有这个need_vip的指示, 只能机械判断
            // 目前已知的: "高清 1080P+"(改称"1080P 高码率"了)
            // 番剧没有4K的吧...
            // let expire_time = &data[..13];
            // let mut data_json: serde_json::Value = if let Ok(value) = serde_json::from_str(&data[13..]) {
            //     value
            // } else {
            //     debug!("[TOOLS] 解析JSON失败: {data}");
            //     return None;
            // };
            // let data_json = if data_json["code"].as_i64().unwrap_or(-2333) == 0 {
            //     &mut data_json["result"]
            // } else {
            //     debug!("[TOOLS] 解析JSON失败: {data}");
            //     return None;
            // };
            // // 移除support_formats里的need_vip内容
            // let support_formats = data_json["support_formats"].as_array_mut().unwrap();
            // support_formats.retain(|support_format| {
            //     match support_format["description"].as_str().unwrap() {
            //         "高清 1080P+" | "1080P 高码率" => false,
            //         _ => match support_format["new_description"].as_str().unwrap_or("") {
            //             "高清 1080P+" | "1080P 高码率" => false,
            //             _ => true,
            //         },
            //     }
            // });
            // debug!("support_formats {}", data_json["support_formats"]);
            // // 上一步记录了description
            // let accept_description = data_json["accept_description"].as_array_mut().unwrap();
            // accept_description.retain(|accept_description_item| {
            //     match accept_description_item.as_str().unwrap() {
            //         "高清 1080P+" | "1080P 高码率" => false,
            //         _ => true,
            //     }
            // });
            // debug!("accept_description {}", data_json["accept_description"]);
            // new_return_data.push_str(expire_time);
            // new_return_data.push_str(&data_json.to_string());
            return None;
        }
        PlayurlType::ChinaTv => {
            // 没电视, 这直接None算了
            return None;
        }
    };
    Some(new_return_data)
}

pub fn update_server<T: std::fmt::Display>(is_auto_close: bool) {
    thread::spawn(move || {
        let mut tags = format!("v{}", env!("CARGO_PKG_VERSION"));
        println!("[Info] 开始检查更新");
        loop {
            let releases_date = if let Ok(value) = getwebpage(
                "https://api.github.com/repos/pchpub/BiliRoaming-Rust-Server/releases/latest"
                    .to_string(),
                false,
                "".to_string(),
                "BiliRoaming-Rust-Server".to_string(),
                "".to_owned(),
            ) {
                value
            } else {
                continue;
            };
            let releases_json: serde_json::Value =
                if let Ok(value) = serde_json::from_str(&releases_date) {
                    value
                } else {
                    continue;
                };
            if releases_json["tag_name"].as_str().unwrap() == tags {
                continue;
            }
            for item in releases_json["assets"].as_array().unwrap() {
                if item["name"].as_str().unwrap() == "biliroaming_rust_server" {
                    let download_url = item["browser_download_url"].as_str().unwrap();
                    //println!("{:?}", env::current_exe().unwrap());
                    match download(
                        download_url.to_string(),
                        false,
                        "".to_string(),
                        "".to_string(),
                        env::current_exe().unwrap_or(PathBuf::from(r#"./biliroaming_rust_server"#)),
                    ) {
                        Ok(_) => {
                            if is_auto_close {
                                println!("BiliRoaming Rust Server 下载完成,已关闭,等待自动重启");
                                std::process::exit(0); //自动更新是给用systemctl的人用到的,退出程序,这很好
                            } else {
                                tags = releases_json["tag_name"].as_str().unwrap().to_string();
                                println!("BiliRoaming Rust Server 下载完成,请手动重启"); //有的人用systemctl，有的人用screen，退出程序不太好
                                break;
                            }
                        }
                        Err(_) => {
                            println!("[Error] 更新服务端失败喵"); //这个喵是自动添加的,本来不打算留的（但留着感觉挺好的
                        }
                    }
                }
            }
            thread::sleep(std::time::Duration::from_secs(6 * 60 * 60));
        }
    });
}

pub fn vec_to_string<T: std::fmt::Display>(vec: &Vec<T>, delimiter: &str) -> String {
    match vec.len() {
        0 => "".to_owned(),
        1 => vec[0].to_string(),
        _ => {
            let mut processed_string = String::with_capacity(32); //TO CHECK
            for single_key in vec.iter().zip(0..) {
                if single_key.1 == 0 {
                    processed_string.push_str(&single_key.0.to_string());
                } else {
                    processed_string.push_str(delimiter);
                    processed_string.push_str(&single_key.0.to_string());
                }
            }
            processed_string
        }
    }
}
