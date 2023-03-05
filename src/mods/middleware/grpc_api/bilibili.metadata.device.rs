/// 设备信息
/// gRPC头部:x-bili-device-bin
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// 产品id
    /// 粉 白 蓝 直播姬 HD 海外 OTT 漫画 TV野版 小视频 网易漫画 网易漫画 网易漫画HD 国际版 东南亚版
    /// 1  2  3    4    5   6    7   8     9     10      11       12       13       14       30
    #[prost(int32, tag = "1")]
    pub app_id: i32,
    /// 构建id
    #[prost(int32, tag = "2")]
    pub build: i32,
    /// 设备buvid
    #[prost(string, tag = "3")]
    pub buvid: ::prost::alloc::string::String,
    /// 包类型
    #[prost(string, tag = "4")]
    pub mobi_app: ::prost::alloc::string::String,
    /// 平台类型
    /// ios android
    #[prost(string, tag = "5")]
    pub platform: ::prost::alloc::string::String,
    /// 设备类型
    #[prost(string, tag = "6")]
    pub device: ::prost::alloc::string::String,
    /// 渠道
    #[prost(string, tag = "7")]
    pub channel: ::prost::alloc::string::String,
    /// 手机品牌
    #[prost(string, tag = "8")]
    pub brand: ::prost::alloc::string::String,
    /// 手机型号
    #[prost(string, tag = "9")]
    pub model: ::prost::alloc::string::String,
    /// 系统版本
    #[prost(string, tag = "10")]
    pub osver: ::prost::alloc::string::String,
    /// 本地设备指纹
    #[prost(string, tag = "11")]
    pub fp_local: ::prost::alloc::string::String,
    /// 远程设备指纹
    #[prost(string, tag = "12")]
    pub fp_remote: ::prost::alloc::string::String,
    /// APP版本号
    #[prost(string, tag = "13")]
    pub version_name: ::prost::alloc::string::String,
    /// 设备指纹, 不区分本地或远程设备指纹，作为推送目标的索引
    #[prost(string, tag = "14")]
    pub fp: ::prost::alloc::string::String,
    /// 首次启动时的毫秒时间戳
    #[prost(int64, tag = "15")]
    pub fts: i64,
}
