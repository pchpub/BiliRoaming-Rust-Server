// use super::get_user_info::{appkey_to_sec, auth_user, getuser_list};
// use super::types::BiliConfig;
// use super::types::LogPlayUrl;
// use actix_web::{HttpRequest, HttpResponse};
// use chrono::prelude::*;
// use deadpool_redis::Pool;
// use qstring::QString;
// use serde_json::{self, json};
// use std::sync::Arc;

// //查询数据+地区（1位）+类型（2位）+版本（2位）
// //查询数据 a asscesskey
// //        e epid
// //        c cid
// //        v is_vip
// //        t is_tv
// //地区 cn 1
// //     hk 2
// //     tw 3
// //     th 4
// //     default 2
// //类型 app playurl 01
// //     app search 02
// //     app subtitle 03
// //     app season 04
// //     user_info 05
// //     user_cerinfo 06
// //     web playurl 07
// //     web search 08
// //     web subtitle 09
// //     web season 10
// //     resign_info 11
// //     api 12
// //     health 13 eg. 0141301 = playurl th health ver.1
// //     ep_area 14
// //     statistics 20
// //版本 ：用于处理版本更新后导致的格式变更
// //     now 01

// pub async fn log_playurl(req: &HttpRequest, is_app: bool, is_th: bool) {
//     // log_info
//     let req_clone = req.clone();
//     let req = &req_clone;
//     let (pool, config) = req.app_data::<(Pool, BiliConfig)>().unwrap();
//     let query_string = req.query_string();
//     let query = QString::from(query_string);
//     // get client ip from header x-real-ip or peer ip
//     let client_ip: String = match req.headers().get("X-Real-IP") {
//         Some(value) => value.to_str().unwrap().to_owned(),
//         None => format!("{:?}", req.peer_addr()),
//     };
//     // get actual ua
//     let user_agent = match req.headers().get("user-agent") {
//         Option::Some(ua) => req
//             .headers()
//             .get("user-agent")
//             .unwrap()
//             .to_str()
//             .unwrap()
//             .to_string(),
//         _ => "".to_string(),
//     };
//     // get appkey
//     let mut appkey = match query.get("appkey") {
//         Option::Some(key) => key,
//         _ => "1d8b6e7d45233436",
//     };
//     // get appsec
//     let mut appsec = match appkey_to_sec(appkey) {
//         Ok(value) => value,
//         Err(()) => {
//             log_invalid(
//                 &client_ip,
//                 &user_agent,
//                 query_string,
//                 is_app,
//                 is_th,
//                 "no valid appsec",
//             )
//             .await;
//             return;
//         }
//     };

//     // get access key
//     let mut access_key = match query.get("access_key") {
//         Option::Some(key) => key.to_string(),
//         _ => "".to_string(),
//     };

//     if access_key.len() == 0 {
//         log_invalid(
//             &client_ip,
//             &user_agent,
//             query_string,
//             is_app,
//             is_th,
//             "no valid accesskey",
//         )
//         .await;
//         return;
//     }

//     let ep_id = match query.get("ep_id") {
//         Option::Some(key) => Some(key),
//         _ => None,
//     };

//     let cid = match query.get("cid") {
//         Option::Some(key) => Some(key),
//         _ => None,
//     };

//     let season_id = match query.get("season_id") {
//         Option::Some(key) => Some(key),
//         _ => None,
//     };

//     let user_info =
//         match getuser_list(pool, &access_key, appkey, &appsec, &user_agent, &config).await {
//             Ok(value) => value,
//             Err(value) => {
//                 log_invalid(&client_ip, &user_agent, query_string, is_app, is_th, &value).await;
//                 return;
//             }
//         };

//     let log: LogPlayUrl = LogPlayUrl {
//         ts: Local::now().timestamp_millis(),
//         ip: client_ip,
//         uid: user_info.uid,
//         access_key: todo!(),
//         season_id: todo!(),
//         ep_id: todo!(),
//         area_num,
//     };
// }

// pub async fn log_users() {
//     // 记录访客
// }

// pub async fn log_health() {
//     // log health info
// }

// pub async fn log_invalid(
//     client_ip: &str,
//     client_ua: &str,
//     client_req_string: &str,
//     is_app: bool,
//     is_th: bool,
//     invalid_reason: &str,
// ) {
// }
// pub async fn get_access_log(uid: u64) -> Result<u64, ()> {}
// async fn generate_status_page() {
//     // generate status page
// }
