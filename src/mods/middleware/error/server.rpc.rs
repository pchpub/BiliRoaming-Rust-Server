#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiliUpstreamError {
    #[prost(enumeration = "BiliUpstreamErrorType", tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub details: ::core::option::Option<BiliUpstreamErrorDetails>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiliUpstreamErrorDetails {
    #[prost(int64, tag = "1")]
    pub code: i64,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub raw_query: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub raw_response_body: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "12")]
    pub raw_response_header: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// 错误码
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BiliUpstreamErrorType {
    /// 正常
    Ok = 0,
    /// 应用程序不存在或已被封禁, code = -1
    ApiFatal = 1,
    /// API校验密匙错误, code = -3
    ApiSignInvalid = 2,
    /// Access Key错误, code = -2;
    /// 用户未登录, code = -101;
    /// 使用登录状态访问了，并且登录状态无效，客服端可以／需要删除登录状态, code = 61000
    LoginInvalid = 10,
    /// 账号被封停, code = -102;
    UserIsBanned = 11,
    /// 请求错误, code = -400, 可能是appkey不合法
    /// 请求未授权, code = -401
    ReqInvalid = 20,
    /// 访问权限不足, code = -403
    ReqAccessDenied = 21,
    /// 啥都木有, code = -404
    ReqNotFound = 22,
    /// 请求被拦截, code = -412, 客户端ip服务端风控
    ReqFatal = 23,
    /// 鉴权失败，请联系账号组, code = -663, 即appkey和access_key, mobi_app等不对应
    ReqAppkeyInvalid = 24,
    /// -663, code = -663, 可能是此鉴权相关api已经被弃用
    ReqApiDeprecated = 25,
    /// 被降级过滤的请求, code = -1200, 爬虫限制
    ReqCrawlerLimit = 26,
    /// 服务器错误, code = -500
    ServerInternal = 31,
    /// 过载保护,服务暂不可用, code = -503
    ServerOverload = 32,
    /// 服务调用超时, code = -504
    ServerTimeout = 33,
    /// 处理失败, code = -10500, 确认为DRM限制IP为家宽
    ResDrmLimit = 40,
    /// 大会员专享限制, code = -10403
    /// 开通大会员观看, code = 6002105
    ResVipOnly = 41,
    /// 抱歉您所在地区不可观看！, code = -10403
    /// 抱歉您所在地区不可观看！, code = 6002105
    /// ??? , code = 6010001
    ResAreaLimit = 42,
    /// 抱歉您所使用的平台不可观看！, code = -10403
    ResPlatformLimit = 43,
    /// / 其他
    Unknown = 100,
}
impl BiliUpstreamErrorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BiliUpstreamErrorType::Ok => "Ok",
            BiliUpstreamErrorType::ApiFatal => "ApiFatal",
            BiliUpstreamErrorType::ApiSignInvalid => "ApiSignInvalid",
            BiliUpstreamErrorType::LoginInvalid => "LoginInvalid",
            BiliUpstreamErrorType::UserIsBanned => "UserIsBanned",
            BiliUpstreamErrorType::ReqInvalid => "ReqInvalid",
            BiliUpstreamErrorType::ReqAccessDenied => "ReqAccessDenied",
            BiliUpstreamErrorType::ReqNotFound => "ReqNotFound",
            BiliUpstreamErrorType::ReqFatal => "ReqFatal",
            BiliUpstreamErrorType::ReqAppkeyInvalid => "ReqAppkeyInvalid",
            BiliUpstreamErrorType::ReqApiDeprecated => "ReqApiDeprecated",
            BiliUpstreamErrorType::ReqCrawlerLimit => "ReqCrawlerLimit",
            BiliUpstreamErrorType::ServerInternal => "ServerInternal",
            BiliUpstreamErrorType::ServerOverload => "ServerOverload",
            BiliUpstreamErrorType::ServerTimeout => "ServerTimeout",
            BiliUpstreamErrorType::ResDrmLimit => "ResDrmLimit",
            BiliUpstreamErrorType::ResVipOnly => "ResVipOnly",
            BiliUpstreamErrorType::ResAreaLimit => "ResAreaLimit",
            BiliUpstreamErrorType::ResPlatformLimit => "ResPlatformLimit",
            BiliUpstreamErrorType::Unknown => "Unknown",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Ok" => Some(Self::Ok),
            "ApiFatal" => Some(Self::ApiFatal),
            "ApiSignInvalid" => Some(Self::ApiSignInvalid),
            "LoginInvalid" => Some(Self::LoginInvalid),
            "UserIsBanned" => Some(Self::UserIsBanned),
            "ReqInvalid" => Some(Self::ReqInvalid),
            "ReqAccessDenied" => Some(Self::ReqAccessDenied),
            "ReqNotFound" => Some(Self::ReqNotFound),
            "ReqFatal" => Some(Self::ReqFatal),
            "ReqAppkeyInvalid" => Some(Self::ReqAppkeyInvalid),
            "ReqApiDeprecated" => Some(Self::ReqApiDeprecated),
            "ReqCrawlerLimit" => Some(Self::ReqCrawlerLimit),
            "ServerInternal" => Some(Self::ServerInternal),
            "ServerOverload" => Some(Self::ServerOverload),
            "ServerTimeout" => Some(Self::ServerTimeout),
            "ResDrmLimit" => Some(Self::ResDrmLimit),
            "ResVipOnly" => Some(Self::ResVipOnly),
            "ResAreaLimit" => Some(Self::ResAreaLimit),
            "ResPlatformLimit" => Some(Self::ResPlatformLimit),
            "Unknown" => Some(Self::Unknown),
            _ => None,
        }
    }
}
