use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
use actix_web::{dev::ServiceRequest, http::header::ContentType};
// use governor::clock::{Clock, DefaultClock};
use qstring::QString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BiliUserToken;

impl KeyExtractor for BiliUserToken {
    type Key = String;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

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

    fn exceed_rate_limit_response(
        &self,
        negative:&actix_governor::governor::NotUntil<actix_governor::governor::clock::QuantaInstant>,
        mut response: actix_web::HttpResponseBuilder,
    ) -> actix_web::HttpResponse {
        let wait_time = negative
            .wait_time_from(actix_governor::governor::clock::Clock::now(&actix_governor::governor::clock::DefaultClock::default()))
            .as_secs();
        response
            .content_type(ContentType::json())
            .body(format!(
                r#"{{"code":-429,"message":"请求过快,请{wait_time}后重试"}}"#
        ))
    }
}
