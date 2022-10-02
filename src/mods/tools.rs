use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, HttpRequest};
use async_channel::Sender;
use deadpool_redis::Pool;
use qstring::QString;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use super::get_bili_res::get_playurl;
use super::request::redis_get;
use super::types::{GetEpAreaType, Area, BiliConfig, SendData};
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
        }
    }
}

pub fn playurl_get_deadline(
    playurl_type: PlayurlType,
    data: &mut serde_json::Value,
) -> Result<u64, ()> {
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
                let items = if let Some(value) = data["dash"]["video"].as_array_mut() {
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
        }
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
        Some(value) => match &value[..] {
            "0" => return "🟢".to_string(),
            "1" => return "🟡".to_string(),
            "2" => return "🟠".to_string(),
            "3" => return "🟠".to_string(),
            "4" => return "🔴".to_string(),
            _ => return "🔴".to_string(),
        },
        None => {
            return "🔴".to_string();
        }
    }
}

pub async fn get_ep_area(pool: &Pool, ep: &str, area: &u8) -> Result<GetEpAreaType, ()> {
    let key = format!("e{ep}{area}1401");
    let data_raw = redis_get(pool,&key).await;
    // let area = area.parse::<usize>().unwrap_or(2);
    if let Some(value) = data_raw {
        let mut ep_area_data: [u8;4] = [2,2,2,2];
        let mut is_all_available = true;
        for (index,char) in value.char_indices() {
            match char {
                '0' => {
                    ep_area_data[index] = 0; //0表示正常
                },
                '1' => {
                    ep_area_data[index] = 1; //非0不正常
                },
                '2' => {
                    is_all_available = false;
                },
                _ => {},
            }
        }

        if is_all_available {
            if *area == 4 && ep_area_data[3] == 0 {
                return Ok(GetEpAreaType::Available(Area::Th));
            }else{
                if ep_area_data[1] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Hk));
                }else if ep_area_data[2] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Tw));
                }else if ep_area_data[3] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Th));
                }else if ep_area_data[0] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Cn));
                }else{
                    return Err(()); //不这样搞的话可能被攻击时会出大问题
                }
            }
        }else{
            if ep_area_data[*area as usize -1] == 0 {
                if *area == 2 && ep_area_data[1] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Hk));
                }else{
                    return Ok(GetEpAreaType::Available(Area::new(*area as u8)));
                }
            }else{
                return Ok(GetEpAreaType::NoCurrentAreaData);
            }
        }
    }else{
        return Ok(GetEpAreaType::NoCurrentAreaData);
    };
}

pub async fn redir_playurl_request(req: &HttpRequest, is_app: bool, is_th: bool) -> HttpResponse {
    let (pool, config, _bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<SendData>>)>()
        .unwrap();
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let area = match query.get("area") {
        Option::Some(area) => area,
        _ => {
            if is_th {
                "th"
            } else {
                "hk"
            }
        }
    };

    let area_num: u8 = match area {
        "cn" => 1,
        "hk" => 2,
        "tw" => 3,
        "th" => 4,
        _ => 2,
    };
    if config.ep_id_area_cache_open {
        if let Ok(value) = get_ep_area(pool,query.get("ep_id").unwrap(),&area_num).await {
            match value {
                GetEpAreaType::NoCurrentAreaData => {
                    return get_playurl(req, is_app, is_th, query_string, query, area_num).await;
                },
                GetEpAreaType::OnlyHasCurrentAreaData(is_exist) => {
                    if is_exist {
                        return get_playurl(req, is_app, is_th, query_string, query, area_num).await;
                    }else{
                        return HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .insert_header(("From", "biliroaming-rust-server"))
                        .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                        .insert_header(("Access-Control-Allow-Credentials", "true"))
                        .insert_header(("Access-Control-Allow-Methods", "GET"))
                        .body("{\"code\":8403,\"message\":\"该剧集被判定为没有地区能播放\"}");
                    }
                },
                GetEpAreaType::Available(area) => {
                    let is_th: bool;
                    match area {
                        Area::Th => {
                            is_th =true;
                        },
                        _ => {
                            is_th = false;
                        }
                    }
                    return get_playurl(req, is_app, is_th, query_string, query, area.num()).await;
                },
            }
        }else{
            return get_playurl(req, is_app, is_th, query_string, query, area_num).await;
        }
    }else{
        return get_playurl(req, is_app, is_th, query_string, query, area_num).await;
    }
}