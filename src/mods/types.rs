use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct BiliConfig {
    pub redis : String,
    pub woker_num : usize,
    pub port : u16,
    pub cn_app_playurl_api : String,
    pub tw_app_playurl_api : String,
    pub hk_app_playurl_api : String,
    pub th_app_playurl_api : String,
    pub cn_web_playurl_api : String,
    pub tw_web_playurl_api : String,
    pub hk_web_playurl_api : String,
    pub th_web_playurl_api : String,
    pub cn_app_playurl_backup_api : String,
    pub tw_app_playurl_backup_api : String,
    pub hk_app_playurl_backup_api : String,
    pub th_app_playurl_backup_api : String,
    pub cn_web_playurl_backup_api : String,
    pub tw_web_playurl_backup_api : String,
    pub hk_web_playurl_backup_api : String,
    pub th_web_playurl_backup_api : String,
    pub cn_app_search_api : String,
    pub tw_app_search_api : String,
    pub hk_app_search_api : String,
    pub th_app_search_api : String,
    pub cn_web_search_api : String,
    pub tw_web_search_api : String,
    pub hk_web_search_api : String,
    pub th_web_search_api : String,
    pub th_app_season_api : String,
    pub th_app_season_sub_api : String,
    pub th_app_season_sub_name : String,
    pub th_app_season_sub_open : bool,
    pub cn_proxy_playurl_url : String,
    pub hk_proxy_playurl_url : String,
    pub tw_proxy_playurl_url : String,
    pub th_proxy_playurl_url : String,
    pub cn_proxy_playurl_open : bool,
    pub hk_proxy_playurl_open : bool,
    pub tw_proxy_playurl_open : bool,
    pub th_proxy_playurl_open : bool,
    pub cn_proxy_playurl_backup_policy : bool,
    pub hk_proxy_playurl_backup_policy : bool,
    pub tw_proxy_playurl_backup_policy : bool,
    pub th_proxy_playurl_backup_policy : bool,
    pub cn_proxy_playurl_backup_url : String,
    pub hk_proxy_playurl_backup_url : String,
    pub tw_proxy_playurl_backup_url : String,
    pub th_proxy_playurl_backup_url : String,
    pub cn_proxy_playurl_backup_open : bool,
    pub hk_proxy_playurl_backup_open : bool,
    pub tw_proxy_playurl_backup_open : bool,
    pub th_proxy_playurl_backup_open : bool,
    pub cn_proxy_search_url : String,
    pub hk_proxy_search_url : String,
    pub tw_proxy_search_url : String,
    pub th_proxy_search_url : String,
    pub cn_proxy_search_open : bool,
    pub hk_proxy_search_open : bool,
    pub tw_proxy_search_open : bool,
    pub th_proxy_search_open : bool,
    // pub th_accesskey : String,
    // pub th_token : String,
    // pub th_force_update : bool,
    // pub cn_accesskey : String,
    // pub cn_token : String,
    // pub cn_force_update : bool, 此方法弃用
    pub cn_proxy_token_url : String,
    pub th_proxy_token_url : String,
    pub cn_proxy_token_open : bool,
    pub th_proxy_token_open : bool,
    pub th_proxy_subtitle_url : String,
    pub th_proxy_subtitle_open : bool,
    pub aid : u64,
    pub aid_replace_open : bool,
    pub resign_pub : HashMap<String,bool>,
    pub resign_open : HashMap<String,bool>,
    pub cache : HashMap<String, u64>,
    pub local_wblist : HashMap<String, (bool, bool)>,
    pub one_click_run : bool,
    pub search_remake : HashMap<String, String>,
}

#[derive(Serialize, Deserialize,Clone)]
pub struct UserCerinfo {
    pub uid: u64,
    pub black:bool,
    pub white:bool,
    pub status_expire_time: u64,
}

impl UserCerinfo {
    pub fn to_json(&self) -> String {
        format!("{{\"uid\":{},\"black\":{},\"white\":{},\"status_expire_time\":{}}}", self.uid,self.black,self.white,self.status_expire_time).to_string()
    }
}

#[derive(Serialize, Deserialize,Clone)]
pub struct UserInfo {
    pub access_key: String,
    pub uid: u64,
    pub vip_expire_time: u64,
    pub expire_time: u64,
}

impl UserInfo {
    pub fn to_json(&self) -> String {
        format!("{{\"access_key\":\"{}\",\"uid\":{},\"vip_expire_time\":{},\"expire_time\":{}}}", self.access_key,self.uid,self.vip_expire_time,self.expire_time)
    }
}

#[derive(Serialize, Deserialize,Clone)]
pub struct ResignInfo {
    pub area_num : i32,
    pub access_key: String,
    pub refresh_token: String,
    pub expire_time : u64,
}

impl ResignInfo {
    pub fn to_json(&self) -> String {
        format!("{{\"area_num\":{},\"access_key\":\"{}\",\"refresh_token\":\"{}\",\"expire_time\":{}}}",self.area_num,self.access_key,self.refresh_token,self.expire_time)
    }
}

pub struct SendData {
    pub data_type : u8,
    pub key : String,
    pub url : String,
    pub proxy_open : bool,
    pub user_agent : String,
    pub proxy_url: String,
}
