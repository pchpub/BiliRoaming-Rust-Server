use std::fs::{self, File};

use super::types::{BiliConfig};

pub fn load_biliconfig(config_type: Option<&str>) -> Result<BiliConfig, String> {
    let config: BiliConfig;
    let config_file: File;
    match config_type {
        None => {
            return Err("[error] 无配置文件".to_owned());
        }
        Some(value) => {
            match File::open(format!("config.{}", value)) {
                Ok(value) => {
                    config_file = value;
                }
                Err(_) => {
                    return Err("[error] 配置文件打开失败".to_owned());
                }
            }
            match value {
                "json" => config = serde_json::from_reader(config_file).unwrap(),
                "yml" => config = serde_yaml::from_reader(config_file).unwrap(),
                _ => {
                    return Err("[error] 未预期的错误-1".to_owned());
                }
            }
        }
    }
    match config_type.unwrap() {
        "json" => {
            if let Err(_) = fs::write(
                "config.json",
                serde_json::to_string_pretty(&config).unwrap(),
            ) {
                println!("[Warning] config.json 更新失败");
            }
        }
        "yml" => {
            if let Err(_) = fs::write("config.yml", serde_yaml::to_string(&config).unwrap()) {
                println!("[Warning] config.yml 更新失败");
            }
        }
        _ => {
            return Err("[error] 未预期的错误-2".to_owned());
        }
    }
    Ok(config)
}

//A method to update config （算了，牺牲点兼容性好了，就不加了
// pub fn old_config_update<'a>(config_type: Option<&str>,config_file: &File,config: &'a mut BiliConfig) -> Result<&'a BiliConfig, ()> {
//     let config_version: u16;
//     match config_type.unwrap() {
//         "json" => {
//             let config: serde_json::Value; 
//             config = serde_json::from_reader(config_file).unwrap();
//             match  config.get("config_version") {//判断下是不是老的配置
//                 Some(value) => {
//                     config_version = value.as_i64().unwrap_or(1) as u16;
//                 },
//                 None => {
//                     config_version = 1;
//                 },
//             }
//         },
//         "yml" => {
//             let config: serde_yaml::Value; 
//             config = serde_yaml::from_reader(config_file).unwrap();
//             match  config.get("config_version") {//判断下是不是老的配置
//                 Some(value) => {
//                     config_version = value.as_i64().unwrap_or(1) as u16;
//                 },
//                 None => {
//                     config_version = 1;
//                 },
//             }
//         },
//         _ => {
//             return Err(());
//         }
//     }
//     if config_version == 1 {
//         // config.report_config = 
//     }
//     Err(())
// }


