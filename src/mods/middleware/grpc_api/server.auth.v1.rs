/// 用户鉴权
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfo {
    /// 用户uid
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    /// 登录access_key
    #[prost(string, tag = "2")]
    pub access_key: ::prost::alloc::string::String,
    /// 哔哩哔哩客户端信息
    #[prost(message, optional, tag = "3")]
    pub app_info: ::core::option::Option<ClientInfoApp>,
    /// 用户设备信息
    #[prost(message, optional, tag = "4")]
    pub device_info: ::core::option::Option<ClientInfoDevice>,
    /// sso登录信息
    #[prost(message, optional, tag = "5")]
    pub extra_info: ::core::option::Option<ClientUserInfoExtra>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthReply {
    /// 响应代码
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// 提示信息
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// 哔哩哔哩客户端信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfoApp {
    /// app版本号, 如`7.8.2`
    #[prost(string, tag = "1")]
    pub app_ver: ::prost::alloc::string::String,
    /// app build, 如`7082100`
    #[prost(int32, tag = "2")]
    pub app_build: i32,
    /// app innerVer
    #[prost(int32, tag = "3")]
    pub app_build_inner: i32,
    /// app类型, 不提供则默认为大陆版
    #[prost(enumeration = "ClientAppType", tag = "4")]
    pub app_type: i32,
    /// app来源, 不提供则默认为master
    #[prost(string, tag = "5")]
    pub app_channel: ::prost::alloc::string::String,
    /// app包类型
    #[prost(string, tag = "6")]
    pub mobi_app: ::prost::alloc::string::String,
    /// appkey
    #[prost(string, tag = "7")]
    pub appkey: ::prost::alloc::string::String,
    /// 其他非必要信息
    #[prost(message, optional, tag = "10")]
    pub app_extra: ::core::option::Option<ClientInfoAppExtra>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfoAppExtra {
    /// 产品编号，由数据平台分配，粉=1，白=2，蓝=3，直播姬=4，HD=5，海外=6，OTT=7，漫画=8，TV野版=9，小视频=10，网易漫画=11，网易漫画lite=12，网易漫画HD=13,国际版=14
    #[prost(uint32, tag = "1")]
    pub app_id: u32,
}
/// 用户设备信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfoDevice {
    /// 平台类型
    #[prost(string, tag = "1")]
    pub platform: ::prost::alloc::string::String,
    /// AndroidId
    #[prost(string, tag = "2")]
    pub android_id: ::prost::alloc::string::String,
    /// DrmId
    #[prost(string, tag = "3")]
    pub drm_id: ::prost::alloc::string::String,
    /// 硬件指纹
    #[prost(string, tag = "4")]
    pub fp: ::prost::alloc::string::String,
    /// 设备其他信息
    #[prost(message, optional, tag = "10")]
    pub device_extra: ::core::option::Option<ClientInfoDeviceExtra>,
}
/// 用户设备其他信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfoDeviceExtra {
    /// dalvik版本, 安卓独有字段, 默认`2.1.0``
    #[prost(string, tag = "1")]
    pub dalvik_ver: ::prost::alloc::string::String,
    /// 系统版本, 安卓默认`11``, ios默认`16.1`即可
    #[prost(string, tag = "2")]
    pub os_ver: ::prost::alloc::string::String,
    /// 手机品牌, 随机生成
    #[prost(string, tag = "3")]
    pub brand: ::prost::alloc::string::String,
    /// 手机型号, 随机生成
    #[prost(string, tag = "4")]
    pub model: ::prost::alloc::string::String,
    /// wifi, default
    #[prost(uint32, tag = "5")]
    pub network_type: u32,
    /// 免流类型, 保持默认为0, 即不免流.
    #[prost(uint32, tag = "6")]
    pub network_tf: u32,
    /// 运营商代码, 留空
    #[prost(string, tag = "7")]
    pub network_oid: ::prost::alloc::string::String,
    /// 区域, 默认为`CN`即可
    #[prost(string, tag = "8")]
    pub region: ::prost::alloc::string::String,
    /// 语言, 默认为`zh`即可
    #[prost(string, tag = "9")]
    pub language: ::prost::alloc::string::String,
}
/// 用户信息, v2
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUserInfo {
    /// 用户uid
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    /// 用户access_key
    #[prost(string, tag = "2")]
    pub access_key: ::prost::alloc::string::String,
    /// 首次访问时间
    #[prost(int64, tag = "3")]
    pub fts: i64,
    /// 访问次数
    #[prost(uint32, tag = "4")]
    pub access_count: u32,
    /// buvid, used for general purpose
    #[prost(string, tag = "5")]
    pub buvid: ::prost::alloc::string::String,
    /// buvid, used for auth related
    #[prost(string, tag = "6")]
    pub buvid_auth: ::prost::alloc::string::String,
    /// 大会员过期时间
    #[prost(int64, tag = "7")]
    pub vip_expire_time: i64,
    /// 缓存过期时间
    #[prost(int64, tag = "8")]
    pub expire_time: i64,
    /// 漫游黑名单
    #[prost(message, optional, tag = "100")]
    pub user_cer_info: ::core::option::Option<ClientUserCerInfo>,
}
/// sso登录信息, 可选, 自建服务器用, 供服务器自行刷新access_key
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUserInfoExtra {
    #[prost(string, tag = "1")]
    pub dede_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dede_user_id_ck_md5: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bili_jct: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub sessdata: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub refresh_token: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub expires: u64,
}
/// 漫游黑名单
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUserCerInfo {
    #[prost(bool, tag = "1")]
    pub is_whitelisted: bool,
    #[prost(bool, tag = "2")]
    pub is_blacklisted: bool,
    #[prost(int64, tag = "3")]
    pub ban_until: i64,
    #[prost(int64, tag = "4")]
    pub last_updated: i64,
}
/// 哔哩哔哩客户端类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientAppType {
    /// 大陆版(粉版)
    Android = 0,
    /// play版
    AndroidI = 1,
    /// 概念版(蓝版)
    AndroidB = 2,
    /// HD版
    AndroidHd = 3,
    /// 东南亚版
    BstarA = 4,
    /// 云视听小电视(TV版)
    AndroidTv = 5,
    /// IOS版通用
    Ios = 6,
    /// PC的electron版本
    Pc = 10,
    /// 其他未知
    Unknown = 999,
}
impl ClientAppType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientAppType::Android => "Android",
            ClientAppType::AndroidI => "AndroidI",
            ClientAppType::AndroidB => "AndroidB",
            ClientAppType::AndroidHd => "AndroidHd",
            ClientAppType::BstarA => "BstarA",
            ClientAppType::AndroidTv => "AndroidTv",
            ClientAppType::Ios => "Ios",
            ClientAppType::Pc => "Pc",
            ClientAppType::Unknown => "UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Android" => Some(Self::Android),
            "AndroidI" => Some(Self::AndroidI),
            "AndroidB" => Some(Self::AndroidB),
            "AndroidHd" => Some(Self::AndroidHd),
            "BstarA" => Some(Self::BstarA),
            "AndroidTv" => Some(Self::AndroidTv),
            "Ios" => Some(Self::Ios),
            "Pc" => Some(Self::Pc),
            "UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod auth_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuthServer.
    #[async_trait]
    pub trait Auth: Send + Sync + 'static {
        /// 用户登录
        async fn auth(
            &self,
            request: tonic::Request<super::ClientInfo>,
        ) -> Result<tonic::Response<super::AuthReply>, tonic::Status>;
    }
    /// 视频url
    #[derive(Debug)]
    pub struct AuthServer<T: Auth> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Auth> AuthServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthServer<T>
    where
        T: Auth,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/server.auth.v1.Auth/Auth" => {
                    #[allow(non_camel_case_types)]
                    struct AuthSvc<T: Auth>(pub Arc<T>);
                    impl<T: Auth> tonic::server::UnaryService<super::ClientInfo>
                    for AuthSvc<T> {
                        type Response = super::AuthReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClientInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).auth(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Auth> Clone for AuthServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Auth> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Auth> tonic::server::NamedService for AuthServer<T> {
        const NAME: &'static str = "server.auth.v1.Auth";
    }
}
