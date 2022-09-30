use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(default = "default_true")]
    pub online_blacklist_open: bool,
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
    pub report_open: bool,
    #[serde(default)]
    pub report_config: ReportConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReportConfig {
    pub method: Method,
    pub url: String,
    pub content: String,
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
        Self {
            method: Method::Post,
            url: r#"https://api.telegram.org/bot{your_token}/sendMessage"#.to_string(),
            content: "chat_id={chat_id}&text=大陆 Playurl:              {CnPlayurl}\n香港 Playurl:              {HkPlayurl}\n台湾 Playurl:              {TwPlayurl}\n泰区 Playurl:              {TwPlayurl}\n大陆 Search:              {CnSearch}\n香港 Search:              {HkSearch}\n台湾 Search:              {TwSearch}\n泰区 Search:              {ThSearch}\n泰区 Season:              {ThSeason}\n\n变动: {ChangedAreaName} {ChangedDataType} -> {ChangedHealthType}".to_string(),
            url_separate_elements: Default::default(),
            url_insert_order: Default::default(),
            content_separate_elements: Default::default(),
            content_insert_order: Default::default(),
        }
    }
}

fn vec_char_to_string(content: &Vec<String>,start: usize,end: usize) -> Result<String,()>{
    let mut string = String::new();
    for index in start..end {
        string = string + &content[index];
    }
    Ok(string)
}

impl ReportConfig {
    pub fn init(&mut self) -> Result<(), ()> {
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
            ("ChangedAreaName",ReportOrderName::ChangedAreaName),
            ("ChangedDataType",ReportOrderName::ChangedDataType),
            ("ChangedHealthType",ReportOrderName::ChangedHealthType),
        ]);

        {
            let mut has_start = false;
            let mut start_index = 0;
            let mut last_end = 0;
            let mut index = 0;
            let len = self.url.chars().count();
            let mut chars = Vec::with_capacity(len);
            for char in self.url.chars() {
                chars.push(format!("{}",char));
            }
            for char in chars.iter() {
                match &char[..] {
                    "{" => {
                        has_start = true;
                        start_index = index;
                    }
                    "}" => {
                        if has_start {
                            match key2order.get(&vec_char_to_string(&chars,start_index + 1,index).unwrap()[..]) {
                                Some(value) => {
                                    last_end = index + 1;
                                    self.url_insert_order.push(value.clone());
                                    self.url_separate_elements
                                        .push(
                                            vec_char_to_string(&chars,last_end,start_index).unwrap()
                                        );
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
                    .push(
                        vec_char_to_string(&chars,last_end,len).unwrap()
                    );
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
                chars.push(format!("{}",char));
            }
            for char in chars.iter() {
                match &char[..] {
                    "{" => {
                        has_start = true;
                        start_index = index;
                    }
                    "}" => {
                        if has_start {
                            match key2order.get(&vec_char_to_string(&chars,start_index + 1,index).unwrap()[..]) {
                                Some(value) => {
                                    last_end = index + 1;
                                    self.content_insert_order.push(value.clone());
                                    self.content_separate_elements
                                        .push(
                                            vec_char_to_string(&chars,last_end,start_index).unwrap()
                                        );
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
                    .push(
                        vec_char_to_string(&chars,last_end,len).unwrap()
                    );
            }
        }
        Ok(())
    }

    pub fn build_url(
        &self,
        cn_playurl: &str,
        hk_playurl: &str,
        tw_playurl: &str,
        th_playurl: &str,
        cn_search: &str,
        hk_search: &str,
        tw_search: &str,
        th_search: &str,
        th_season: &str,
        changed_area_name: &str,
        changed_data_type: &str,
        changed_health_type: &str,
    ) -> Result<String,()> {
        let health_values = HashMap::from([
            (ReportOrderName::CnPlayurl, cn_playurl),
            (ReportOrderName::HkPlayurl, hk_playurl),
            (ReportOrderName::CnPlayurl, tw_playurl),
            (ReportOrderName::ThPlayurl, th_playurl),
            (ReportOrderName::CnSearch, cn_search),
            (ReportOrderName::HkSearch, hk_search),
            (ReportOrderName::TwSearch, tw_search),
            (ReportOrderName::ThSearch, th_search),
            (ReportOrderName::ThSeason, th_season),
            (ReportOrderName::ChangedAreaName,changed_area_name),
            (ReportOrderName::ChangedDataType,changed_data_type),
            (ReportOrderName::ChangedHealthType,changed_health_type),
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
                    .unwrap_or(&&"");
            index += 1;
        }
        if len_order != len_elements {
            url = url + &self.url_separate_elements[index];
        }
        return Ok(url);
    }

    pub fn build_content(
        &self,
        cn_playurl: &str,
        hk_playurl: &str,
        tw_playurl: &str,
        th_playurl: &str,
        cn_search: &str,
        hk_search: &str,
        tw_search: &str,
        th_search: &str,
        th_season: &str,
        changed_area_name: &str,
        changed_data_type: &str,
        changed_health_type: &str,
    ) -> Result<String,()>{
        match self.method {
            Method::Get => {
                println!("[Error] GET has no context");
                return Err(());
            },
            Method::Post => {
                let health_values = HashMap::from([
                    (ReportOrderName::CnPlayurl, cn_playurl),
                    (ReportOrderName::HkPlayurl, hk_playurl),
                    (ReportOrderName::CnPlayurl, tw_playurl),
                    (ReportOrderName::ThPlayurl, th_playurl),
                    (ReportOrderName::CnSearch, cn_search),
                    (ReportOrderName::HkSearch, hk_search),
                    (ReportOrderName::TwSearch, tw_search),
                    (ReportOrderName::ThSearch, th_search),
                    (ReportOrderName::ThSeason, th_season),
                    (ReportOrderName::ChangedAreaName,changed_area_name),
                    (ReportOrderName::ChangedDataType,changed_data_type),
                    (ReportOrderName::ChangedHealthType,changed_health_type),
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
                            .unwrap_or(&&"");
                    index += 1;
                }
                if len_order != len_elements {
                    content = content + &self.content_separate_elements[index];
                }
                return Ok(content);
            },
        }
        
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Clone)]
pub enum Method {
    Get,
    Post,
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
