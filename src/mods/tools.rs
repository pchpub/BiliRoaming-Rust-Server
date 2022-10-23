use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use async_channel::Sender;
use deadpool_redis::Pool;
use qstring::QString;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use super::get_bili_res::get_playurl;
use super::request::{redis_get, redis_set};
use super::types::{Area, BiliConfig, GetEpAreaType, SendData};
use super::{
    request::{download, getwebpage},
    types::PlayurlType,
};
pub fn check_playurl_need_vip(
    playurl_type: PlayurlType,
    data: &serde_json::Value,
) -> Result<bool,()> {
    match playurl_type {
        PlayurlType::Thailand => {
            Err(())
        },
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
                // return Ok(false);
                // try to get vip status from "vip_status" while "need_vip" was not found
                match data["code"].as_i64().unwrap_or(2) {
                    1 => {
                        return Ok(true);
                    },
                    0 => {
                        return Ok(false);
                    },
                    value => {
                        println!("[Debug] New vip_status Found: {} data: {}",value,serde_json::to_string_pretty(data).unwrap_or_default());
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        },
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
                return Ok(false);
            } else {
                return Err(());
            }
        },
        PlayurlType::ChinaTv => {
            Err(())
        },
    }
}

pub fn remove_parameters_playurl(
    playurl_type: PlayurlType,
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

pub fn playurl_get_deadline(
    playurl_type: PlayurlType,
    data: &mut serde_json::Value,
) -> Result<u64, ()> {
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
            if data["code"].as_i64().unwrap_or(233) == 0 {
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
            if data["code"].as_i64().unwrap_or(233) == 0 {
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

#[inline]
pub async fn get_ep_area(pool: &Pool, ep: &str, area: &u8) -> Result<GetEpAreaType, ()> {
    let key = format!("e{ep}1401");
    let data_raw = redis_get(pool, &key).await;
    if let Some(value) = data_raw {
        let mut ep_area_data: [u8; 4] = [2, 2, 2, 2];
        let mut is_all_available = true;
        for (index, char) in value.char_indices() {
            match char {
                '0' => {
                    ep_area_data[index] = 0; //0表示正常
                }
                '1' => {
                    ep_area_data[index] = 1; //非0不正常
                }
                '2' => {
                    is_all_available = false;
                }
                _ => {}
            }
        }

        if is_all_available {
            if *area == 4 && ep_area_data[3] == 0 {
                return Ok(GetEpAreaType::Available(Area::Th));
            } else if ep_area_data[*area as usize - 1] == 0 {
                return Ok(GetEpAreaType::Available(Area::new(*area)));
            } else {
                if ep_area_data[1] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Hk));
                } else if ep_area_data[2] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Tw));
                } else if ep_area_data[3] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Th));
                } else if ep_area_data[0] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Cn));
                } else {
                    return Err(()); //不这样搞的话可能被攻击时会出大问题
                }
            }
        } else {
            if ep_area_data[*area as usize - 1] == 0 {
                if *area == 2 && ep_area_data[1] == 0 {
                    return Ok(GetEpAreaType::Available(Area::Hk));
                } else {
                    return Ok(GetEpAreaType::Available(Area::new(*area as u8)));
                }
            } else {
                return Ok(GetEpAreaType::NoCurrentAreaData(key, value));
            }
        }
    } else {
        return Ok(GetEpAreaType::NoEpData(key));
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

    if config.area_cache_open {
        let ep_id = if let Some(value) = query.get("ep_id") {
            value
        } else {
            let return_data =
                match get_playurl(req, is_app, is_th, query_string, query, area_num).await {
                    Ok(value) => value,
                    Err(value) => value,
                };
            return build_response(return_data);
        };
        if let Ok(value) = get_ep_area(pool, ep_id, &area_num).await {
            match value {
                GetEpAreaType::NoCurrentAreaData(key, redis_value) => {
                    match get_playurl(req, is_app, is_th, query_string, query, area_num).await {
                        Ok(http_body) => match check_ep_available(&http_body) {
                            Ok(is_available) => {
                                if let Err(_) = update_ep_area_cache(
                                    pool,
                                    &area_num,
                                    &key,
                                    &redis_value,
                                    is_available,
                                )
                                .await
                                {
                                    println!("[Error] failed to update ep area cache");
                                }
                                return build_response(http_body);
                            }
                            Err(_) => {
                                return build_response(http_body);
                            }
                        },
                        Err(http_body) => {
                            return build_response(http_body);
                        }
                    }
                }
                GetEpAreaType::OnlyHasCurrentAreaData(is_exist) => {
                    if is_exist {
                        let return_data =
                            match get_playurl(req, is_app, is_th, query_string, query, area_num)
                                .await
                            {
                                Ok(value) => value,
                                Err(value) => value,
                            };
                        return build_response(return_data);
                    } else {
                        return build_response(
                            "{\"code\":8403,\"message\":\"该剧集被判定为没有地区能播放\"}"
                                .to_string(),
                        );
                    }
                }
                GetEpAreaType::Available(area) => {
                    let is_th: bool;
                    match area {
                        Area::Th => {
                            is_th = true;
                        }
                        _ => {
                            is_th = false;
                        }
                    }
                    let return_data = match get_playurl(
                        req,
                        is_app,
                        is_th,
                        query_string,
                        query,
                        area_num,
                    )
                    .await
                    {
                        Ok(value) => value,
                        Err(value) => value,
                    };
                    return build_response(return_data);
                }
                GetEpAreaType::NoEpData(key) => {
                    match get_playurl(req, is_app, is_th, query_string, query, area_num).await {
                        Ok(http_body) => match check_ep_available(&http_body) {
                            Ok(is_available) => {
                                if is_available {
                                    if let Err(_) = update_ep_area_cache(
                                        pool,
                                        &area_num,
                                        &key,
                                        "2222",
                                        is_available,
                                    )
                                    .await
                                    {
                                        println!("[Error] failed to update ep area cache");
                                    }
                                }
                                return build_response(http_body);
                            }
                            Err(_) => {
                                return build_response(http_body);
                            }
                        },
                        Err(http_body) => {
                            return build_response(http_body);
                        }
                    }
                }
            }
        } else {
            let return_data =
                match get_playurl(req, is_app, is_th, query_string, query, area_num).await {
                    Ok(value) => value,
                    Err(value) => value,
                };
            return build_response(return_data);
        }
    } else {
        let return_data = match get_playurl(req, is_app, is_th, query_string, query, area_num).await
        {
            Ok(value) => value,
            Err(value) => value,
        };

        return build_response(return_data);
    }
}

#[inline]
async fn update_ep_area_cache(
    pool: &Pool,
    area_num: &u8,
    key: &str,
    value: &str,
    is_available: bool,
) -> Result<(), ()> {
    let area_num = *area_num as usize;
    let new_value = {
        if is_available {
            value[..area_num - 1].to_owned() + "0" + &value[area_num..]
        } else {
            value[..area_num - 1].to_owned() + "1" + &value[area_num..]
        }
    };
    let _ = redis_set(pool, key, &new_value, 0).await;
    Ok(())
}

#[inline]
fn check_ep_available(http_body: &str) -> Result<bool, ()> {
    // 此处判断来自 @cxw620
    let http_body_json: serde_json::Value = serde_json::from_str(http_body).unwrap();
    let code = http_body_json["code"].as_i64().unwrap_or(233);
    let message = http_body_json["message"].as_str().unwrap_or("").clone();
    /*
        {"code":10015002,"message":"访问权限不足","ttl":1}
        {"code":-10403,"message":"大会员专享限制"}
        {"code":-10403,"message":"抱歉您所使用的平台不可观看！"}
        {"code":-10403,"message":"抱歉您所在地区不可观看！"}
        {"code":-400,"message":"请求错误"}
        {"code":-404,"message":"啥都木有"}
        {"code":-404,"message":"啥都木有","ttl":1}
    */
    match code {
        0 => return Ok(true),
        -10403 => {
            if message == "大会员专享限制" || message == "抱歉您所使用的平台不可观看！"
            {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        10015002 => {
            if message == "访问权限不足" {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        -10500 => {
            return Ok(true);
            // 万恶的米奇妙妙屋,不用家宽就 -10500
            // link: https://t.me/biliroaming_chat/1231065
            //       https://t.me/biliroaming_chat/1231113
        }
        -404 => {
            return Ok(false);
        }
        _ => return Err(()),
    }
}

#[inline]
fn build_response(message: String) -> HttpResponse {
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("From", "biliroaming-rust-server"))
        .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
        .insert_header(("Access-Control-Allow-Credentials", "true"))
        .insert_header(("Access-Control-Allow-Methods", "GET"))
        .body(message);
}
