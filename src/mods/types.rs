use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct BiliConfig {
    #[serde(default = "default_false")]
    pub auto_update: bool,
    #[serde(default = "default_true")]
    pub auto_close: bool,
    pub redis: String,
    pub woker_num: usize,
    pub port: u16,
    #[serde(default = "default_false")]
    pub limit_biliroaming_version_open: bool,
    #[serde(default = "default_min_version")]
    pub limit_biliroaming_version_min: u16,//u8其实够了(0-255),但为了保险点,用u16(0-32768)
    #[serde(default = "default_max_version")]
    pub limit_biliroaming_version_max: u16,
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
    #[serde(default = "random_string")]
    pub api_sign: String, //实验性
    #[serde(default = "default_hashmap_false")]
    pub api_assesskey_open: HashMap<String, bool>, //api是否暴露
    #[serde(default = "default_false")]
    pub telegram_report: bool,
    #[serde(default = "default_string")]
    pub telegram_token : String,
    #[serde(default = "default_string")]
    pub telegram_chat_id: String,
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_string() -> String {
    "".to_string()
}

pub fn random_string() -> String {
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

fn default_min_version() -> u16 {
    64
}

fn default_max_version() -> u16 {
    80
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

pub enum SendData {
    Playurl(SendPlayurlData),
    Health(SendHealthData),
}

pub struct SendPlayurlData {
    pub key: String,
    pub url: String,
    pub proxy_open: bool,
    pub user_agent: String,
    pub proxy_url: String,
    pub area_num: u8,
}

pub struct SendHealthData {
    pub area_num: u8,
    pub data_type: SesourceType, 
    pub health_type: HealthType,
}

impl SendHealthData {
    pub fn area_name(&self) -> String{
        match self.area_num {
            1 => "大陆".to_string(),
            2 => "香港".to_string(),
            3 => "台湾".to_string(),
            4 => "泰区".to_string(),
            _ => "[Error] 未预期的错误".to_string(),
        }
    }
}

pub enum SesourceType {
    PlayUrl,
    Search,
    Season,
    Token,
}

impl std::fmt::Display for SesourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SesourceType::PlayUrl => {
                write!(f, "PlayUrl")
            },
            SesourceType::Search => {
                write!(f, "Search")
            },
            SesourceType::Season => {
                write!(f, "Season")
            },
            SesourceType::Token => {
                write!(f, "Token")
            },
        }
    }
}

pub enum HealthType {
    Online,
    Offline,
    Unknown,
    Closed,
}

impl HealthType {
    pub fn to_color_char(&self) -> String {
        match self {
            HealthType::Online => return "🟢".to_string(),
            HealthType::Offline => return "🔴".to_string(),
            HealthType::Unknown => return "🟡".to_string(),
            HealthType::Closed => return "🟤".to_string(),
        }
    }
}

pub enum PlayurlType {
    Thailand,
    China,
}