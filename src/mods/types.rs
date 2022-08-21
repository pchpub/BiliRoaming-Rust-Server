use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct BiliConfig {
    pub redis: String,
    pub woker_num: usize,
    pub port: u16,
    pub cn_app_playurl_api: String,
    pub tw_app_playurl_api: String,
    pub hk_app_playurl_api: String,
    pub th_app_playurl_api: String,
    pub cn_web_playurl_api: String,
    pub tw_web_playurl_api: String,
    pub hk_web_playurl_api: String,
    pub th_web_playurl_api: String,
    pub cn_app_playurl_backup_api: String,
    pub tw_app_playurl_backup_api: String,
    pub hk_app_playurl_backup_api: String,
    pub th_app_playurl_backup_api: String,
    pub cn_web_playurl_backup_api: String,
    pub tw_web_playurl_backup_api: String,
    pub hk_web_playurl_backup_api: String,
    pub th_web_playurl_backup_api: String,
    pub cn_app_search_api: String,
    pub tw_app_search_api: String,
    pub hk_app_search_api: String,
    pub th_app_search_api: String,
    pub cn_web_search_api: String,
    pub tw_web_search_api: String,
    pub hk_web_search_api: String,
    pub th_web_search_api: String,
    pub th_app_season_api: String,
    pub th_app_season_sub_api: String,
    pub th_app_season_sub_name: String,
    pub th_app_season_sub_open: bool,
    pub cn_proxy_playurl_url: String,
    pub hk_proxy_playurl_url: String,
    pub tw_proxy_playurl_url: String,
    pub th_proxy_playurl_url: String,
    pub cn_proxy_playurl_open: bool,
    pub hk_proxy_playurl_open: bool,
    pub tw_proxy_playurl_open: bool,
    pub th_proxy_playurl_open: bool,
    #[serde(default = "default_false")]
    pub cn_proxy_playurl_backup_policy: bool, //打算砍掉
    #[serde(default = "default_false")]
    pub hk_proxy_playurl_backup_policy: bool,
    #[serde(default = "default_false")]
    pub tw_proxy_playurl_backup_policy: bool,
    #[serde(default = "default_false")]
    pub th_proxy_playurl_backup_policy: bool,
    #[serde(default = "default_string")]
    pub cn_proxy_playurl_backup_url: String,
    #[serde(default = "default_string")]
    pub hk_proxy_playurl_backup_url: String,
    #[serde(default = "default_string")]
    pub tw_proxy_playurl_backup_url: String,
    #[serde(default = "default_string")]
    pub th_proxy_playurl_backup_url: String,
    #[serde(default = "default_false")]
    pub cn_proxy_playurl_backup_open: bool,
    #[serde(default = "default_false")]
    pub hk_proxy_playurl_backup_open: bool,
    #[serde(default = "default_false")]
    pub tw_proxy_playurl_backup_open: bool,
    #[serde(default = "default_false")]
    pub th_proxy_playurl_backup_open: bool,
    pub cn_proxy_search_url: String,
    pub hk_proxy_search_url: String,
    pub tw_proxy_search_url: String,
    pub th_proxy_search_url: String,
    pub cn_proxy_search_open: bool,
    pub hk_proxy_search_open: bool,
    pub tw_proxy_search_open: bool,
    pub th_proxy_search_open: bool,
    // pub th_accesskey : String,
    // pub th_token : String,
    // pub th_force_update : bool,
    // pub cn_accesskey : String,
    // pub cn_token : String,
    // pub cn_force_update : bool, 此方法弃用
    pub cn_proxy_token_url: String,
    pub th_proxy_token_url: String,
    pub cn_proxy_token_open: bool,
    pub th_proxy_token_open: bool,
    pub th_proxy_subtitle_url: String,
    pub th_proxy_subtitle_open: bool,
    pub aid: u64,
    pub aid_replace_open: bool,
    #[serde(default = "default_hashmap_false")]
    pub resign_pub: HashMap<String, bool>,
    #[serde(default = "default_hashmap_false")]
    pub resign_open: HashMap<String, bool>,
    #[serde(default = "default_hashmap_false")]
    pub resign_api_policy: HashMap<String, bool>, //启用后assesskey从api获取
    #[serde(default = "default_hashmap_string")]
    pub resign_api: HashMap<String, String>,
    #[serde(default = "default_hashmap_string")]
    pub resign_api_sign: HashMap<String, String>,

    pub cache: HashMap<String, u64>,
    pub local_wblist: HashMap<String, (bool, bool)>,
    #[serde(default = "default_false")]
    pub one_click_run: bool,
    pub appsearch_remake: HashMap<String, String>,
    pub websearch_remake: HashMap<String, String>,
    #[serde(default = "default_string")]
    pub donate_url: String,
    #[serde(default = "default_sign")]
    pub api_sign: String, //实验性
    #[serde(default = "default_hashmap_false")]
    pub api_assesskey_open: HashMap<String, bool>, //api是否暴露
}

fn default_false() -> bool {
    false
}

fn default_string() -> String {
    "".to_string()
}

fn default_sign() -> String {
    let rand_sign = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>();
    format!("{}", rand_sign)
}

fn default_hashmap_false() -> HashMap<String, bool>{
    HashMap::from([
        ("1".to_string(),false),
        ("2".to_string(),false),
        ("3".to_string(),false),
        ("4".to_string(),false),
    ])
}

fn default_hashmap_string() -> HashMap<String, String>{
    HashMap::from([
        ("1".to_string(),"".to_string()),
        ("2".to_string(),"".to_string()),
        ("3".to_string(),"".to_string()),
        ("4".to_string(),"".to_string()),
    ])
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserCerinfo {
    pub uid: u64,
    pub black: bool,
    pub white: bool,
    pub status_expire_time: u64,
}

impl UserCerinfo {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"uid\":{},\"black\":{},\"white\":{},\"status_expire_time\":{}}}",
            self.uid, self.black, self.white, self.status_expire_time
        )
        .to_string()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub access_key: String,
    pub uid: u64,
    pub vip_expire_time: u64,
    pub expire_time: u64,
}

impl UserInfo {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"access_key\":\"{}\",\"uid\":{},\"vip_expire_time\":{},\"expire_time\":{}}}",
            self.access_key, self.uid, self.vip_expire_time, self.expire_time
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResignInfo {
    pub area_num: i32,
    pub access_key: String,
    pub refresh_token: String,
    pub expire_time: u64,
}

impl ResignInfo {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"area_num\":{},\"access_key\":\"{}\",\"refresh_token\":\"{}\",\"expire_time\":{}}}",
            self.area_num, self.access_key, self.refresh_token, self.expire_time
        )
    }
}

pub struct SendData {
    pub data_type: u8,
    pub key: String,
    pub url: String,
    pub proxy_open: bool,
    pub user_agent: String,
    pub proxy_url: String,
    pub area_num: u8,
}

pub enum PlayurlType {
    Thailand,
    China,
}