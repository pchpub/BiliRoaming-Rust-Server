use super::{
    ep_info::get_ep_need_vip,
    request::{redis_get, redis_set},
    tools::remove_viponly_clarity,
};
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
    #[serde(default = "default_api_bilibili_com")]
    pub general_api_bilibili_com_proxy_api: String,
    #[serde(default = "default_app_bilibili_com")]
    pub general_app_bilibili_com_proxy_api: String,
    pub aid: u64,
    pub aid_replace_open: bool,
    #[serde(default = "default_hashmap_false")]
    pub resign_pub: HashMap<String, bool>,
    #[serde(default = "default_hashmap_false")]
    pub resign_open: HashMap<String, bool>,
    #[serde(default = "default_false")]
    pub resign_from_existed_key: bool, // 仅限 cn (危险功能)
    // #[serde(default = "default_hashmap_string")]                                 // 与上方 resign 功能重复
    // pub resign_from_local: HashMap<String, String>, //限制白名单共享带会员的uid    // 注释在 user_info.rs 的 255 行
    // #[serde(default = "default_true")]                                           //
    // pub resign_from_local_open: bool, //启用后白名单将共享带会员                   //
    #[serde(default = "default_hashmap_false")]
    pub resign_from_api_open: HashMap<String, bool>, //启用后assesskey从api获取
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
    // 以下为不会序列化的配置
    #[serde(skip_serializing, default)]
    pub cn_resign_info: UserResignInfo,
    #[serde(skip_serializing, default)]
    pub th_resign_info: UserResignInfo,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BlackListType {
    OnlyLocalBlackList,
    OnlyOnlineBlackList(OnlineBlackListConfig),
    MixedBlackList(OnlineBlackListConfig),
    NoOnlineBlacklist,
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
        let key = &cache_type.gen_key()[0];
        if let Some(value) = redis_get(self.redis_pool, key).await {
            return Some(value);
        }
        None
    }
    pub async fn update_cache(&self, cache_type: &CacheType<'_>, value: &str, expire_time: u64) {
        let keys = cache_type.gen_key();
        // let _new_value: &str;
        match cache_type {
            CacheType::Playurl(params) => {
                // vip用户获取到playurl后刷新缓存, keys[0]就是vip的key, keys[1]就是non-vip的key
                redis_set(self.redis_pool, &keys[0], value, expire_time).await;
                // 双保险, 虽然实际上应该只需要`keys.len() > 1`
                if params.is_vip && !params.ep_need_vip {
                    let playurl_type = &params.get_playurl_type();
                    if let Some(value) = remove_viponly_clarity(playurl_type, value).await {
                        redis_set(self.redis_pool, &keys[1], &value, expire_time)
                            .await
                            .unwrap()
                    }
                }
            }
            _ => {
                for key in keys {
                    redis_set(self.redis_pool, &key, value, expire_time)
                        .await
                        .unwrap()
                }
            }
        }

        // for key in keys {
        //     redis_set(self.redis_pool, &key, value, expire_time)
        //         .await
        //         .unwrap()
        // }
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
    Playurl(&'cache_type PlayurlParams<'cache_type>),
    ThSeason(&'cache_type str),
    ThSubtitle(&'cache_type str),
    EpArea(&'cache_type str),
    EpVipInfo(&'cache_type str),
    UserInfo(&'cache_type str, u64),
    UserCerInfo(&'cache_type str, u64),
    // UserUniqueInfo(&'cache_type str, u64)
}
impl<'cache_type> CacheType<'cache_type> {
    #[inline]
    pub fn gen_key(&self) -> Vec<String> {
        let mut keys = Vec::with_capacity(2);
        match self {
            CacheType::Playurl(params) => {
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
                if !params.ep_need_vip && params.is_vip {
                    let mut key = String::with_capacity(32);
                    // let ep_need_vip_str =
                    //     unsafe { String::from_utf8_unchecked(vec![params.ep_need_vip as u8 + 48]) };
                    match params.is_app {
                        true => {
                            key.push_str("e");
                            key.push_str(params.ep_id);
                            key.push_str("c");
                            key.push_str(params.cid);
                            key.push_str("v0");
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
                            key.push_str("v0");
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
            // CacheType::UserUniqueInfo(access_key, uid) => {
            //     let mut key = String::with_capacity(64);
            //     key.push_str("a");
            //     key.push_str(access_key);
            //     key += "2001";
            //     keys.push(key);
            //     let mut key = String::with_capacity(32);
            //     key.push_str("u");
            //     key.push_str(&uid.to_string());
            //     key += "2001";
            //     keys.push(key);
            // },
        };
        keys
    }
}

// pub enum CacheKey {
//     // 只能改gen_key了, 返回playurl cache的时候改返回值的话对性能消耗估计挺大的
//     CommonKey(String),
//     SpecialKey(String), // 需要后续处理的key
// }

// impl CacheKey {
//     pub fn gen_raw_key(&self) -> &str {
//         match self {
//             key => &key,
//             CacheKey::SpecialKey(key) => &key,
//         }
//     }
// }

// impl Display for CacheKey {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             key => {
//                 write!(f, "{}", key)
//             }
//             CacheKey::SpecialKey(key) => {
//                 write!(f, "{}", key)
//             }
//         }
//     }
// }

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

#[macro_export]
/// support like `build_signed_url!(unsigned_url, vec![query_param], "sign_secret");`, return tuple (signed_url, md5_sign), mg5_sign for debug
macro_rules! build_signed_url {
    ($unsigned_url:expr, $query_vec:expr, $sign_secret:expr) => {{
        let req_params = qstring::QString::new($query_vec).to_string();
        let mut signed_url = String::with_capacity(600);
        signed_url.push_str(&($unsigned_url));
        signed_url.push_str("?");
        signed_url.push_str(&req_params);
        signed_url.push_str("&sign=");
        let mut sign = crypto::md5::Md5::new();
        crypto::digest::Digest::input_str(&mut sign, &(req_params + $sign_secret));
        let md5_sign = crypto::digest::Digest::result_str(&mut sign);
        signed_url.push_str(&md5_sign);
        (signed_url, md5_sign)
    }};
}
#[macro_export]
/// support like `build_signed_url!(unsigned_url, vec![query_param], "sign_secret");`, return tuple (signed_url, md5_sign), mg5_sign for debug
macro_rules! build_signed_params {
    ($query_vec:expr, $sign_secret:expr) => {{
        let req_params = qstring::QString::new($query_vec).to_string();
        let mut signed_params = String::with_capacity(600);
        signed_params.push_str(&req_params);
        signed_params.push_str("&sign=");
        let mut sign = crypto::md5::Md5::new();
        crypto::digest::Digest::input_str(&mut sign, &(req_params + $sign_secret));
        let md5_sign = crypto::digest::Digest::result_str(&mut sign);
        signed_params.push_str(&md5_sign);
        (signed_params, md5_sign)
    }};
}

/*
* the following is background task related struct & impl
*/
pub enum BackgroundTaskType {
    Health(HealthTask),
    Cache(CacheTask),
}
pub enum HealthTask {
    HealthCheck,
    HealthReport(HealthReportType),
}
pub enum CacheTask {
    UserInfoCacheRefresh(String, u8),
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
    pub upstream_header: String,
    pub proxy_open: bool,
    pub proxy_url: String,
}
impl std::default::Default for UpstreamReply {
    fn default() -> Self {
        Self {
            code: -2333,
            message: String::from("default null"),
            upstream_header: String::from("default null"),
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
                "详细信息:\n区域代码: {}\n网络正常: {}\n代理信息: {} {}\n请求资源(EP/SID/KEYWORD): {}\n上游返回信息: CODE {}, Message -> {}\n上游返回Headers: \n{}",
                health_data.area_num,
                health_data.is_200_ok,
                health_data.upstream_reply.proxy_open,
                health_data.upstream_reply.proxy_url,
                req_id,
                health_data.upstream_reply.code,
                health_data.upstream_reply.message,
                health_data.upstream_reply.upstream_header,
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
    pub fn is_available(&self) -> bool {
        match self {
            HealthReportType::Playurl(value) => value.is_available(),
            HealthReportType::Search(value) => value.is_available(),
            HealthReportType::ThSeason(value) => value.is_available(),
            HealthReportType::Others(_) => false,
        }
    }
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
        if self.is_available() {
            "🟢".to_string()
        } else {
            "🔴".to_string()
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
    pub method: ReportConfigCustomRequestMethod,
    pub url: String,
    pub content: String,
    pub proxy_open: bool,
    pub proxy_url: String,
    #[serde(skip)]
    url_separate_elements: Vec<String>,
    #[serde(skip)]
    url_insert_order: Vec<ReportConfigCustomOrderName>,
    #[serde(skip)]
    content_separate_elements: Vec<String>,
    #[serde(skip)]
    content_insert_order: Vec<ReportConfigCustomOrderName>,
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
            method: ReportConfigCustomRequestMethod::Post,
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
            ("CnPlayurl", ReportConfigCustomOrderName::CnPlayurl),
            ("HkPlayurl", ReportConfigCustomOrderName::HkPlayurl),
            ("TwPlayurl", ReportConfigCustomOrderName::TwPlayurl),
            ("ThPlayurl", ReportConfigCustomOrderName::ThPlayurl),
            ("CnSearch", ReportConfigCustomOrderName::CnSearch),
            ("HkSearch", ReportConfigCustomOrderName::HkSearch),
            ("TwSearch", ReportConfigCustomOrderName::TwSearch),
            ("ThSearch", ReportConfigCustomOrderName::ThSearch),
            ("ThSeason", ReportConfigCustomOrderName::ThSeason),
            (
                "ChangedAreaName",
                ReportConfigCustomOrderName::ChangedAreaName,
            ),
            (
                "ChangedDataType",
                ReportConfigCustomOrderName::ChangedDataType,
            ),
            (
                "ChangedHealthType",
                ReportConfigCustomOrderName::ChangedHealthType,
            ),
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
                ReportConfigCustomOrderName::CnPlayurl,
                report_health_data.health_cn_playurl.clone(),
            ),
            (
                ReportConfigCustomOrderName::HkPlayurl,
                report_health_data.health_hk_playurl.clone(),
            ),
            (
                ReportConfigCustomOrderName::TwPlayurl,
                report_health_data.health_tw_playurl.clone(),
            ),
            (
                ReportConfigCustomOrderName::ThPlayurl,
                report_health_data.health_th_playurl.clone(),
            ),
            (
                ReportConfigCustomOrderName::CnSearch,
                report_health_data.health_cn_search.clone(),
            ),
            (
                ReportConfigCustomOrderName::HkSearch,
                report_health_data.health_hk_search.clone(),
            ),
            (
                ReportConfigCustomOrderName::TwSearch,
                report_health_data.health_tw_search.clone(),
            ),
            (
                ReportConfigCustomOrderName::ThSearch,
                report_health_data.health_th_search.clone(),
            ),
            (
                ReportConfigCustomOrderName::ThSeason,
                report_health_data.health_th_season.clone(),
            ),
            (
                ReportConfigCustomOrderName::ChangedAreaName,
                changed_area_name.to_owned(),
            ),
            (
                ReportConfigCustomOrderName::ChangedDataType,
                changed_data_type.to_owned(),
            ),
            (
                ReportConfigCustomOrderName::ChangedHealthType,
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
            ReportConfigCustomRequestMethod::Get => {
                println!("[Error] GET has no context");
                return Err(());
            }
            ReportConfigCustomRequestMethod::Post => {
                let health_values = HashMap::from([
                    (
                        ReportConfigCustomOrderName::CnPlayurl,
                        report_health_data.health_cn_playurl.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::HkPlayurl,
                        report_health_data.health_hk_playurl.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::TwPlayurl,
                        report_health_data.health_tw_playurl.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::ThPlayurl,
                        report_health_data.health_th_playurl.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::CnSearch,
                        report_health_data.health_cn_search.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::HkSearch,
                        report_health_data.health_hk_search.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::TwSearch,
                        report_health_data.health_tw_search.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::ThSearch,
                        report_health_data.health_th_search.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::ThSeason,
                        report_health_data.health_th_season.clone(),
                    ),
                    (
                        ReportConfigCustomOrderName::ChangedAreaName,
                        changed_area_name.to_owned(),
                    ),
                    (
                        ReportConfigCustomOrderName::ChangedDataType,
                        changed_data_type.to_owned(),
                    ),
                    (
                        ReportConfigCustomOrderName::ChangedHealthType,
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
enum ReportConfigCustomOrderName {
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
pub enum ReportConfigCustomRequestMethod {
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
                    "服务器温馨提醒您: \n\n{}\n\n详细信息:\n区域代码: {}\n网络正常: {}\n代理信息: {} {}\n上游返回信息: CODE {}, Message -> {}\n上游返回Headers: \n{}",
                    value.custom_message,
                    value.area_num,
                    value.is_200_ok,
                    value.upstream_reply.proxy_open,
                    value.upstream_reply.proxy_url,
                    value.upstream_reply.code,
                    value.upstream_reply.message,
                    value.upstream_reply.upstream_header,
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
                    "服务器温馨提醒您: {}<br>详细信息:<br>区域代码: {}<br>网络正常: {}<br>代理信息: {} {}<br>上游返回信息: CODE {}, Message -> {}<br>上游返回Headers: <br>{}",
                    value.custom_message.replace("\n", "<br>"),
                    value.area_num,
                    value.is_200_ok,
                    value.upstream_reply.proxy_open,
                    value.upstream_reply.proxy_url,
                    value.upstream_reply.code,
                    value.upstream_reply.message,
                    value.upstream_reply.upstream_header.replace("\n", "<br>"),
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
    3
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

fn default_api_bilibili_com() -> String {
    "api.bilibili.com".to_string()
}

fn default_app_bilibili_com() -> String {
    "app.bilibili.com".to_string()
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

fn default_i64() -> i64 {
    0
}

pub struct UpstreamRawResp {
    pub resp_header: Vec<u8>, //keep raw code
    pub resp_content: String,
}

impl UpstreamRawResp {
    pub fn new(resp_content: String, resp_header: Vec<u8>) -> UpstreamRawResp {
        UpstreamRawResp {
            resp_header,
            resp_content,
        }
    }
    pub fn init_headers(&self) -> HashMap<String, String> {
        let mut resp_header: HashMap<String, String> = HashMap::new();
        let resp_header_raw_string =
            unsafe { String::from_utf8_unchecked(self.resp_header.clone()) };
        let mut resp_header_raw_string_vec: Vec<&str> = resp_header_raw_string.split("‡").collect();
        resp_header_raw_string_vec.pop(); //去掉最后一个
        for header_item in resp_header_raw_string_vec {
            let header_item: Vec<&str> = header_item.split(": ").collect();
            if header_item.len() == 2 {
                resp_header.insert(header_item[0].to_string(), header_item[1].to_string());
            }
        }
        resp_header
    }
    pub fn json(&self) -> Option<serde_json::Value> {
        if let Ok(json_content) = serde_json::from_str(&self.resp_content) {
            Some(json_content)
        } else {
            None
        }
    }
    // pub fn read_header(&self, header: &str) -> Option<String> {
    //     let headers_hashmap = self.init_headers();
    //     headers_hashmap.get(header).unwrap()
    // }
    pub fn read_headers(&self) -> String {
        let mut headers: Vec<String> = Vec::new();
        let headers_hashmap = self.init_headers();
        for (key, value) in &headers_hashmap {
            headers.push(key.to_owned());
            unsafe {
                headers.push(String::from_utf8_unchecked(vec![58u8, 32]));
            }
            headers.push(value.to_owned());
            unsafe {
                headers.push(String::from_utf8_unchecked(vec![13u8, 10]));
            }
        }
        headers.join("")
    }
}

impl std::fmt::Display for UpstreamRawResp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.resp_content)
    }
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
    #[serde(default = "default_i64")]
    pub code: i64,
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

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            code: 0,
            access_key: Default::default(),
            uid: Default::default(),
            vip_expire_time: Default::default(),
            expire_time: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserResignInfo {
    // pub area_num: i32,
    pub access_key: String,
    pub refresh_token: String,
    pub expire_time: u64,
}

impl UserResignInfo {
    pub fn new(resin_info_str: &str) -> UserResignInfo {
        serde_json::from_str(resin_info_str).unwrap()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
        // format!(
        //     "{{\"area_num\":{},\"access_key\":\"{}\",\"refresh_token\":\"{}\",\"expire_time\":{}}}",
        //     self.area_num, self.access_key, self.refresh_token, self.expire_time
        // )
    }
}

impl Default for UserResignInfo {
    fn default() -> Self {
        Self {
            access_key: Default::default(),
            refresh_token: Default::default(),
            expire_time: Default::default(),
        }
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
    pub ep_need_vip: bool,
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
            ep_need_vip: self.ep_need_vip,
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
    pub ep_need_vip: bool,
    pub area: &'playurl_params str,
    pub area_num: u8,
    pub user_agent: &'playurl_params str,
}

pub trait HasIsappIsthUseragent {
    fn is_app(&self) -> bool;
    fn is_th(&self) -> bool;
    fn user_agent(&self) -> &str;
}

impl HasIsappIsthUseragent for PlayurlParams<'_> {
    fn is_app(&self) -> bool {
        self.is_app
    }
    fn is_th(&self) -> bool {
        self.is_th
    }
    fn user_agent(&self) -> &str {
        self.user_agent
    }
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
            ep_need_vip: true,
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
        self.area = area.to_str();
    }
    pub async fn init_ep_need_vip(&mut self, bili_runtime: &BiliRuntime<'_>) {
        self.ep_need_vip = if let Some(value) = get_ep_need_vip(self.ep_id, bili_runtime).await {
            value == 1
        } else {
            if self.is_th {
                // 此处处理东南亚区会员, 好坏一并缓存罢了
                // // 不想弄了, 麻烦的一批
                false
            } else {
                // should not
                self.is_vip
            }
        };
    }
    pub fn appkey_to_sec(&mut self) -> Result<(), ()> {
        if self.is_th {
            self.appkey = "7d089525d3611b1c";
        }
        self.appsec = match self.appkey {
            "9d5889cf67e615cd" => "8fd9bb32efea8cef801fd895bef2713d", // Ai4cCreatorAndroid
            "1d8b6e7d45233436" => "560c52ccd288fed045859ed18bffd973", // Android
            "783bbb7264451d82" => "2653583c8873dea268ab9386918b1d65", // Android credential related
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
            "85eb6835b0a1034e" => "2ad42749773c441109bdc0191257a664", // unknown // 不能用于获取UserInfo, 会404
            "84956560bc028eb7" => "94aba54af9065f71de72f5508f1cd42e", // unknown // 不能用于获取UserInfo, 会404
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
        // if self.appsec =
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

impl HasIsappIsthUseragent for SearchParams<'_> {
    fn is_app(&self) -> bool {
        self.is_app
    }

    fn is_th(&self) -> bool {
        self.is_th
    }

    fn user_agent(&self) -> &str {
        self.user_agent
    }
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
    pub fn appkey_to_sec(&mut self) -> Result<(), ()> {
        if self.is_th {
            self.appkey = "7d089525d3611b1c";
        }
        self.appsec = match self.appkey {
            "9d5889cf67e615cd" => "8fd9bb32efea8cef801fd895bef2713d", // Ai4cCreatorAndroid
            "1d8b6e7d45233436" => "560c52ccd288fed045859ed18bffd973", // Android
            "783bbb7264451d82" => "2653583c8873dea268ab9386918b1d65", // Android credential related
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
        self.area = area.to_str();
        self.area_num = area.num();
        self.appkey_to_sec().unwrap();
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

/*
* the following is log related struct & impl
*/
// for web panel log, not intend to support currently
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
    UserLoginInvalid,          //用户登录无效
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
                    .timestamp_opt(
                        if timestamp != 0 {
                            timestamp
                        } else {
                            3376656000
                        },
                        0,
                    )
                    .unwrap()
                    .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
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
            EType::UserLoginInvalid => String::from(
                "{{\"code\":-61000,\"message\":\"无效的用户态, 请尝试重新登陆Bilibili\"}}",
            ),
        }
    }
}
