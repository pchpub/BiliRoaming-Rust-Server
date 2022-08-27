use actix_governor::KeyExtractor;
use actix_web::{dev::ServiceRequest, http::header::ContentType};
use governor::clock::{Clock, DefaultClock};
use qstring::QString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BiliUserToken;

impl KeyExtractor for BiliUserToken {
    type Key = String;
    type KeyExtractionError = &'static str;

    #[cfg(feature = "log")]
    fn name(&self) -> &'static str {
        "BiliUserToken"
    }

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let key = match QString::from(req.query_string()).get("access_key") {
            Option::Some(key) => key.to_string(),
            _ => {
                match req.headers().get("X-Real-IP") {
                    Some(value) => value.to_str().unwrap().to_owned(),
                    None => format!("{:?}",req.peer_addr()),
                }
            },
             //req.headers().get("X-Real-IP").unwrap().to_str().unwrap().to_owned(),
        };
        Ok(key)
    }

    fn response_error_content(
        &self,
        negative: &governor::NotUntil<governor::clock::QuantaInstant>,
    ) -> (String, ContentType) {
        let wait_time = negative
            .wait_time_from(DefaultClock::default().now())
            .as_secs();
        let json_response = format!(
            r#"{{"code":-429,"message":"请求过快,请{wait_time}后重试"}}"#
        );
        (json_response, ContentType::json())
    }

    fn response_error(&self, err: &'static str) -> actix_web::Error {
        //这个不会用到,不管了
        actix_web::error::ErrorUnauthorized(err.to_string())
    }

    #[cfg(feature = "log")]
    fn key_name(&self, key: &Self::Key) -> Option<String> {
        None
    }
}
