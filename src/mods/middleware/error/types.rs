use super::definition::*;
use crate::{
    calc_md5,
    mods::types::{ErrorTrait, RawRespTrait},
    unsafe_str_copy,
};
use std::fmt::Display;

macro_rules! get_value_or_default {
    ($key: expr, $value: expr, i64, $default: expr) => {{
        match $value.get($key) {
            Some(value) => value.as_i64().unwrap_or($default),
            None => $default,
        }
    }};
    ($key: expr, $value: expr, &str, $default: expr) => {{
        match $value.get($key) {
            Some(value) => value.as_str().unwrap_or($default),
            None => $default,
        }
    }};
    ($key: expr, $value: expr, bool, $default: expr) => {{
        match $value.get($key) {
            Some(value) => value.as_bool().unwrap_or($default),
            None => $default,
        }
    }};
}

impl<T: RawRespTrait> From<T> for BiliUpstreamError {
    fn from(value: T) -> Self {
        let (code, message, e_type) = match &value.json() {
            Some(value) => {
                let code = get_value_or_default!("code", value, i64, -10403);
                let message = get_value_or_default!("message", value, &str, "Serialization Error");
                let e_type = match code {
                    0 => BiliUpstreamErrorType::Ok,
                    -1 => BiliUpstreamErrorType::ApiFatal,
                    -3 => BiliUpstreamErrorType::ApiSignInvalid,
                    -2 | -101 | 61000 => BiliUpstreamErrorType::LoginInvalid,
                    -102 => BiliUpstreamErrorType::UserIsBanned,
                    -400 | -401 => BiliUpstreamErrorType::ReqInvalid,
                    -403 => BiliUpstreamErrorType::ReqAccessDenied,
                    -404 => BiliUpstreamErrorType::ReqNotFound,
                    -412 => BiliUpstreamErrorType::ReqFatal,
                    -500 => BiliUpstreamErrorType::ServerInternal,
                    -503 => BiliUpstreamErrorType::ServerOverload,
                    -663 => match message {
                        "-663" => BiliUpstreamErrorType::ReqApiDeprecated,
                        "鉴权失败，请联系账号组" => {
                            BiliUpstreamErrorType::ReqAppkeyInvalid
                        }
                        _ => BiliUpstreamErrorType::Unknown,
                    },
                    -10403 => match message {
                        "大会员专享限制" => BiliUpstreamErrorType::ResVipOnly,
                        "抱歉您所使用的平台不可观看！" => {
                            BiliUpstreamErrorType::ResPlatformLimit
                        }
                        "抱歉您所在地区不可观看！" => {
                            BiliUpstreamErrorType::ResAreaLimit
                        }
                        _ => BiliUpstreamErrorType::Unknown,
                    },
                    -10500 => BiliUpstreamErrorType::ResDrmLimit,
                    10015002 => match message {
                        "访问权限不足" => BiliUpstreamErrorType::ResVipOnly,
                        _ => BiliUpstreamErrorType::Unknown,
                    },
                    6002105 => match message {
                        "开通大会员观看" => BiliUpstreamErrorType::ResVipOnly,
                        _ => BiliUpstreamErrorType::Unknown,
                    },
                    // TODO: 补充gRPC的相关错误代码
                    // ! gRPC的错误, 一部分直接返回UNKNOWN, 通过from_grpc处理, 另一部分在dialog弹窗里面. 这里处理dialog弹窗里面的错误码
                    // ! gRPC error code => code: 6002003, msg: "抱歉您所在地区不可观看！", r#type: "area_limit"
                    6002003 => match message {
                        "抱歉您所在地区不可观看！" => {
                            BiliUpstreamErrorType::ResAreaLimit
                        }
                        _ => BiliUpstreamErrorType::Unknown,
                    },
                    // ? api.bilibili.com/pgc/view/v2/app/season -> dialog, which shows area limit info
                    6010001 => BiliUpstreamErrorType::ResAreaLimit,
                    _ => BiliUpstreamErrorType::Unknown,
                };
                (code, message.to_owned(), e_type)
            }
            None => (
                -10403,
                unsafe_str_copy!("Serialization Error"),
                BiliUpstreamErrorType::Unknown,
            ),
        };
        Self {
            r#type: e_type as i32,
            details: Some(BiliUpstreamErrorDetails {
                code,
                message,
                raw_query: value.raw_query().to_owned(),
                raw_response_body: value.raw_resp_content().to_owned(),
                raw_response_header: value.raw_headers().to_owned(),
            }),
        }
    }
}
impl BiliUpstreamError {}

impl ErrorTrait for BiliUpstreamError {
    fn e_code(&self) -> i32 {
        let e_type =
            BiliUpstreamErrorType::from_i32(self.r#type).unwrap_or(BiliUpstreamErrorType::Unknown);
        match e_type {
            BiliUpstreamErrorType::Ok => 0,
            BiliUpstreamErrorType::ApiFatal => -1,
            BiliUpstreamErrorType::ApiSignInvalid => -3,
            BiliUpstreamErrorType::LoginInvalid => -101,
            BiliUpstreamErrorType::UserIsBanned => -102,
            BiliUpstreamErrorType::ReqInvalid => -400,
            BiliUpstreamErrorType::ReqAccessDenied => -403,
            BiliUpstreamErrorType::ReqNotFound => -404,
            BiliUpstreamErrorType::ReqFatal => -412,
            BiliUpstreamErrorType::ReqAppkeyInvalid => -663,
            BiliUpstreamErrorType::ReqApiDeprecated => -663,
            BiliUpstreamErrorType::ReqCrawlerLimit => -1200,
            BiliUpstreamErrorType::ServerInternal => -500,
            BiliUpstreamErrorType::ServerOverload => -503,
            BiliUpstreamErrorType::ServerTimeout => -504,
            BiliUpstreamErrorType::ResDrmLimit => -10500,
            BiliUpstreamErrorType::ResVipOnly => 6002105,
            BiliUpstreamErrorType::ResAreaLimit => 6002105,
            BiliUpstreamErrorType::ResPlatformLimit => -10403,
            BiliUpstreamErrorType::Unknown => -10403,
        }
    }
    fn e_message<'e>(&self) -> &'e str {
        let e_type =
            BiliUpstreamErrorType::from_i32(self.r#type).unwrap_or(BiliUpstreamErrorType::Unknown);
        // match e_type {
        //     BiliUpstreamErrorType::Ok => todo!(),
        //     BiliUpstreamErrorType::ApiFatal => todo!(),
        //     BiliUpstreamErrorType::ApiSignInvalid => todo!(),
        //     BiliUpstreamErrorType::LoginInvalid => todo!(),
        //     BiliUpstreamErrorType::UserIsBanned => todo!(),
        //     BiliUpstreamErrorType::ReqInvalid => todo!(),
        //     BiliUpstreamErrorType::ReqAccessDenied => todo!(),
        //     BiliUpstreamErrorType::ReqNotFound => todo!(),
        //     BiliUpstreamErrorType::ReqFatal => todo!(),
        //     BiliUpstreamErrorType::ReqAppkeyInvalid => todo!(),
        //     BiliUpstreamErrorType::ReqApiDeprecated => todo!(),
        //     BiliUpstreamErrorType::ReqCrawlerLimit => todo!(),
        //     BiliUpstreamErrorType::ServerInternal => todo!(),
        //     BiliUpstreamErrorType::ServerOverload => todo!(),
        //     BiliUpstreamErrorType::ServerTimeout => todo!(),
        //     BiliUpstreamErrorType::ResDrmLimit => todo!(),
        //     BiliUpstreamErrorType::ResVipOnly => todo!(),
        //     BiliUpstreamErrorType::ResAreaLimit => todo!(),
        //     BiliUpstreamErrorType::ResPlatformLimit => todo!(),
        //     BiliUpstreamErrorType::Unknown => todo!(),
        // }
        e_type.as_str_name()
    }
    fn e_trace_id<'e>(&self) -> String {
        match &self.details {
            Some(value) => match value.raw_response_header.get("x-bili-trace-id") {
                Some(value) => unsafe_str_copy!(value),
                None => {
                    calc_md5!(&value.raw_query)
                }
            },
            None => calc_md5!("unknown"),
        }
    }
}

impl Display for BiliUpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.e_code();
        let message = self.e_message();
        let id = self.e_trace_id();
        write!(
            f,
            "{{\"code\": {}, \"message\": \"上游错误: {}\nTraceID: {}\nTips: 大概率鼠鼠的锅, 请稍后重试\", \"ttl\": 1}}",
            code, message, id
        )
    }
}
