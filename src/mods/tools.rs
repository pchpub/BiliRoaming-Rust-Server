use super::types::{ClientType, PlayurlType};
use base64::prelude::*;
use log::{debug, error};
use pcre2::bytes::Regex;
use rand::Rng;
use std::u8;

#[inline]
pub fn check_vip_status_from_playurl(
    playurl_type: PlayurlType,
    data: &serde_json::Value,
) -> Result<bool, ()> {
    match playurl_type {
        PlayurlType::Thailand => Err(()),
        PlayurlType::ChinaApp => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                let mut quality_need_vip: Vec<u64> = Vec::with_capacity(2);
                let items = if let Some(value) = data["support_formats"].as_array() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    if item["need_vip"].as_bool().unwrap_or(false) {
                        quality_need_vip.push(item["quality"].as_u64().unwrap_or(0));
                    }
                }

                if quality_need_vip.len() != 0 {
                    for video in {
                        if let Some(value) = data["dash"]["video"].as_array() {
                            value
                        } else {
                            error!(
                                r#"[VIP STATUS] data["dash"]["video"] not exist DATA: {}"#,
                                data.to_string()
                            );
                            return Err(());
                        }
                    } {
                        if quality_need_vip.contains(&video["id"].as_u64().unwrap_or(0)) {
                            return Ok(true);
                        }
                    }
                    return Ok(false);
                }

                match data["vip_status"].as_i64() {
                    Some(vip_status) => {
                        match vip_status {
                            // 这种方法会让 试看 的情况出现问题,所以不作为首选方法
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
                    }
                    None => {
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaWeb => {
            if data["code"].as_i64().unwrap_or(233) == 0 {
                if data["result"]["is_preview"].as_bool().unwrap_or(false) {
                    return Ok(true);
                }
                let mut quality_need_vip: Vec<u64> = Vec::with_capacity(2);
                let items = if let Some(value) = data["result"]["support_formats"].as_array() {
                    value
                } else {
                    return Err(());
                };
                for item in items {
                    if item["need_vip"].as_bool().unwrap_or(false) {
                        quality_need_vip.push(item["quality"].as_u64().unwrap_or(0));
                        // return Ok(true);
                    }
                }

                if quality_need_vip.len() != 0 {
                    for video in data["result"]["dash"]["video"].as_array().unwrap() {
                        if quality_need_vip.contains(&video["id"].as_u64().unwrap_or(0)) {
                            return Ok(true);
                        }
                    }
                }

                match data["result"]["vip_status"].as_i64() {
                    Some(vip_status) => match vip_status {
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
                    },
                    None => return Err(()),
                }
            } else {
                return Err(());
            }
        }
        PlayurlType::ChinaTv => Err(()), //过年回家的时候抓包看看（宿舍没电视机）
    }
}

#[inline]
pub fn get_user_mid_from_playurl(playurl_string: &str) -> Option<u64> {
    // 感觉不太优雅...
    let re = Regex::new(r#"(?m)&mid=(\d{0,})&platform"#).unwrap();
    let mid = match re.captures(playurl_string.as_bytes()) {
        Ok(value) => {
            if let Some(value) = value {
                value
            } else {
                return None;
            }
        }
        Err(value) => {
            error!("REGEX CAPTURE FAILED: {:?}", value);
            return None;
        }
    };
    let mid: u64 = String::from_utf8(mid[1].to_vec()).unwrap().parse().unwrap();
    Some(mid)
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
        _ => Ok(()),
    }
}

#[inline]
/// 大会员获取非大会员专享视频时, 且缓存为非大会员时: 去除大会员专享清晰度
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
            if data.contains(r#""need_vip":true"#) {
                // 处理
                let expire_time = &data[..13];
                let mut data_json: serde_json::Value =
                    if let Ok(value) = serde_json::from_str(&data[13..]) {
                        value
                    } else {
                        debug!("[TOOLS] 解析JSON失败: {data}");
                        return None;
                    };
                data_json.as_object_mut().unwrap().remove("vip_type");
                data_json.as_object_mut().unwrap().remove("vip_status");
                data_json["has_paid"] = serde_json::Value::Bool(false);
                let mut quality_to_del: Vec<u64> = vec![];
                let mut support_format_allowed = serde_json::Value::Null; //获取最高画质那档的信息
                let mut support_format_allowed_found = false;
                // 移除support_formats里的need_vip内容
                let support_formats =
                    if let Some(value) = data_json["support_formats"].as_array_mut() {
                        value
                    } else {
                        return None;
                    };
                support_formats.retain(|support_format| {
                    if support_format.as_object().unwrap().contains_key("need_vip")
                        && support_format["need_vip"].as_bool().unwrap_or(true)
                    {
                        quality_to_del.push(support_format["quality"].as_u64().unwrap_or(0));
                        false
                    } else {
                        if !support_format_allowed_found {
                            support_format_allowed = support_format.clone();
                        }
                        support_format_allowed_found = true;
                        true
                    }
                });

                if support_format_allowed_found {
                    data_json["format"] = support_format_allowed["format"].clone();
                    data_json["quality"] = support_format_allowed["quality"].clone();
                }

                data_json["dash"]["video"]
                    .as_array_mut()
                    .unwrap()
                    .retain(|video| {
                        if quality_to_del.contains(&video["id"].as_u64().unwrap_or(0)) {
                            false
                        } else {
                            true
                        }
                    });

                new_return_data.push_str(expire_time);
                new_return_data.push_str(&data_json.to_string());
            } else {
                return None;
            }
        }
        PlayurlType::ChinaWeb => {
            if data.contains(r#""need_vip":true"#) {
                let expire_time = &data[..13];
                let mut data_json: serde_json::Value =
                    if let Ok(value) = serde_json::from_str(&data[13..]) {
                        value
                    } else {
                        return None;
                    };
                let data_json_result = if data_json["code"].as_i64().unwrap_or(-2333) == 0 {
                    &mut data_json["result"]
                } else {
                    return None;
                };
                data_json_result.as_object_mut().unwrap().remove("vip_type");
                data_json_result
                    .as_object_mut()
                    .unwrap()
                    .remove("vip_status");
                data_json_result["has_paid"] = serde_json::Value::Bool(false);
                let mut quality_to_del: Vec<u64> = vec![];
                let mut support_format_allowed = serde_json::Value::Null; //获取最高画质那档的信息
                let mut support_format_allowed_found = false;
                // 不应当删除support_format里面的内容, 否则网页端显示异常, APP端没影响就保持原样了
                let support_formats = data_json_result["support_formats"].as_array_mut().unwrap();
                for support_format in support_formats {
                    if support_format.as_object().unwrap().contains_key("need_vip")
                        && support_format["need_vip"].as_bool().unwrap_or(true)
                    {
                        quality_to_del.push(support_format["quality"].as_u64().unwrap_or(0));
                    } else {
                        if !support_format_allowed_found {
                            support_format_allowed = support_format.clone();
                        }
                        support_format_allowed_found = true;
                    }
                }

                if support_format_allowed_found {
                    data_json_result["format"] = support_format_allowed["format"].clone();
                    data_json_result["quality"] = support_format_allowed["quality"].clone();
                }

                data_json_result["dash"]["video"]
                    .as_array_mut()
                    .unwrap()
                    .retain(|video| {
                        if quality_to_del.contains(&video["id"].as_u64().unwrap_or(0)) {
                            false
                        } else {
                            true
                        }
                    });
                new_return_data.push_str(expire_time);
                new_return_data.push_str(&data_json.to_string());
            } else {
                return None;
            }
        }
        PlayurlType::ChinaTv => {
            // 没电视, 这直接None算了
            return None;
        }
    };
    Some(new_return_data)
}

#[inline]
/// - 返回(`appkey`, `appsec`, `mobi_app`).
pub fn get_mobi_app(client_type: &ClientType) -> (&'static str, &'static str, &'static str) {
    match client_type {
        ClientType::Android => (
            "1d8b6e7d45233436",
            "560c52ccd288fed045859ed18bffd973",
            "android",
        ),
        ClientType::AndroidB => (
            "07da50c9a0bf829f",
            "25bdede4e1581c836cab73a48790ca6e",
            "android_b",
        ),
        ClientType::AndroidHD => (
            "dfca71928277209b",
            "b5475a8825547a4fc26c7d518eaaa02e",
            "android_hd",
        ),
        ClientType::AndroidI => (
            // bilibili 国际版
            "ae57252b0c09105d",
            "c75875c596a69eb55bd119e74b07cfe3",
            "android_i",
        ),
        ClientType::Ios => (
            "27eb53fc9058f8c3",
            "c2ed53a74eeefe3cf99fbd01d8c9c375",
            "iphone", // ios
        ),
        ClientType::Web => (
            "27eb53fc9058f8c3",
            "c2ed53a74eeefe3cf99fbd01d8c9c375",
            "iphone", // web 用的ios的appkey
        ),
        // ClientType::AndroidTV => todo!(), // 等会儿测试
        // ClientType::BstarA => todo!(), // 不应该获得BstarA
        _ => (
            "783bbb7264451d82",
            "2653583c8873dea268ab9386918b1d65",
            "android",
        ),
    }
}

// 砍都砍掉了
// pub async fn update_server<T: std::fmt::Display>(is_auto_close: bool) {
//     let update_task = tokio::task::spawn(async {
//         let mut tags = format!("v{}", env!("CARGO_PKG_VERSION"));
//         println!("[Info] 开始检查更新");
//         loop {
//             let releases_date = if let Ok(value) = async_getwebpage(
//                 "https://api.github.com/repos/pchpub/BiliRoaming-Rust-Server/releases/latest",
//                 false,
//                 "",
//                 "BiliRoaming-Rust-Server",
//                 "",
//                 None,
//             ).await {
//                 value.resp_content
//             } else {
//                 continue;
//             };
//             let releases_json: serde_json::Value = if let Ok(value) = serde_json::from_str(&releases_date) {
//                 value
//             } else {
//                 continue;
//             };
//             if releases_json["tag_name"].as_str().unwrap() == tags {
//                 continue;
//             }
//             for item in releases_json["assets"].as_array().unwrap() {
//                 if item["name"].as_str().unwrap() == "biliroaming_rust_server" {
//                     let download_url = item["browser_download_url"].as_str().unwrap();
//                     //println!("{:?}", env::current_exe().unwrap());
//                     match download(
//                         download_url.to_string(),
//                         false,
//                         "".to_string(),
//                         "".to_string(),
//                         env::current_exe().unwrap_or(PathBuf::from(r#"./biliroaming_rust_server"#)),
//                     ) {
//                         Ok(_) => {
//                             if is_auto_close {
//                                 println!("BiliRoaming Rust Server 下载完成,已关闭,等待自动重启");
//                                 std::process::exit(0); //自动更新是给用systemctl的人用到的,退出程序,这很好
//                             } else {
//                                 tags = releases_json["tag_name"].as_str().unwrap().to_string();
//                                 println!("BiliRoaming Rust Server 下载完成,请手动重启"); //有的人用systemctl，有的人用screen，退出程序不太好
//                                 break;
//                             }
//                         }
//                         Err(_) => {
//                             println!("[Error] 更新服务端失败喵"); //这个喵是自动添加的,本来不打算留的（但留着感觉挺好的
//                         }
//                     }
//                 }
//             }
//             thread::sleep(std::time::Duration::from_secs(6 * 60 * 60));
//         }
//     });
//     update_task.await;
// }

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

// x-bili-aurora-eid
#[inline]
pub fn gen_aurora_eid(mid: &str) -> String {
    // 需要使用mid进行生成, 生成逻辑
    let mut result_byte = vec![];
    let mid_byte = mid.as_bytes();
    for i in 0..mid_byte.len() {
        result_byte.push(mid_byte[i] ^ (b"ad1va46a7lza"[i % 12]))
    }
    let final_string = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD_NO_PAD,
        result_byte,
    );
    final_string
}

/// 有些api带eid, 这时候就可以获取到mid, 此函数作为后备方案
pub fn eid_to_mid(eid: &str) -> Result<String, ()> {
    fn mid_and_index_to_mid(mid: &u8, index: &usize) -> Result<char, ()> {
        let index = index % 12;
        match (mid, index) {
            (81, 0) => Ok('0'),
            (84, 1) => Ok('0'),
            (1, 2) => Ok('0'),
            (70, 3) => Ok('0'),
            (81, 4) => Ok('0'),
            (4, 5) => Ok('0'),
            (6, 6) => Ok('0'),
            (81, 7) => Ok('0'),
            (7, 8) => Ok('0'),
            (92, 9) => Ok('0'),
            (74, 10) => Ok('0'),
            (81, 11) => Ok('0'),
            (80, 0) => Ok('1'),
            (85, 1) => Ok('1'),
            (0, 2) => Ok('1'),
            (71, 3) => Ok('1'),
            (80, 4) => Ok('1'),
            (5, 5) => Ok('1'),
            (7, 6) => Ok('1'),
            (80, 7) => Ok('1'),
            (6, 8) => Ok('1'),
            (93, 9) => Ok('1'),
            (75, 10) => Ok('1'),
            (80, 11) => Ok('1'),
            (83, 0) => Ok('2'),
            (86, 1) => Ok('2'),
            (3, 2) => Ok('2'),
            (68, 3) => Ok('2'),
            (83, 4) => Ok('2'),
            (6, 5) => Ok('2'),
            (4, 6) => Ok('2'),
            (83, 7) => Ok('2'),
            (5, 8) => Ok('2'),
            (94, 9) => Ok('2'),
            (72, 10) => Ok('2'),
            (83, 11) => Ok('2'),
            (82, 0) => Ok('3'),
            (87, 1) => Ok('3'),
            (2, 2) => Ok('3'),
            (69, 3) => Ok('3'),
            (82, 4) => Ok('3'),
            (7, 5) => Ok('3'),
            (5, 6) => Ok('3'),
            (82, 7) => Ok('3'),
            (4, 8) => Ok('3'),
            (95, 9) => Ok('3'),
            (73, 10) => Ok('3'),
            (82, 11) => Ok('3'),
            (85, 0) => Ok('4'),
            (80, 1) => Ok('4'),
            (5, 2) => Ok('4'),
            (66, 3) => Ok('4'),
            (85, 4) => Ok('4'),
            (0, 5) => Ok('4'),
            (2, 6) => Ok('4'),
            (85, 7) => Ok('4'),
            (3, 8) => Ok('4'),
            (88, 9) => Ok('4'),
            (78, 10) => Ok('4'),
            (85, 11) => Ok('4'),
            (84, 0) => Ok('5'),
            (81, 1) => Ok('5'),
            (4, 2) => Ok('5'),
            (67, 3) => Ok('5'),
            (84, 4) => Ok('5'),
            (1, 5) => Ok('5'),
            (3, 6) => Ok('5'),
            (84, 7) => Ok('5'),
            (2, 8) => Ok('5'),
            (89, 9) => Ok('5'),
            (79, 10) => Ok('5'),
            (84, 11) => Ok('5'),
            (87, 0) => Ok('6'),
            (82, 1) => Ok('6'),
            (7, 2) => Ok('6'),
            (64, 3) => Ok('6'),
            (87, 4) => Ok('6'),
            (2, 5) => Ok('6'),
            (0, 6) => Ok('6'),
            (87, 7) => Ok('6'),
            (1, 8) => Ok('6'),
            (90, 9) => Ok('6'),
            (76, 10) => Ok('6'),
            (87, 11) => Ok('6'),
            (86, 0) => Ok('7'),
            (83, 1) => Ok('7'),
            (6, 2) => Ok('7'),
            (65, 3) => Ok('7'),
            (86, 4) => Ok('7'),
            (3, 5) => Ok('7'),
            (1, 6) => Ok('7'),
            (86, 7) => Ok('7'),
            (0, 8) => Ok('7'),
            (91, 9) => Ok('7'),
            (77, 10) => Ok('7'),
            (86, 11) => Ok('7'),
            (89, 0) => Ok('8'),
            (92, 1) => Ok('8'),
            (9, 2) => Ok('8'),
            (78, 3) => Ok('8'),
            (89, 4) => Ok('8'),
            (12, 5) => Ok('8'),
            (14, 6) => Ok('8'),
            (89, 7) => Ok('8'),
            (15, 8) => Ok('8'),
            (84, 9) => Ok('8'),
            (66, 10) => Ok('8'),
            (89, 11) => Ok('8'),
            (88, 0) => Ok('9'),
            (93, 1) => Ok('9'),
            (8, 2) => Ok('9'),
            (79, 3) => Ok('9'),
            (88, 4) => Ok('9'),
            (13, 5) => Ok('9'),
            (15, 6) => Ok('9'),
            (88, 7) => Ok('9'),
            (14, 8) => Ok('9'),
            (85, 9) => Ok('9'),
            (67, 10) => Ok('9'),
            (88, 11) => Ok('9'),
            _ => Err(()),
        }
    }
    let eid: Vec<(u8, usize)> = if let Ok(value) = BASE64_STANDARD.decode(eid) {
        value.into_iter().zip(0..).collect()
    } else {
        return Err(());
    };
    let mut mid = String::with_capacity(eid.len());
    for (single_char, index) in &eid {
        mid.push({
            if let Ok(value) = mid_and_index_to_mid(single_char, index) {
                value
            } else {
                return Err(());
            }
        });
    }
    Ok(mid)
}

pub fn spawn_random_accesskey(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let dist = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    let mut secret = String::new();
    for _ in 0..len {
        secret.push(dist[rng.gen_range(0..16)]);
    }
    secret
}

pub fn load_ssl() -> Result<rustls::ServerConfig, Box<dyn std::error::Error>> {
    use rustls::{Certificate, PrivateKey, ServerConfig};
    use std::fs::File;
    use std::io::BufReader;

    let mut cert_file = BufReader::new(File::open("certificates/fullchain.pem")?);
    let mut private_key_file = BufReader::new(File::open("certificates/privkey.pem")?);

    let cert_chain = rustls_pemfile::certs(&mut cert_file)?
        .into_iter()
        .map(|cert| Certificate(cert))
        .collect::<Vec<Certificate>>();
    let mut keys = rustls_pemfile::ec_private_keys(&mut private_key_file)?
        .into_iter()
        .map(|key| PrivateKey(key))
        .collect::<Vec<PrivateKey>>();

    let config = ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0))?;

    Ok(config)
}

pub async fn update_config_file() -> Result<bool,Box<dyn std::error::Error>> {
    use tokio::fs;
    use std::path::Path;

    async fn read_config_json() -> Result<serde_json::Value,Box<dyn std::error::Error>> {
        let config = fs::read_to_string("config.json").await?;
        let config: serde_json::Value = serde_json::from_str(&config)?;
        Ok(config)
    }

    async fn read_config_yaml() -> Result<serde_yaml::Value,Box<dyn std::error::Error>> {
        let config = fs::read_to_string("config.yaml").await?;
        let config: serde_yaml::Value = serde_yaml::from_str(&config)?;
        Ok(config)
    }

    let mut is_updated: bool = false;

    if Path::new("config.json").exists() {
        let mut config = read_config_json().await?;
        if config["config_version"].as_i64().unwrap_or(3) == 3 {
            config["http_port"] = config["port"].clone();
            config["config_version"] = serde_json::Value::from(4);
            config["worker_num"] = config["woker_num"].clone();
            is_updated = true;
        }
        if is_updated {
            fs::write("config.json", serde_json::to_string_pretty(&config)?).await?;
        }
    } else if Path::new("config.yaml").exists() {
        let mut config = read_config_yaml().await?;
        if config["config_version"].as_i64().unwrap_or(3) == 3 {
            config["http_port"] = config["port"].clone();
            config["config_version"] = serde_yaml::Value::from(4);
            config["worker_num"] = config["woker_num"].clone();
            is_updated = true;
        }
        if is_updated {
            fs::write("config.yaml", serde_yaml::to_string(&config)?).await?;
        }
    }
    Ok(is_updated)
}