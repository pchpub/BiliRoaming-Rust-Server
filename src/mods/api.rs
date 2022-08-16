use super::get_bili_res::get_resign_accesskey;
use super::types::{BiliConfig, SendData};
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use async_channel::Sender;
use deadpool_redis::Pool;
use qstring::QString;
use std::sync::Arc;

pub async fn get_api_accesskey(req: &HttpRequest) -> HttpResponse {
    let (pool, config, _bilisender) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<SendData>>)>()
        .unwrap();
    let query_string = req.query_string();
    let query = QString::from(query_string);
    let area_num: u8 = match query.get("area_num") {
        Option::Some(key) => key.parse().unwrap(),
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("From", "biliroaming-rust-server"))
                .body("{\"code\":-2403,\"message\":\"area_num为空\"}");
        }
    };

    match query.get("sign") {
        Option::Some(key) => {
            if key != &config.api_sign {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .body("{\"code\":-2403,\"message\":\"sign错误\"}");
            }
        }
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("From", "biliroaming-rust-server"))
                .body("{\"code\":-2403,\"message\":\"sign为空\"}");
        }
    };
    
    let user_agent = "User-Agent:Mozilla/5.0 (Linux; Android 4.1.2; Nexus 7 Build/JZ054K) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Safari/535.19";

    let access_key = get_resign_accesskey(&pool,&area_num,user_agent,&config).await.unwrap();

    HttpResponse::Ok() // Debug
        .content_type(ContentType::json())
        .insert_header(("From", "biliroaming-rust-server"))
        .body(format!("{{\"code\":0,\"message\":\"\",\"access_key\":\"{access_key}\"}}"))

}
