use std::{
    fs::{self, File},
    path::Path,
};

use log::debug;

use super::types::{BiliConfig, BiliRuntime};
pub fn init_biliconfig() -> BiliConfig {
    let mut config_type: Option<&str> = None;
    let config_suffix = ["json", "yml"];
    for suffix in config_suffix {
        if Path::new(&format!("config.{suffix}")).exists() {
            config_type = Some(suffix);
        }
    }
    let mut config = match load_biliconfig(config_type) {
        Ok(value) => value,
        Err(value) => {
            println!("{value}");
            std::process::exit(78);
        }
    };
    let report_config = &mut config.report_config;
    if config.report_open {
        match report_config.init() {
            Ok(_) => (),
            Err(value) => {
                println!("{}", value);
                config.report_open = false;
            }
        }
    }
    config
}

fn load_biliconfig(config_type: Option<&str>) -> Result<BiliConfig, String> {
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

pub async fn prepare_before_start(bili_runtime: BiliRuntime<'_>) {
    // set resign_info
    if bili_runtime.config.cn_resign_info.access_key != "".to_owned() {
        bili_runtime.redis_set("a11101", &bili_runtime.config.cn_resign_info.to_json(), 0).await;
    }

    if bili_runtime.config.th_resign_info.access_key != "".to_owned() {
        bili_runtime.redis_set("a41101", &bili_runtime.config.th_resign_info.to_json(), 0).await;
    }
}

pub fn load_sslconfig() -> Result<rustls::ServerConfig, Box<dyn std::error::Error>> {
    use rustls::{Certificate, PrivateKey, ServerConfig};
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

    debug!("{:?}",keys);

    let config = ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0))?;

    Ok(config)
}

pub async fn update_biliconfig() -> Result<bool,Box<dyn std::error::Error>> {
    use tokio::fs;

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
        if config["config_version"].as_i64().unwrap_or(3) <= 3 {
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
        if config["config_version"].as_i64().unwrap_or(3) <= 3 {
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