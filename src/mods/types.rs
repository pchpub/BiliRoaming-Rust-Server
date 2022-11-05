use super::request::{redis_get, redis_set};
use async_channel::{Sender, TrySendError};
use chrono::{FixedOffset, TimeZone, Utc};
use deadpool_redis::Pool;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, sync::Arc};
use urlencoding::encode;

/*
* the following is server config related
*/
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
    #[serde(default = "default_rate_limit_per_second")]
    pub rate_limit_per_second: u64,
    #[serde(default = "default_rate_limit_burst")]
    pub rate_limit_burst: u32,
    pub cn_app_playurl_api: String,
    pub tw_app_playurl_api: String,
    pub hk_app_playurl_api: String,
    pub th_app_playurl_api: String,
    pub cn_web_playurl_api: String,
    pub tw_web_playurl_api: String,
    pub hk_web_playurl_api: String,
    pub th_web_playurl_api: String,
    // pub cn_app_playurl_backup_api: String,
    // pub tw_app_playurl_backup_api: String,
    // pub hk_app_playurl_backup_api: String,
    // pub th_app_playurl_backup_api: String,
    // pub cn_web_playurl_backup_api: String,
    // pub tw_web_playurl_backup_api: String,
    // pub hk_web_playurl_backup_api: String,
    // pub th_web_playurl_backup_api: String,
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
    // #[serde(default = "default_false")]
    // pub cn_proxy_playurl_backup_policy: bool, //~~打算砍掉~~ 好，去掉一个屎山
    // #[serde(default = "default_false")]
    // pub hk_proxy_playurl_backup_policy: bool,
    // #[serde(default = "default_false")]
    // pub tw_proxy_playurl_backup_policy: bool,
    // #[serde(default = "default_false")]
    // pub th_proxy_playurl_backup_policy: bool,
    // #[serde(default = "default_string")]
    // pub cn_proxy_playurl_backup_url: String,
    // #[serde(default = "default_string")]
    // pub hk_proxy_playurl_backup_url: String,
    // #[serde(default = "default_string")]
    // pub tw_proxy_playurl_backup_url: String,
    // #[serde(default = "default_string")]
    // pub th_proxy_playurl_backup_url: String,
    // #[serde(default = "default_false")]
    // pub cn_proxy_playurl_backup_open: bool,
    // #[serde(default = "default_false")]
    // pub hk_proxy_playurl_backup_open: bool,
    // #[serde(default = "default_false")]
    // pub tw_proxy_playurl_backup_open: bool,
    // #[serde(default = "default_false")]
    // pub th_proxy_playurl_backup_open: bool,
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
    #[serde(default = "default_hashmap_string")]
    pub resign_from_local: HashMap<String, String>, //限制白名单共享带会员的uid
    #[serde(default = "default_true")]
    pub resign_from_local_open: bool, //启用后白名单将共享带会员
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
/// Generic BiliRuntime for passing frequently used `BiliConfig`, `Pool` & `async_channel Sender`
/// - Initialize at the very beginning of each handler
/// - also used in background task
pub struct BiliRuntime<'bili_runtime> {
    pub config: &'bili_runtime BiliConfig,
    pub redis_pool: &'bili_runtime Pool,
    pub channel: &'bili_runtime Arc<Sender<BackgroundTaskType>>,
}
impl<'bili_runtime> BiliRuntime<'bili_runtime> {
    pub fn new(
        config: &'bili_runtime BiliConfig,
        redis_pool: &'bili_runtime Pool,
        channel: &'bili_runtime Arc<Sender<BackgroundTaskType>>,
    ) -> BiliRuntime<'bili_runtime> {
        BiliRuntime {
            config,
            redis_pool,
            channel,
        }
    }
    // TODO: Easier Config
    pub async fn get_cache(&self, cache_type: &CacheType<'_>) -> Option<String> {
        let keys = cache_type.gen_key();
        for key in keys {
            if let Some(value) = redis_get(self.redis_pool, &key).await {
                return Some(value);
            }
        }
        None
    }
    pub async fn update_cache(&self, cache_type: &CacheType<'_>, value: &str, expire_time: u64) {
        let keys = cache_type.gen_key();
        for key in keys {
            redis_set(self.redis_pool, &key, value, expire_time)
                .await
                .unwrap()
        }
    }
    pub async fn redis_get(&self, key: &str) -> Option<String> {
        redis_get(self.redis_pool, key).await
    }
    pub async fn redis_set(&self, key: &str, value: &str, expire_time: u64) {
        redis_set(self.redis_pool, key, value, expire_time)
            .await
            .unwrap()
    }
    pub async fn send_task(&self, background_task_data: BackgroundTaskType) {
        let bilisender = Arc::clone(&self.channel);
        tokio::spawn(async move {
            //println!("[Debug] bilisender_cl.len:{}", bilisender_cl.len());
            match bilisender.try_send(background_task_data) {
                Ok(_) => (),
                Err(TrySendError::Full(_)) => {
                    println!("[Error] channel is full");
                }
                Err(TrySendError::Closed(_)) => {
                    println!("[Error] channel is closed");
                }
            };
        });
    }
}

pub enum ReqType {
    Playurl(Area, bool),
    Search(Area, bool),
    ThSeason,
    ThSubtitle,
    Accesskey,
    Other(bool, String),
}
impl ReqType {
    pub fn get_api<'config>(&self, config: &'config BiliConfig) -> &'config str {
        match self {
            ReqType::Playurl(area, is_app) => {
                if *is_app {
                    match area {
                        Area::Cn => &config.cn_app_playurl_api,
                        Area::Hk => &config.hk_app_playurl_api,
                        Area::Tw => &config.tw_app_playurl_api,
                        Area::Th => &config.th_app_playurl_api, //should not
                    }
                } else {
                    match area {
                        Area::Cn => &config.cn_web_playurl_api,
                        Area::Hk => &config.hk_web_playurl_api,
                        Area::Tw => &config.tw_web_playurl_api,
                        Area::Th => &config.th_web_playurl_api, //should not
                    }
                }
            }
            ReqType::Search(area, is_app) => {
                if *is_app {
                    match area {
                        Area::Cn => &config.cn_app_search_api,
                        Area::Hk => &config.hk_app_search_api,
                        Area::Tw => &config.tw_app_search_api,
                        Area::Th => &config.th_app_search_api, //should not
                    }
                } else {
                    match area {
                        Area::Cn => &config.cn_web_search_api,
                        Area::Hk => &config.hk_web_search_api,
                        Area::Tw => &config.tw_web_search_api,
                        Area::Th => &config.th_web_search_api, //should not
                    }
                }
            }
            ReqType::ThSeason => &config.th_app_season_api,
            ReqType::ThSubtitle => &config.th_app_season_sub_api,
            ReqType::Accesskey => unimplemented!(),
            ReqType::Other(_, _) => "",
        }
    }
    pub fn get_proxy<'config>(&self, config: &'config BiliConfig) -> (bool, &'config str) {
        match self {
            ReqType::Playurl(area, _) => match area {
                Area::Cn => (config.cn_proxy_playurl_open, &config.cn_proxy_playurl_url),
                Area::Hk => (config.hk_proxy_playurl_open, &config.hk_proxy_playurl_url),
                Area::Tw => (config.tw_proxy_playurl_open, &config.tw_proxy_playurl_url),
                Area::Th => (config.th_proxy_playurl_open, &config.th_proxy_playurl_url), //should not
            },
            ReqType::Search(area, _) => match area {
                Area::Cn => (config.cn_proxy_search_open, &config.cn_proxy_search_url),
                Area::Hk => (config.hk_proxy_search_open, &config.hk_proxy_search_url),
                Area::Tw => (config.tw_proxy_search_open, &config.tw_proxy_search_url),
                Area::Th => (config.th_proxy_search_open, &config.th_proxy_search_url),
            },
            ReqType::ThSeason => (config.th_proxy_playurl_open, &config.th_proxy_playurl_url),
            ReqType::ThSubtitle => (config.th_proxy_subtitle_open, &config.th_proxy_subtitle_url),
            ReqType::Accesskey => unimplemented!(),
            ReqType::Other(_, _) => (false, ""),
        }
    }
}

pub enum CacheType<'cache_type> {
    Playurl(&'cache_type PlayurlParams<'cache_type>, bool),
    ThSeason(&'cache_type str),
    ThSubtitle(&'cache_type str),
    EpArea(&'cache_type str),
    EpVipInfo(&'cache_type str),
    UserInfo(&'cache_type str, u64),
    UserCerInfo(&'cache_type str, u64),
}
impl<'cache_type> CacheType<'cache_type> {
    // async fn update_redis(key: &str, value: &str, expire_time: u64, redis_pool: &Pool) {
    //     redis_set(redis_pool, key, value, expire_time)
    //         .await
    //         .unwrap()
    // }
    // for better performance
    #[inline]
    pub fn gen_key(&self) -> Vec<String> {
        let mut keys = vec![];
        match self {
            CacheType::Playurl(params, ep_need_vip) => {
                let mut key = String::with_capacity(32);
                // not safe, 1 + 48 = 49, num 1's ascii...
                let area_num_str =
                    unsafe { String::from_utf8_unchecked(vec![params.area_num + 48]) };
                let is_tv_str =
                    unsafe { String::from_utf8_unchecked(vec![params.is_tv as u8 + 48]) };
                let user_is_vip_str =
                    unsafe { String::from_utf8_unchecked(vec![params.is_vip as u8 + 48]) };
                match params.is_app {
                    true => {
                        key.push_str("e");
                        key.push_str(params.ep_id);
                        key.push_str("c");
                        key.push_str(params.cid);
                        key.push_str("v");
                        key.push_str(&user_is_vip_str);
                        key.push_str("t");
                        key.push_str(&is_tv_str);
                        key.push_str(&area_num_str);
                        key += "0101";
                    }
                    false => {
                        key.push_str("e");
                        key.push_str(params.ep_id);
                        key.push_str("c");
                        key.push_str(params.cid);
                        key.push_str("v");
                        key.push_str(&user_is_vip_str);
                        key.push_str("t0");
                        key.push_str(&area_num_str);
                        key += "0701";
                    }
                };
                keys.push(key);
                // 若不是带会员专享, ep_need_vip == false, 就给non-vip也存上一份
                if !*ep_need_vip && params.is_vip {
                    let mut key = String::with_capacity(32);
                    let ep_need_vip_str =
                        unsafe { String::from_utf8_unchecked(vec![*ep_need_vip as u8 + 48]) };
                    match params.is_app {
                        true => {
                            key.push_str("e");
                            key.push_str(params.ep_id);
                            key.push_str("c");
                            key.push_str(params.cid);
                            key.push_str("v");
                            key.push_str(&ep_need_vip_str);
                            key.push_str("t");
                            key.push_str(&is_tv_str);
                            key.push_str(&area_num_str);
                            key += "0101";
                        }
                        false => {
                            key.push_str("e");
                            key.push_str(params.ep_id);
                            key.push_str("c");
                            key.push_str(params.cid);
                            key.push_str("v");
                            key.push_str(&ep_need_vip_str);
                            key.push_str("t0");
                            key.push_str(&area_num_str);
                            key += "0701";
                        }
                    };
                    keys.push(key);
                }
            }
            CacheType::ThSeason(ep_id) => {
                let mut key = String::with_capacity(16);
                key.push_str("e");
                key.push_str(ep_id);
                key += "41201";
                keys.push(key);
            }
            CacheType::ThSubtitle(season_id) => {
                let mut key = String::with_capacity(16);
                key.push_str("s");
                key.push_str(season_id);
                key += "41001";
                keys.push(key);
            }
            CacheType::EpArea(ep_id) => {
                let mut key = String::with_capacity(16);
                key.push_str("e");
                key.push_str(ep_id);
                key += "1401";
                keys.push(key);
            }
            CacheType::EpVipInfo(ep_id) => {
                let mut key = String::with_capacity(64);
                key.push_str("e");
                key.push_str(ep_id);
                key += "150101";
                keys.push(key);
            }
            CacheType::UserInfo(access_key, uid) => {
                let mut key = String::with_capacity(64);
                key.push_str("a");
                key.push_str(access_key);
                key += "20501";
                keys.push(key);
                let mut key = String::with_capacity(32);
                key.push_str("u");
                key.push_str(&uid.to_string());
                key += "20501";
                keys.push(key);
            }
            CacheType::UserCerInfo(access_key, uid) => {
                let mut key = String::with_capacity(64);
                key.push_str("a");
                key.push_str(access_key);
                key += "20602";
                keys.push(key);
                let mut key = String::with_capacity(32);
                key.push_str("u");
                key.push_str(&uid.to_string());
                key += "20602";
                keys.push(key);
            }
        };
        keys
    }
    // Not implemented
    // pub async fn update(self, redis_pool: &Pool) {
    //     match self {
    //         CacheType::Playurl(area, params, data) => {
    //             let ep_id = params.ep_id;
    //             let cid = params.cid;

    //             let need_vip = if let Some(value) =
    //                 get_ep_need_vip_background(params.ep_id, redis_pool).await
    //             {
    //                 value as u8
    //             } else {
    //                 // should not
    //                 params.is_vip as u8
    //             };
    //             let is_tv = (params.is_tv && params.is_app) as u8;
    //             let is_app: &str = if params.is_app { "01" } else { "07" };
    //             let area_num = area.num();
    //             let key = format!("e{ep_id}c{cid}v{need_vip}t{is_tv}{area_num}{is_app}01");

    //             Self::update_redis(&key, &data.to_string(), 0, redis_pool).await
    //         }
    //         CacheType::EpArea(_) => todo!(),
    //         CacheType::EpVipInfo(_) => todo!(),
    //         CacheType::EpInfo(_) => todo!(),
    //         CacheType::UserInfo(_) => todo!(),
    //         CacheType::UserCerInfo(_) => todo!(),
    //     }
    // }
}

#[macro_export]
/// `build_result_response` accept Result<String, EType>
macro_rules! build_result_response {
    ($resp:ident) => {
        match $resp {
            Ok(value) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials", "true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(value);
            }
            Err(value) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials", "true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(value.to_string());
            }
        }
    };
}

#[macro_export]
/// `build_response` accepts &str, String, EType, or any that has method `to_string()`
macro_rules! build_response {
    // support enum
    ($resp:path) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body($resp.to_string())
    };
    // support value.to_string(), etc.
    ($resp:expr) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body($resp)
    };
    ($resp:ident) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body($resp)
    };
    // support like `build_response!(-412, "什么旧版本魔人,升下级");`
    ($err_code:expr, $err_msg:expr) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body(format!(
                "{{\"code\":{},\"message\":\"其他错误: {}\"}}",
                $err_code, $err_msg
            ))
    };
}

/*
* the following is background task related struct & impl
*/
// pub enum SendData {
//     Cache(CacheTask),
//     Health(HealthReportType),
// }
pub enum BackgroundTaskType {
    HealthTask(HealthTask),
    CacheTask(CacheTask),
}
pub enum HealthTask {
    HealthCheck,
    HealthReport(HealthReportType),
}
pub struct HealthCheck {
    pub need_check_area: Vec<u8>,
}
impl HealthCheck {
    pub fn add_area(&mut self, area: Area) {
        self.need_check_area.push(area.num())
    }
}
pub enum CacheTask {
    UserInfoCacheRefresh(String),
    PlayurlCacheRefresh(PlayurlParamsStatic),
    ProactivePlayurlCacheRefresh,
    EpInfoCacheRefresh(bool, Vec<EpInfo>),
    EpAreaCacheRefresh(String, String),
}

/*
* the following is server health related struct & impl
*/
pub struct UpstreamReply {
    pub code: i64,
    pub message: String,
    pub proxy_open: bool,
    pub proxy_url: String,
}
impl std::default::Default for UpstreamReply {
    fn default() -> Self {
        Self {
            code: -2333,
            message: String::from("default null"),
            proxy_open: false,
            proxy_url: String::new(),
        }
    }
}
impl UpstreamReply {
    pub fn is_available(&self) -> bool {
        // for playurl health check only
        let code = self.code;
        match code {
            0 => true,
            -10403 => {
                if self.message == "大会员专享限制"
                    || self.message == "抱歉您所使用的平台不可观看！"
                {
                    true
                } else {
                    false
                }
            }
            10015002 => {
                if self.message == "访问权限不足" {
                    true
                } else {
                    false
                }
            }
            // 万恶的米奇妙妙屋,不用家宽就 -10500
            // link: https://t.me/biliroaming_chat/1231065
            //       https://t.me/biliroaming_chat/1231113
            -10500 => true,
            -404 => false,
            _ => false,
        }
    }
}
pub struct HealthData {
    pub area_num: u8,
    // network available
    pub is_200_ok: bool,
    pub upstream_reply: UpstreamReply,
    pub is_custom: bool,
    pub custom_message: String,
}
impl std::default::Default for HealthData {
    fn default() -> Self {
        Self {
            area_num: 0,
            is_200_ok: true,
            upstream_reply: UpstreamReply {
                ..Default::default()
            },
            is_custom: false,
            custom_message: String::new(),
        }
    }
}
impl HealthData {
    pub fn init(
        area: Area,
        is_200_ok: bool,
        upstream_reply: UpstreamReply,
        req_id: &str,
    ) -> HealthData {
        let area_num = area.num();
        let mut health_data = HealthData {
            area_num,
            is_200_ok,
            upstream_reply,
            ..Default::default()
        };
        health_data.is_custom = !health_data.is_available();
        if health_data.is_custom {
            health_data.custom_message = format!(
                "详细信息:\n区域代码: {}\n网络正常: {}\n代理信息: {}-{}\n请求资源(EP/SID/KEYWORD): {}\n上游返回信息: CODE {}, Message -> {}",
                health_data.area_num,
                health_data.is_200_ok,
                health_data.upstream_reply.proxy_open,
                health_data.upstream_reply.proxy_url,
                req_id,
                health_data.upstream_reply.code,
                health_data.upstream_reply.message
            );
        }
        health_data
    }
    pub fn init_custom(area: Area, is_200_ok: bool, custom_message: &str) -> HealthData {
        // custom HealthData only for send custom message
        let area_num = area.num();
        return HealthData {
            area_num,
            is_200_ok,
            is_custom: true,
            custom_message: custom_message.to_string(),
            ..Default::default()
        };
    }
    pub fn is_available(&self) -> bool {
        if !self.is_200_ok {
            return false;
        };
        let code = self.upstream_reply.code;
        let message = &self.upstream_reply.message;
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
            0 => true,
            -10403 => {
                if message == "大会员专享限制" || message == "抱歉您所使用的平台不可观看！"
                {
                    true
                } else {
                    false
                }
            }
            10015002 => {
                if message == "访问权限不足" {
                    true
                } else {
                    false
                }
            }
            -10500 => {
                true
                // 万恶的米奇妙妙屋,不用家宽就 -10500
                // link: https://t.me/biliroaming_chat/1231065
                //       https://t.me/biliroaming_chat/1231113
            }
            // -404除非EP弄错或者东南亚区域的ep, 否则不可能出现吧... 暂且认为是健康的
            -404 => true,
            _ => false,
        }
    }
}
pub enum HealthReportType {
    Playurl(HealthData),
    Search(HealthData),
    ThSeason(HealthData),
    Others(HealthData),
}
impl HealthReportType {
    pub fn incident_attr(&self) -> (String, String) {
        return match self {
            HealthReportType::Playurl(value) => (
                "PlayUrl".to_string(),
                match value.area_num {
                    1 => "大陆",
                    2 => "香港",
                    3 => "台湾",
                    4 => "泰区",
                    _ => "[Error] 未预期的错误",
                }
                .to_string(),
            ),
            HealthReportType::Search(value) => (
                "Search".to_string(),
                match value.area_num {
                    1 => "大陆",
                    2 => "香港",
                    3 => "台湾",
                    4 => "泰区",
                    _ => "[Error] 未预期的错误",
                }
                .to_string(),
            ),
            HealthReportType::ThSeason(value) => (
                "Season".to_string(),
                match value.area_num {
                    1 => "大陆",
                    2 => "香港",
                    3 => "台湾",
                    4 => "泰区",
                    _ => "[Error] 未预期的错误",
                }
                .to_string(),
            ),
            HealthReportType::Others(_) => (String::new(), String::new()),
        };
    }
    pub fn status_color_char(&self) -> String {
        if match self {
            HealthReportType::Playurl(value) => value.is_available(),
            HealthReportType::Search(value) => value.is_available(),
            HealthReportType::ThSeason(value) => value.is_available(),
            HealthReportType::Others(_) => true,
        } {
            "🟢".to_string()
        } else {
            "🔴".to_string()
        }
    }
    pub fn is_available(&self) -> bool {
        match self {
            HealthReportType::Playurl(value) => value.is_available(),
            HealthReportType::Search(value) => value.is_available(),
            HealthReportType::ThSeason(value) => value.is_available(),
            HealthReportType::Others(_) => false,
        }
    }
    pub fn additional_msg(&self) -> Option<&String> {
        match self {
            HealthReportType::Playurl(value) => {
                if value.is_custom {
                    Some(&value.custom_message)
                } else {
                    None
                }
            }
            HealthReportType::Search(value) => {
                if value.is_custom {
                    Some(&value.custom_message)
                } else {
                    None
                }
            }
            HealthReportType::ThSeason(value) => {
                if value.is_custom {
                    Some(&value.custom_message)
                } else {
                    None
                }
            }
            HealthReportType::Others(_) => None,
        }
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
                if config.tg_bot_token.is_empty() || config.tg_chat_id.is_empty() {
                    Err(
                        "[ERROR] tg_bot相关设置无效, 请检查是否填入tg_bot_token或tg_chat_id!"
                            .to_string(),
                    )
                } else {
                    Ok(())
                }
            }
            ReportConfig::PushPlus(config) => {
                if config.pushplus_secret.is_empty() {
                    Err("[ERROR] pushplus相关设置无效, 请检查是否填入pushplus_secret!".to_string())
                } else {
                    Ok(())
                }
            }
            ReportConfig::Custom(config) => config.init(),
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
    pub method: ReportRequestMethod,
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
            tg_bot_token: String::new(),
            tg_chat_id: String::new(),
            tg_proxy_api_open: false,
            tg_proxy_url: String::new(),
        }
    }
}

impl std::default::Default for ReportConfigPushplus {
    fn default() -> Self {
        Self {
            pushplus_push_title: "Biliroaming-Rust-Server Status".to_string(),
            pushplus_secret: String::new(),
            pushplus_group_id: String::new(),
        }
    }
}

impl std::default::Default for ReportConfigCustom {
    fn default() -> Self {
        Self {
            method: ReportRequestMethod::Post,
            url: r#"https://api.telegram.org/bot{your_token}/sendMessage"#.to_string(),
            content: "chat_id={your_chat_id}&text=大陆 Playurl:              {CnPlayurl}\n香港 Playurl:              {HkPlayurl}\n台湾 Playurl:              {TwPlayurl}\n泰区 Playurl:              {ThPlayurl}\n大陆 Search:              {CnSearch}\n香港 Search:              {HkSearch}\n台湾 Search:              {TwSearch}\n泰区 Search:              {ThSearch}\n泰区 Season:              {ThSeason}\n\n变动: {ChangedAreaName} {ChangedDataType} -> {ChangedHealthType}".to_string(),
            proxy_open: false,
            proxy_url: String::new(),
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
            (
                ReportOrderName::CnPlayurl,
                report_health_data.health_cn_playurl.clone(),
            ),
            (
                ReportOrderName::HkPlayurl,
                report_health_data.health_hk_playurl.clone(),
            ),
            (
                ReportOrderName::TwPlayurl,
                report_health_data.health_tw_playurl.clone(),
            ),
            (
                ReportOrderName::ThPlayurl,
                report_health_data.health_th_playurl.clone(),
            ),
            (
                ReportOrderName::CnSearch,
                report_health_data.health_cn_search.clone(),
            ),
            (
                ReportOrderName::HkSearch,
                report_health_data.health_hk_search.clone(),
            ),
            (
                ReportOrderName::TwSearch,
                report_health_data.health_tw_search.clone(),
            ),
            (
                ReportOrderName::ThSearch,
                report_health_data.health_th_search.clone(),
            ),
            (
                ReportOrderName::ThSeason,
                report_health_data.health_th_season.clone(),
            ),
            (
                ReportOrderName::ChangedAreaName,
                changed_area_name.to_owned(),
            ),
            (
                ReportOrderName::ChangedDataType,
                changed_data_type.to_owned(),
            ),
            (
                ReportOrderName::ChangedHealthType,
                changed_health_type.to_owned(),
            ),
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
            ReportRequestMethod::Get => {
                println!("[Error] GET has no context");
                return Err(());
            }
            ReportRequestMethod::Post => {
                let health_values = HashMap::from([
                    (
                        ReportOrderName::CnPlayurl,
                        report_health_data.health_cn_playurl.clone(),
                    ),
                    (
                        ReportOrderName::HkPlayurl,
                        report_health_data.health_hk_playurl.clone(),
                    ),
                    (
                        ReportOrderName::TwPlayurl,
                        report_health_data.health_tw_playurl.clone(),
                    ),
                    (
                        ReportOrderName::ThPlayurl,
                        report_health_data.health_th_playurl.clone(),
                    ),
                    (
                        ReportOrderName::CnSearch,
                        report_health_data.health_cn_search.clone(),
                    ),
                    (
                        ReportOrderName::HkSearch,
                        report_health_data.health_hk_search.clone(),
                    ),
                    (
                        ReportOrderName::TwSearch,
                        report_health_data.health_tw_search.clone(),
                    ),
                    (
                        ReportOrderName::ThSearch,
                        report_health_data.health_th_search.clone(),
                    ),
                    (
                        ReportOrderName::ThSeason,
                        report_health_data.health_th_season.clone(),
                    ),
                    (
                        ReportOrderName::ChangedAreaName,
                        changed_area_name.to_owned(),
                    ),
                    (
                        ReportOrderName::ChangedDataType,
                        changed_data_type.to_owned(),
                    ),
                    (
                        ReportOrderName::ChangedHealthType,
                        changed_health_type.to_owned(),
                    ),
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
pub enum ReportRequestMethod {
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
        health_report_type: &HealthReportType,
    ) -> String {
        match report_config {
            ReportConfig::TgBot(_config) => return self.generate_tg_text(health_report_type),
            ReportConfig::PushPlus(_config) => return self.generate_type_html(health_report_type),
            ReportConfig::Custom(_config) => {
                return "".to_owned();
            }
        };
    }
    fn generate_tg_text(&self, health_report_type: &HealthReportType) -> String {
        match health_report_type {
            HealthReportType::Others(value) => {
                format!(
                    "服务器温馨提醒您: \n\n{}\n\n详细信息:\n区域代码: {}\n网络正常: {}\n代理信息: {} {}\n上游返回信息: CODE {}, Message -> {}",
                    value.custom_message,
                    value.area_num,
                    value.is_200_ok,
                    value.upstream_reply.proxy_open,
                    value.upstream_reply.proxy_url,
                    value.upstream_reply.code,
                    value.upstream_reply.message
                )
            }
            _ => {
                let (area_name, data_type) = health_report_type.incident_attr();
                let color_char = health_report_type.status_color_char();
                let additional_msg = if let Some(value) = health_report_type.additional_msg() {
                    value
                } else {
                    ""
                };
                format!(
                    "服务器网络状态有变动!\n\n大陆 Playurl:              {}\n香港 Playurl:              {}\n台湾 Playurl:              {}\n泰区 Playurl:              {}\n大陆 Search:              {}\n香港 Search:              {}\n台湾 Search:              {}\n泰区 Search:              {}\n泰区 Season:              {}\n\n变动: {} {} -> {}\n\n{}",
                    self.health_cn_playurl,
                    self.health_hk_playurl,
                    self.health_tw_playurl,
                    self.health_th_playurl,
                    self.health_cn_search,
                    self.health_hk_search,
                    self.health_tw_search,
                    self.health_th_search,
                    self.health_th_season,
                    area_name,
                    data_type,
                    color_char,
                    additional_msg
                )
            }
        }
    }
    fn generate_type_html(&self, health_report_type: &HealthReportType) -> String {
        match health_report_type {
            HealthReportType::Others(value) => {
                format!(
                    "服务器温馨提醒您: {}<br>详细信息:<br>区域代码: {}<br>网络正常: {}<br>代理信息: {} {}<br>上游返回信息: CODE {}, Message -> {}",
                    value.custom_message.replace("\n", "<br>"),
                    value.area_num,
                    value.is_200_ok,
                    value.upstream_reply.proxy_open,
                    value.upstream_reply.proxy_url,
                    value.upstream_reply.code,
                    value.upstream_reply.message
                )
            }
            _ => {
                let (area_name, data_type) = health_report_type.incident_attr();
                let color_char = health_report_type.status_color_char();
                let additional_msg = if let Some(value) = health_report_type.additional_msg() {
                    value.replace("\n", "<br>")
                } else {
                    String::new()
                };
                format!(
                    "服务器网络状态有变动!<br>大陆 Playurl: {}<br>香港 Playurl: {}<br>台湾 Playurl: {}<br>泰区 Playurl: {}<br>大陆 Search: {}<br>香港 Search: {}<br>台湾 Search: {}<br>泰区 Search: {}<br>泰区 Season: {}<br>变动: {} {} -> {}<br>{}",
                    self.health_cn_playurl,
                    self.health_hk_playurl,
                    self.health_tw_playurl,
                    self.health_th_playurl,
                    self.health_cn_search,
                    self.health_hk_search,
                    self.health_tw_search,
                    self.health_th_search,
                    self.health_th_season,
                    area_name,
                    data_type,
                    color_char,
                    additional_msg
                )
            }
        }
    }
}

/*
* the following is general types
*/
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

fn default_rate_limit_per_second() -> u64 {
    3
}

fn default_rate_limit_burst() -> u32 {
    20
}

fn default_u64() -> u64 {
    0
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

    pub fn to_str(&self) -> &'static str {
        match self {
            Area::Cn => "cn",
            Area::Hk => "hk",
            Area::Tw => "tw",
            Area::Th => "th",
        }
    }
}

/*
* the following is user related struct & impl
*/
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
    pub fn is_vip(&self) -> bool {
        let dt = chrono::Local::now();
        let ts = dt.timestamp_millis() as u64;
        if self.vip_expire_time > ts {
            true
        } else {
            false
        }
    }
}

pub enum UserCerStatus {
    Black(String),
    White,
    Normal,
}
pub struct UserInfoDetail {
    pub ip: String,
    pub uid: u64,
    pub access_key: String,
    pub ua: String,
    pub auth: UserCerinfo,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserResignInfo {
    pub area_num: i32,
    pub access_key: String,
    pub refresh_token: String,
    pub expire_time: u64,
}

impl UserResignInfo {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
        // format!(
        //     "{{\"area_num\":{},\"access_key\":\"{}\",\"refresh_token\":\"{}\",\"expire_time\":{}}}",
        //     self.area_num, self.access_key, self.refresh_token, self.expire_time
        // )
    }
}

/*
* the following is playurl related struct & impl
*/
pub struct PlayurlParamsStatic {
    pub access_key: String,
    pub appkey: String,
    pub appsec: String,
    pub ep_id: String,
    pub cid: String,
    pub season_id: String,
    pub build: String,
    pub device: String,
    // extra info
    pub is_app: bool,
    pub is_tv: bool,
    pub is_th: bool,
    pub is_vip: bool,
    pub area: String,
    pub area_num: u8,
    pub user_agent: String,
}
impl PlayurlParamsStatic {
    pub fn get_playurl_type(&self) -> PlayurlType {
        if self.is_app {
            PlayurlType::ChinaApp
        } else if self.is_tv {
            PlayurlType::ChinaTv
        } else if self.is_th {
            PlayurlType::Thailand
        } else {
            PlayurlType::ChinaWeb
        }
    }
    pub fn as_ref(&self) -> PlayurlParams {
        PlayurlParams {
            access_key: &self.access_key,
            appkey: &self.appkey,
            appsec: &self.appsec,
            ep_id: &self.ep_id,
            cid: &self.cid,
            season_id: &self.season_id,
            build: &self.build,
            device: &self.device,
            is_app: self.is_app,
            is_tv: self.is_tv,
            is_th: self.is_th,
            is_vip: self.is_vip,
            area: &self.area,
            area_num: self.area_num,
            user_agent: &self.user_agent,
        }
    }
}
// lessen usage of to_string() for better perf
pub struct PlayurlParams<'playurl_params> {
    pub access_key: &'playurl_params str,
    pub appkey: &'playurl_params str,
    pub appsec: &'playurl_params str,
    pub ep_id: &'playurl_params str,
    pub cid: &'playurl_params str,
    pub season_id: &'playurl_params str,
    pub build: &'playurl_params str,
    pub device: &'playurl_params str,
    // extra info
    pub is_app: bool,
    pub is_tv: bool,
    pub is_th: bool,
    pub is_vip: bool,
    pub area: &'playurl_params str,
    pub area_num: u8,
    pub user_agent: &'playurl_params str,
}

impl<'bili_playurl_params: 'playurl_params_impl, 'playurl_params_impl> Default
    for PlayurlParams<'playurl_params_impl>
{
    fn default() -> PlayurlParams<'playurl_params_impl> {
        PlayurlParams {
            access_key: "",
            appkey: "1d8b6e7d45233436",
            appsec: "560c52ccd288fed045859ed18bffd973",
            ep_id: "",
            cid: "",
            season_id: "",
            build: "6800300",
            device: "android",
            is_app: true,
            is_tv: false,
            is_th: false,
            is_vip: false,
            area: "hk",
            area_num: 2,
            user_agent: "Dalvik/2.1.0 (Linux; U; Android 12; PFEM10 Build/SKQ1.211019.001)",
        }
    }
}
impl<'bili_playurl_params: 'playurl_params_impl, 'playurl_params_impl>
    PlayurlParams<'playurl_params_impl>
{
    fn init_area(&mut self, area: Area) {
        self.area_num = area.num();
        if self.area_num == 4 {
            self.is_th = true;
        }
        match area {
            Area::Cn => self.area = "cn",
            Area::Hk => self.area = "hk",
            Area::Tw => self.area = "tw",
            Area::Th => self.area = "th",
        }
    }
    pub fn appkey_to_sec(&mut self) -> Result<(), ()> {
        if self.is_th {
            self.appkey = "7d089525d3611b1c";
        }
        self.appsec = match self.appkey {
            "9d5889cf67e615cd" => "8fd9bb32efea8cef801fd895bef2713d", // Ai4cCreatorAndroid
            "1d8b6e7d45233436" => "560c52ccd288fed045859ed18bffd973", // Android
            "07da50c9a0bf829f" => "25bdede4e1581c836cab73a48790ca6e", // AndroidB
            "8d23902c1688a798" => "710f0212e62bd499b8d3ac6e1db9302a", // AndroidBiliThings
            "dfca71928277209b" => "b5475a8825547a4fc26c7d518eaaa02e", // AndroidHD
            "bb3101000e232e27" => "36efcfed79309338ced0380abd824ac1", // AndroidI
            "4c6e1021617d40d9" => "e559a59044eb2701b7a8628c86aa12ae", // AndroidMallTicket
            "c034e8b74130a886" => "e4e8966b1e71847dc4a3830f2d078523", // AndroidOttSdk
            "4409e2ce8ffd12b8" => "59b43e04ad6965f34319062b478f83dd", // AndroidTV
            "37207f2beaebf8d7" => "e988e794d4d4b6dd43bc0e89d6e90c43", // BiliLink
            "9a75abf7de2d8947" => "35ca1c82be6c2c242ecc04d88c735f31", // BiliScan
            "7d089525d3611b1c" => "acd495b248ec528c2eed1e862d393126", // BstarA
            "178cf125136ca8ea" => "34381a26236dd1171185c0beb042e1c6", // AndroidB
            "27eb53fc9058f8c3" => "c2ed53a74eeefe3cf99fbd01d8c9c375", // ios
            "57263273bc6b67f6" => "a0488e488d1567960d3a765e8d129f90", // Android
            "7d336ec01856996b" => "a1ce6983bc89e20a36c37f40c4f1a0dd", // AndroidB
            "85eb6835b0a1034e" => "2ad42749773c441109bdc0191257a664", // unknown
            "84956560bc028eb7" => "94aba54af9065f71de72f5508f1cd42e", // unknown
            "8e16697a1b4f8121" => "f5dd03b752426f2e623d7badb28d190a", // AndroidI
            "aae92bc66f3edfab" => "af125a0d5279fd576c1b4418a3e8276d", // PC	投稿工具
            "ae57252b0c09105d" => "c75875c596a69eb55bd119e74b07cfe3", // AndroidI
            "bca7e84c2d947ac6" => "60698ba2f68e01ce44738920a0ffe768", // login
            "4ebafd7c4951b366" => "8cb98205e9b2ad3669aad0fce12a4c13", // iPhone
            "iVGUTjsxvpLeuDCf" => "aHRmhWMLkdeMuILqORnYZocwMBpMEOdt", //Android	取流专用
            "YvirImLGlLANCLvM" => "JNlZNgfNGKZEpaDTkCdPQVXntXhuiJEM", //ios	取流专用
            //_ => Ok("560c52ccd288fed045859ed18bffd973"),
            _ => return Err(()),
        };
        Ok(())
    }
    pub fn init_params(&mut self, area: Area) {
        self.init_area(area);
        self.appkey_to_sec().unwrap();
    }
    pub fn get_playurl_type(&self) -> PlayurlType {
        if self.is_app {
            PlayurlType::ChinaApp
        } else if self.is_tv {
            PlayurlType::ChinaTv
        } else if self.is_th {
            PlayurlType::Thailand
        } else {
            PlayurlType::ChinaWeb
        }
    }
}
pub enum PlayurlType {
    Thailand,
    ChinaApp,
    ChinaWeb,
    ChinaTv,
}

/*
* the following is search related struct & impl
*/
pub struct SearchParams<'search_params> {
    pub access_key: &'search_params str,
    pub appkey: &'search_params str,
    pub appsec: &'search_params str,
    pub build: &'search_params str,
    pub device: &'search_params str,
    pub pn: &'search_params str,
    pub ts: &'search_params str,
    pub fnval: &'search_params str,
    pub statistics: &'search_params str,
    pub keyword: &'search_params str,
    // extra info
    pub is_app: bool,
    pub is_tv: bool,
    pub is_th: bool,
    pub is_vip: bool,
    pub area: &'search_params str,
    pub area_num: u8,
    pub user_agent: &'search_params str,
    pub cookie: &'search_params str,
}
impl<'search_params: 'search_params_impl, 'search_params_impl> Default
    for SearchParams<'search_params_impl>
{
    fn default() -> SearchParams<'search_params_impl> {
        SearchParams {
            access_key: "",
            appkey: "1d8b6e7d45233436",
            appsec: "560c52ccd288fed045859ed18bffd973",
            build: "6400000",
            device: "android",
            pn: "1",
            ts: "",
            fnval: "",
            statistics: "",
            keyword: "Bilibili",
            is_app: true,
            is_tv: false,
            is_th: false,
            is_vip: false,
            area: "hk",
            area_num: 2,
            user_agent: "Dalvik/2.1.0 (Linux; U; Android 12; PFEM10 Build/SKQ1.211019.001)",
            cookie: "",
        }
    }
}
impl<'search_params: 'search_params_impl, 'search_params_impl> SearchParams<'search_params_impl> {
    fn ep_area_to_area_num(&mut self) {
        match self.area {
            "cn" => self.area_num = 1,
            "hk" => self.area_num = 2,
            "tw" => self.area_num = 3,
            "th" => self.area_num = 4,
            _ => {
                self.area = "hk";
                self.area_num = 2;
            }
        }
    }
    pub fn appkey_to_sec(&mut self) -> Result<(), ()> {
        if self.is_th {
            self.appkey = "7d089525d3611b1c";
        }
        self.appsec = match self.appkey {
            "9d5889cf67e615cd" => "8fd9bb32efea8cef801fd895bef2713d", // Ai4cCreatorAndroid
            "1d8b6e7d45233436" => "560c52ccd288fed045859ed18bffd973", // Android
            "07da50c9a0bf829f" => "25bdede4e1581c836cab73a48790ca6e", // AndroidB
            "8d23902c1688a798" => "710f0212e62bd499b8d3ac6e1db9302a", // AndroidBiliThings
            "dfca71928277209b" => "b5475a8825547a4fc26c7d518eaaa02e", // AndroidHD
            "bb3101000e232e27" => "36efcfed79309338ced0380abd824ac1", // AndroidI
            "4c6e1021617d40d9" => "e559a59044eb2701b7a8628c86aa12ae", // AndroidMallTicket
            "c034e8b74130a886" => "e4e8966b1e71847dc4a3830f2d078523", // AndroidOttSdk
            "4409e2ce8ffd12b8" => "59b43e04ad6965f34319062b478f83dd", // AndroidTV
            "37207f2beaebf8d7" => "e988e794d4d4b6dd43bc0e89d6e90c43", // BiliLink
            "9a75abf7de2d8947" => "35ca1c82be6c2c242ecc04d88c735f31", // BiliScan
            "7d089525d3611b1c" => "acd495b248ec528c2eed1e862d393126", // BstarA
            "178cf125136ca8ea" => "34381a26236dd1171185c0beb042e1c6", // AndroidB
            "27eb53fc9058f8c3" => "c2ed53a74eeefe3cf99fbd01d8c9c375", // ios
            "57263273bc6b67f6" => "a0488e488d1567960d3a765e8d129f90", // Android
            "7d336ec01856996b" => "a1ce6983bc89e20a36c37f40c4f1a0dd", // AndroidB
            "85eb6835b0a1034e" => "2ad42749773c441109bdc0191257a664", // unknown
            "84956560bc028eb7" => "94aba54af9065f71de72f5508f1cd42e", // unknown
            "8e16697a1b4f8121" => "f5dd03b752426f2e623d7badb28d190a", // AndroidI
            "aae92bc66f3edfab" => "af125a0d5279fd576c1b4418a3e8276d", // PC	投稿工具
            "ae57252b0c09105d" => "c75875c596a69eb55bd119e74b07cfe3", // AndroidI
            "bca7e84c2d947ac6" => "60698ba2f68e01ce44738920a0ffe768", // login
            "4ebafd7c4951b366" => "8cb98205e9b2ad3669aad0fce12a4c13", // iPhone
            "iVGUTjsxvpLeuDCf" => "aHRmhWMLkdeMuILqORnYZocwMBpMEOdt", //Android	取流专用
            "YvirImLGlLANCLvM" => "JNlZNgfNGKZEpaDTkCdPQVXntXhuiJEM", //ios	取流专用
            //_ => Ok("560c52ccd288fed045859ed18bffd973"),
            _ => return Err(()),
        };
        Ok(())
    }
    pub fn init_params(&mut self) {
        self.ep_area_to_area_num();
        self.appkey_to_sec().unwrap();
    }
}

/*
* the following is anime info related struct & impl
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonInfo {
    pub title: String,
    pub newest_ep: u64,
    pub ep_id_map: HashMap<u64, u64>,
}
impl SeasonInfo {
    pub fn init(
        title: String,
        newest_ep: u64,
        ep_id_vec: Vec<(u64, u64)>,
    ) -> Result<SeasonInfo, ()> {
        // let newest_ep = if let newest_ep_u64 = newest_ep.parse::<u64>().unwrap() {
        //     newest_ep_u64
        // } else {
        //     return Err(())
        // };
        let season_info: SeasonInfo = SeasonInfo {
            title,
            newest_ep,
            ep_id_map: ep_id_vec.into_iter().collect(),
        };
        Ok(season_info)
    }
    pub fn serialize(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
    pub fn deserialize(season_info: &str) -> SeasonInfo {
        return serde_json::from_str(season_info).unwrap();
    }
    pub fn get_newest_ep(&self) -> String {
        return self.newest_ep.to_string();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EpInfo {
    pub ep_id: u64,
    pub need_vip: bool,
    pub title: String,
    pub season_id: u64,
}
impl std::default::Default for EpInfo {
    fn default() -> Self {
        Self {
            ep_id: 0,
            need_vip: false,
            title: String::new(),
            season_id: 0,
        }
    }
}

pub enum EpAreaCacheType {
    NoEpData, //key
    NoCurrentAreaData,
    OnlyHasCurrentAreaData(bool),
    Available(Area),
}

/*
* the following is log related struct & impl
*/
pub struct LogPlayUrl {
    pub ts: i64,
    pub ip: String,
    pub uid: u64,
    pub access_key: String,
    pub season_id: u32,
    pub ep_id: u32,
    pub area_num: u8,
}
pub struct LogSearch {
    pub ts: i64,
    pub ip: String,
    pub uid: u64,
    pub access_key: String,
    pub keywords: String,
    pub area_num: u8,
}
pub struct LogAreaAvailable {
    pub ts: i64,
    pub area_num: u8,
    pub error_code: i32,
    // {"code":0,"message":"0","ttl":1,"data":{"addr":"","country":"","province":"","isp":"","latitude":0,"longitude":0,"zone_id":0,"country_code":0}}
    pub health_check_code: i8,
    pub health_check_content: String,
}
pub struct LogAccess {
    pub ts: i64,
    pub ip: String,
    pub uid: u64,
    pub access_key: String,
    pub content: String,
}
pub struct Log {
    pub access_count: u64,
    // uid -> total access count
    pub access_uid_log: HashMap<u64, u64>,
    // ep_id -> ep access count
    pub access_playurl_log: HashMap<u64, String>,
    // keyword -> keyword search count
    pub access_search_log: HashMap<String, u64>,
    // ip -> invalid req details vec
    pub access_invaid_log: HashMap<String, u64>,
    pub area_health_log: HashMap<u8, bool>,
    pub ep_health_log: HashMap<u8, bool>,
}

pub enum EType {
    ServerGeneral,                    //兜底错误
    ServerNetworkError(&'static str), //服务器网络错误
    ServerReqError(&'static str),     //因服务器内部处理问题导致请求上游失败的错误
    ServerOnlyVIPError,               //服务器仅允许大会员使用
    ServerFatalError,                 //服务器被-412了
    // ReqFreqError(u8),
    ReqSignError,              //请求Sign异常
    ReqUAError,                //请求UA异常
    UserBlacklistedError(i64), //用户黑名单错误
    UserWhitelistedError,      //服务器仅允许白名单内用户使用
    UserNonVIPError,           //大会员错误
    UserNotLoginedError,       //用户未登录错误
    InvalidReq,
    OtherError(i64, &'static str), //其他自定义错误
    OtherUpstreamError(i64, String),
}
impl EType {
    pub fn to_string(self) -> String {
        match self {
            EType::ServerGeneral => String::from("{\"code\":-500,\"message\":\"服务器内部错误\"}"),
            EType::ServerNetworkError(value) => {
                format!("{{\"code\":-500,\"message\":\"服务器网络错误: {value}\"}}")
            }
            EType::ServerReqError(value) => {
                format!("{{\"code\":-500,\"message\":\"服务器内部错误: {value}\"}}")
            }
            EType::ServerOnlyVIPError => {
                String::from("{\"code\":-10403,\"message\":\"服务器不欢迎您: 大会员专享限制\"}")
            }
            EType::ServerFatalError => String::from(
                "{\"code\":-412,\"message\":\"服务器被草到风控了... 暂时换个服务器吧...\"}",
            ),
            // ErrorType::ReqFreqError(_) => todo!(),
            EType::ReqSignError => String::from("{\"code\":-3,\"message\":\"API校验密匙错误\"}"),
            EType::ReqUAError => String::from("{\"code\":-412,\"message\":\"请求被拦截\"}"),
            EType::UserBlacklistedError(timestamp) => {
                let dt = Utc
                    .timestamp(
                        if timestamp != 0 {
                            timestamp
                        } else {
                            3376656000
                        },
                        0,
                    )
                    .with_timezone(&FixedOffset::east(8 * 3600));
                let tips = dt.format(r#"\n%Y年%m月%d日 %H:%M解封, 请耐心等待"#);
                format!("{{\"code\":-10403,\"message\":\"服务器不欢迎您: 黑名单限制{tips}\"}}")
            }
            EType::UserWhitelistedError => {
                String::from("{\"code\":-10403,\"message\":\"服务器不欢迎您: 白名单限制\"}")
            }
            EType::UserNonVIPError => {
                String::from("{\"code\":-10403,\"message\":\"大会员专享限制\"}")
            }
            EType::UserNotLoginedError => {
                String::from("{\"code\":-101,\"message\":\"账号未登录\",\"ttl\":1}")
            }
            EType::InvalidReq => String::from("{\"code\":-412,\"message\":\"请求被拦截\"}"),
            EType::OtherError(err_code, err_msg) => {
                format!("{{\"code\":{err_code},\"message\":\"其他错误: {err_msg}\"}}")
            }
            EType::OtherUpstreamError(err_code, err_msg) => {
                format!("{{\"code\":{err_code},\"message\":\"其他上游错误: {err_msg}\"}}")
            }
        }
    }
}
