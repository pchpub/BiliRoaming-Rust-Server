/// 请求元数据
/// gRPC头部:x-bili-metadata-bin
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// 登录Token
    #[prost(string, tag = "1")]
    pub access_key: ::prost::alloc::string::String,
    /// 包类型
    #[prost(string, tag = "2")]
    pub mobi_app: ::prost::alloc::string::String,
    /// 运行设备
    #[prost(string, tag = "3")]
    pub device: ::prost::alloc::string::String,
    /// 构建id
    #[prost(int32, tag = "4")]
    pub build: i32,
    /// APP分发渠道
    #[prost(string, tag = "5")]
    pub channel: ::prost::alloc::string::String,
    /// 设备buvid
    #[prost(string, tag = "6")]
    pub buvid: ::prost::alloc::string::String,
    /// 平台类型
    #[prost(string, tag = "7")]
    pub platform: ::prost::alloc::string::String,
}
