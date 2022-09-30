use deadpool_redis::Pool;
use qstring::QString;
use std::env;
use std::path::PathBuf;
use std::thread;

use super::request::redis_get;
use super::{
    request::{download, getwebpage},
    types::PlayurlType,
};

pub fn remove_parameters_playurl(
    playurl_type: PlayurlType,
    data: &mut serde_json::Value,
) -> Result<(), ()> {
    match playurl_type {
        PlayurlType::Thailand => {
            if data["code"].as_i64().unwrap() == 0 {
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
            if data["code"].as_i64().unwrap() == 0 {
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
            if data["code"].as_i64().unwrap() == 0 {
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
        },
    }
}

pub fn playurl_get_deadline(
    playurl_type: PlayurlType,
    data: &mut serde_json::Value,
) -> Result<u64, ()> {
    match playurl_type {
        PlayurlType::Thailand => {
            if data["code"].as_i64().unwrap() == 0 {
                let items = if let Some(value) = data["data"]["video_info"]["stream_list"].as_array_mut()
                {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    match item["dash_video"]["base_url"].as_str() {
                        Some(value) => {
                            let query_string = if let Ok(value) = get_query_string(value) {
                                value.replace(r#"\u0026"#, r#"\n"#)
                            } else {
                                return Err(());
                            };
                            let query = QString::from(&query_string[..]);
                            if let Some(value) = query.get("deadline") {
                                return Ok(value.parse::<u64>().unwrap());
                            }
                        }
                        None => (),
                    }
                }
                return Err(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaApp => {
            if data["code"].as_i64().unwrap() == 0 {
                let items = if let Some(value) = data["dash"]["video"].as_array_mut()
                {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    match item["base_url"].as_str() {
                        Some(value) => {
                            let query_string = if let Ok(value) = get_query_string(value) {
                                value
                            } else {
                                return Err(());
                            };
                            let query = QString::from(query_string);
                            if let Some(value) = query.get("deadline") {
                                return Ok(value.parse::<u64>().unwrap());
                            }
                        }
                        None => (),
                    }
                }
                return Err(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaWeb => {
            if data["code"].as_i64().unwrap() == 0 {
                let items = if let Some(value) = data["result"]["dash"]["video"].as_array_mut() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    match item["base_url"].as_str() {
                        Some(value) => {
                            let query_string = if let Ok(value) = get_query_string(value) {
                                value
                            } else {
                                return Err(());
                            };
                            let query = QString::from(query_string);
                            if let Some(value) = query.get("deadline") {
                                return Ok(value.parse::<u64>().unwrap());
                            }
                        }
                        None => (),
                    }
                }
                return Err(());
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaTv => {
            return Err(());
        },
    }
}

#[inline]
fn get_query_string(url: &str) -> Result<&str, ()> {
    let mut index = 0;
    for char in url.chars() {
        if char == '?' {
            return Ok(&url[index..]);
        }
        index += 1;
    }
    Err(())
}

pub fn update_server(is_auto_close: bool) {
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

pub async fn health_key_to_char(pool: &Pool, key: &str) -> String {
    match redis_get(pool, key).await {
        Some(value) => {
            match &value[..] {
                "0" => return "🟢".to_string(),
                "1" => return "🟡".to_string(),
                "2" => return "🟠".to_string(),
                "3" => return "🟠".to_string(),
                "4" => return "🔴".to_string(),
                _ => return "🔴".to_string(),
            }
        }
        None => {
            return "🔴".to_string();
        }
    }
}
