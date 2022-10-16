use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Serialize, Deserialize, Clone)]
pub struct BiliConfig {
    #[serde(default = "config_version")]
    pub config_version: u16,
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
    pub limit_biliroaming_version_min: u16, //u8其实够了(0-255),但为了保险点,用u16(0-32768)
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
    pub cn_proxy_token_url: String,
    pub th_proxy_token_url: String,
    pub cn_proxy_token_open: bool,
    pub th_proxy_token_open: bool,
    #[serde(default = "default_string")]
    pub cn_proxy_accesskey_url: String,
    #[serde(default = "default_false")]
    pub cn_proxy_accesskey_open: bool,
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
    #[serde(default)]
    pub blacklist_config: BlackListType,
    pub appsearch_remake: HashMap<String, String>,
    pub websearch_remake: HashMap<String, String>,
    #[serde(default = "default_string")]
    pub donate_url: String,
    #[serde(default = "random_string")]
    pub api_sign: String, //实验性
    #[serde(default = "default_hashmap_false")]
    pub api_assesskey_open: HashMap<String, bool>, //api是否暴露
    #[serde(default = "default_false")]
    pub report_open: bool,
    #[serde(default)]
    pub report_config: ReportConfig,
    #[serde(default = "default_false")]
    pub area_cache_open: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BlackListType {
    OnlyLocalBlackList,
    OnlyOnlineBlackList(OnlineBlackListConfig),
    MixedBlackList(OnlineBlackListConfig),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OnlineBlackListConfig {
    pub api: String,
    pub api_version: u16, //暂时没用，以后向后兼容的时候会用到
}

impl std::default::Default for BlackListType {
    fn default() -> Self {
        Self::MixedBlackList(OnlineBlackListConfig {
            api: "https://black.qimo.ink/api/users/".to_string(),
            api_version: 2,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ReportConfig {
    TgBot(ReportConfigTgBot),
    PushPlus(ReportConfigPushplus),
    Custom(ReportConfigCustom),
}

impl ReportConfig {
    pub fn init(&mut self) -> Result<(), String> {
        match self {
            ReportConfig::TgBot(config) => {
                if config.tg_bot_token.is_empty()
                    || config.tg_chat_id.is_empty()
                {
                    Err("[ERROR] tg_bot相关设置无效, 请检查是否填入tg_bot_token或tg_chat_id!".to_string())
                } else {
                    Ok(())
                }
            },
            ReportConfig::PushPlus(config) => {
                if config.pushplus_secret.is_empty() {
                    Err("[ERROR] pushplus相关设置无效, 请检查是否填入pushplus_secret!".to_string())
                } else {
                    Ok(())
                }
            },
            ReportConfig::Custom(config) => {
                config.init()
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportConfigTgBot {
    pub tg_bot_token: String,
    pub tg_chat_id: String,
    pub tg_proxy_api_open: bool,
    pub tg_proxy_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportConfigPushplus {
    pub pushplus_push_title: String,
    pub pushplus_secret: String,
    pub pushplus_group_id: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportConfigCustom {
    pub method: Method,
    pub url: String,
    pub content: String,
    pub proxy_open: bool,
    pub proxy_url: String,
    #[serde(skip)]
    url_separate_elements: Vec<String>,
    #[serde(skip)]
    url_insert_order: Vec<ReportOrderName>,
    #[serde(skip)]
    content_separate_elements: Vec<String>,
    #[serde(skip)]
    content_insert_order: Vec<ReportOrderName>,
}

impl std::default::Default for ReportConfig {
    fn default() -> Self {
        Self::TgBot(ReportConfigTgBot::default())
    }
}

impl std::default::Default for ReportConfigTgBot {
    fn default() -> Self {
        Self {
            tg_bot_token: "".to_string(),
            tg_chat_id: "".to_string(),
            tg_proxy_api_open: false,
            tg_proxy_url: "".to_string(),
        }
    }
}

impl std::default::Default for ReportConfigPushplus {
    fn default() -> Self {
        Self {
            pushplus_push_title: "Biliroaming-Rust-Server Status".to_string(),
            pushplus_secret: "".to_string(),
            pushplus_group_id: "".to_string(),
        }
    }
}

impl std::default::Default for ReportConfigCustom {
    fn default() -> Self {
        Self {
            method: Method::Post,
            url: r#"https://api.telegram.org/bot{your_token}/sendMessage"#.to_string(),
            content: "chat_id={your_chat_id}&text=大陆 Playurl:              {CnPlayurl}\n香港 Playurl:              {HkPlayurl}\n台湾 Playurl:              {TwPlayurl}\n泰区 Playurl:              {ThPlayurl}\n大陆 Search:              {CnSearch}\n香港 Search:              {HkSearch}\n台湾 Search:              {TwSearch}\n泰区 Search:              {ThSearch}\n泰区 Season:              {ThSeason}\n\n变动: {ChangedAreaName} {ChangedDataType} -> {ChangedHealthType}".to_string(),
            proxy_open: false,
            proxy_url: "".to_string(),
            url_separate_elements: Default::default(),
            url_insert_order: Default::default(),
            content_separate_elements: Default::default(),
            content_insert_order: Default::default(),
        }
    }
}

fn vec_char_to_string(content: &Vec<String>, start: usize, end: usize) -> Result<String, ()> {
    let mut string = String::new();
    for index in start..end {
        string = string + &content[index];
    }
    Ok(string)
}

impl ReportConfigCustom {
    pub fn init(&mut self) -> Result<(), String> {
        let key2order = HashMap::from([
            ("CnPlayurl", ReportOrderName::CnPlayurl),
            ("HkPlayurl", ReportOrderName::HkPlayurl),
            ("TwPlayurl", ReportOrderName::TwPlayurl),
            ("ThPlayurl", ReportOrderName::ThPlayurl),
            ("CnSearch", ReportOrderName::CnSearch),
            ("HkSearch", ReportOrderName::HkSearch),
            ("TwSearch", ReportOrderName::TwSearch),
            ("ThSearch", ReportOrderName::ThSearch),
            ("ThSeason", ReportOrderName::ThSeason),
            ("ChangedAreaName", ReportOrderName::ChangedAreaName),
            ("ChangedDataType", ReportOrderName::ChangedDataType),
            ("ChangedHealthType", ReportOrderName::ChangedHealthType),
        ]);

        {
            let mut has_start = false;
            let mut start_index = 0;
            let mut last_end = 0;
            let mut index = 0;
            let len = self.url.chars().count();
            let mut chars = Vec::with_capacity(len);
            for char in self.url.chars() {
                chars.push(format!("{}", char));
            }
            for char in chars.iter() {
                match &char[..] {
                    "{" => {
                        has_start = true;
                        start_index = index;
                    }
                    "}" => {
                        if has_start {
                            match key2order.get(
                                &vec_char_to_string(&chars, start_index + 1, index).unwrap()[..],
                            ) {
                                Some(value) => {
                                    self.url_insert_order.push(value.clone());
                                    self.url_separate_elements.push(
                                        vec_char_to_string(&chars, last_end, start_index).unwrap(),
                                    );
                                    last_end = index + 1;
                                }
                                None => {}
                            }
                            has_start = false;
                        }
                    }
                    _ => {}
                }
                index += 1;
            }
            if last_end != len {
                self.url_separate_elements
                    .push(vec_char_to_string(&chars, last_end, len).unwrap());
            }
        }
        {
            let mut has_start = false;
            let mut start_index = 0;
            let mut last_end = 0;
            let mut index = 0;
            let len = self.content.chars().count();
            let mut chars = Vec::with_capacity(len);
            for char in self.content.chars() {
                chars.push(format!("{}", char));
            }
            for char in chars.iter() {
                match &char[..] {
                    "{" => {
                        has_start = true;
                        start_index = index;
                    }
                    "}" => {
                        if has_start {
                            match key2order.get(
                                &vec_char_to_string(&chars, start_index + 1, index).unwrap()[..],
                            ) {
                                Some(value) => {
                                    self.content_insert_order.push(value.clone());
                                    self.content_separate_elements.push(
                                        vec_char_to_string(&chars, last_end, start_index).unwrap(),
                                    );
                                    last_end = index + 1;
                                }
                                None => {}
                            }
                            has_start = false;
                        }
                    }
                    _ => {}
                }
                index += 1;
            }
            if last_end != len {
                self.content_separate_elements
                    .push(vec_char_to_string(&chars, last_end, len).unwrap());
            }
        }
        Ok(())
    }

    pub fn build_url(
        &self,
        report_health_data: &ReportHealthData,
        changed_area_name: &str,
        changed_data_type: &str,
        changed_health_type: &str,
    ) -> Result<String, ()> {
        let health_values = HashMap::from([
            (ReportOrderName::CnPlayurl, report_health_data.health_cn_playurl.clone()),
            (ReportOrderName::HkPlayurl, report_health_data.health_hk_playurl.clone()),
            (ReportOrderName::TwPlayurl, report_health_data.health_tw_playurl.clone()),
            (ReportOrderName::ThPlayurl, report_health_data.health_th_playurl.clone()),
            (ReportOrderName::CnSearch, report_health_data.health_cn_search.clone()),
            (ReportOrderName::HkSearch, report_health_data.health_hk_search.clone()),
            (ReportOrderName::TwSearch, report_health_data.health_tw_search.clone()),
            (ReportOrderName::ThSearch, report_health_data.health_th_search.clone()),
            (ReportOrderName::ThSeason, report_health_data.health_th_season.clone()),
            (ReportOrderName::ChangedAreaName, changed_area_name.to_owned()),
            (ReportOrderName::ChangedDataType, changed_data_type.to_owned()),
            (ReportOrderName::ChangedHealthType, changed_health_type.to_owned()),
        ]);
        let mut url = String::new();
        let len_elements = self.url_separate_elements.len();
        let len_order = self.url_insert_order.len();
        let mut index = 0;
        while index < len_order {
            url = url + &self.url_separate_elements[index];
            url = url
                + health_values
                    .get(&self.url_insert_order[index])
                    .unwrap_or(&"".to_owned());
            index += 1;
        }
        if len_order != len_elements {
            url = url + &self.url_separate_elements[index];
        }
        // must encode params before getwebpage
        let encoded_url = match url.split_once("?") {
            Some((url_host, url_params)) => {
                let url_params_vec = url_params.split("&");
                let mut new_url_params_vec: Vec<(&str, String)> = vec![];
                let mut encoded_value;
                for item in url_params_vec {
                    let (key, value) = item.split_once("=").unwrap();
                    encoded_value = encode(value).to_string();
                    new_url_params_vec.push((key, encoded_value));
                }
                format!("{}?{}", url_host, qstring::QString::new(new_url_params_vec))
            }
            None => url,
        };
        // println!("DEBUG {}", encoded_url);
        return Ok(encoded_url);
    }

    pub fn build_content(
        &self,
        report_health_data: &ReportHealthData,
        changed_area_name: &str,
        changed_data_type: &str,
        changed_health_type: &str,
    ) -> Result<String, ()> {
        match self.method {
            Method::Get => {
                println!("[Error] GET has no context");
                return Err(());
            }
            Method::Post => {
                let health_values = HashMap::from([
                    (ReportOrderName::CnPlayurl, report_health_data.health_cn_playurl.clone()),
                    (ReportOrderName::HkPlayurl, report_health_data.health_hk_playurl.clone()),
                    (ReportOrderName::TwPlayurl, report_health_data.health_tw_playurl.clone()),
                    (ReportOrderName::ThPlayurl, report_health_data.health_th_playurl.clone()),
                    (ReportOrderName::CnSearch, report_health_data.health_cn_search.clone()),
                    (ReportOrderName::HkSearch, report_health_data.health_hk_search.clone()),
                    (ReportOrderName::TwSearch, report_health_data.health_tw_search.clone()),
                    (ReportOrderName::ThSearch, report_health_data.health_th_search.clone()),
                    (ReportOrderName::ThSeason, report_health_data.health_th_season.clone()),
                    (ReportOrderName::ChangedAreaName, changed_area_name.to_owned()),
                    (ReportOrderName::ChangedDataType, changed_data_type.to_owned()),
                    (ReportOrderName::ChangedHealthType, changed_health_type.to_owned()),
                ]);
                let mut content = String::new();
                let len_elements = self.content_separate_elements.len();
                let len_order = self.content_insert_order.len();
                let mut index = 0;
                while index < len_order {
                    content = content + &self.content_separate_elements[index];
                    content = content
                        + health_values
                            .get(&self.content_insert_order[index])
                            .unwrap_or(&"".to_owned());
                    index += 1;
                }
                if len_order != len_elements {
                    content = content + &self.content_separate_elements[index];
                }
                return Ok(content);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
enum ReportOrderName {
    CnPlayurl,
    HkPlayurl,
    TwPlayurl,
    ThPlayurl,
    CnSearch,
    HkSearch,
    TwSearch,
    ThSearch,
    ThSeason,
    ChangedAreaName,
    ChangedDataType,
    ChangedHealthType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Method {
    Get,
    Post,
}

pub struct ReportHealthData {
    pub health_cn_playurl: String,
    pub health_hk_playurl: String,
    pub health_tw_playurl: String,
    pub health_th_playurl: String,
    pub health_cn_search: String,
    pub health_hk_search: String,
    pub health_tw_search: String,
    pub health_th_search: String,
    pub health_th_season: String,
}
impl ReportHealthData {
    // 定义发送内容
    pub fn generate_msg(
        &self,
        report_config: &ReportConfig,
        health_data: &SendHealthData,
    ) -> String {
        match report_config {
            ReportConfig::TgBot(_config) => {
                return self.generate_tg_text(health_data)
            },
            ReportConfig::PushPlus(_config) => {
                return self.generate_type_html(health_data)
            },
            ReportConfig::Custom(_config) => {
                return "".to_owned();
            },
        };
    }
    fn generate_tg_text(&self, health_data: &SendHealthData) -> String {
        return format!(
            "大陆 Playurl:              {}\n香港 Playurl:              {}\n台湾 Playurl:              {}\n泰区 Playurl:              {}\n大陆 Search:              {}\n香港 Search:              {}\n台湾 Search:              {}\n泰区 Search:              {}\n泰区 Season:              {}\n\n变动: {} {} -> {}",
            self.health_cn_playurl,
            self.health_hk_playurl,
            self.health_tw_playurl,
            self.health_th_playurl,
            self.health_cn_search,
            self.health_hk_search,
            self.health_tw_search,
            self.health_th_search,
            self.health_th_season,
            health_data.area_name(),
            health_data.data_type,
            health_data.health_type.to_color_char()
        );
    }
    fn generate_type_html(&self, health_data: &SendHealthData) -> String {
        return format!(
            "大陆 Playurl: {}<br>香港 Playurl: {}<br>台湾 Playurl: {}<br>泰区 Playurl: {}<br>大陆 Search: {}<br>香港 Search: {}<br>台湾 Search: {}<br>泰区 Search: {}<br>泰区 Season: {}<br>变动: {} {} -> {}",
            self.health_cn_playurl,
            self.health_hk_playurl,
            self.health_tw_playurl,
            self.health_th_playurl,
            self.health_cn_search,
            self.health_hk_search,
            self.health_tw_search,
            self.health_th_search,
            self.health_th_season,
            health_data.area_name(),
            health_data.data_type,
            health_data.health_type.to_color_char()
        );
    }
}

fn config_version() -> u16 {
    2
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

fn default_hashmap_false() -> HashMap<String, bool> {
    HashMap::from([
        ("1".to_string(), false),
        ("2".to_string(), false),
        ("3".to_string(), false),
        ("4".to_string(), false),
    ])
}

fn default_hashmap_string() -> HashMap<String, String> {
    HashMap::from([
        ("1".to_string(), "".to_string()),
        ("2".to_string(), "".to_string()),
        ("3".to_string(), "".to_string()),
        ("4".to_string(), "".to_string()),
    ])
}

fn default_min_version() -> u16 {
    64
}

fn default_max_version() -> u16 {
    80
}

fn default_u64() -> u64 {
    0
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserCerinfo {
    pub uid: u64,
    pub black: bool,
    pub white: bool,
    #[serde(default = "default_u64")]
    pub ban_until: u64,
    pub status_expire_time: u64,
}

impl UserCerinfo {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
        // format!(
        //     "{{\"uid\":{},\"black\":{},\"white\":{},\"status_expire_time\":{}}}",
        //     self.uid, self.black, self.white, self.status_expire_time
        // )
        // .to_string()
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
        serde_json::to_string(&self).unwrap()
        // format!(
        //     "{{\"access_key\":\"{}\",\"uid\":{},\"vip_expire_time\":{},\"expire_time\":{}}}",
        //     self.access_key, self.uid, self.vip_expire_time, self.expire_time
        // )
    }
}

pub enum UserCerStatus {
    Black(String),
    White,
    Normal,
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
        serde_json::to_string(&self).unwrap()
        // format!(
        //     "{{\"area_num\":{},\"access_key\":\"{}\",\"refresh_token\":\"{}\",\"expire_time\":{}}}",
        //     self.area_num, self.access_key, self.refresh_token, self.expire_time
        // )
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
    pub is_app: bool,
    pub area_num: u8,
}

pub struct SendHealthData {
    pub area_num: u8,
    pub data_type: SesourceType,
    pub health_type: HealthType,
}

impl SendHealthData {
    pub fn area_name(&self) -> String {
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
            }
            SesourceType::Search => {
                write!(f, "Search")
            }
            SesourceType::Season => {
                write!(f, "Season")
            }
            SesourceType::Token => {
                write!(f, "Token")
            }
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
    ChinaApp,
    ChinaWeb,
    ChinaTv,
}

pub enum GetEpAreaType {
    NoEpData(String),                  //key
    NoCurrentAreaData(String, String), //key value
    OnlyHasCurrentAreaData(bool),
    Available(Area),
}

pub enum Area {
    Cn,
    Hk,
    Tw,
    Th,
}

impl Area {
    pub fn new(area_num: u8) -> Self {
        match area_num {
            1 => Self::Cn,
            2 => Self::Hk,
            3 => Self::Tw,
            4 => Self::Th,
            _ => {
                panic!("[Error] 不合法的area_num")
            }
        }
    }

    pub fn num(&self) -> u8 {
        match self {
            Area::Cn => 1,
            Area::Hk => 2,
            Area::Tw => 3,
            Area::Th => 4,
        }
    }
}
