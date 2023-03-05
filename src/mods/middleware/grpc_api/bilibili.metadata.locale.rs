/// 区域标识
/// gRPC头部:x-bili-locale-bin
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Locale {
    /// App设置的locale
    #[prost(message, optional, tag = "1")]
    pub c_locale: ::core::option::Option<LocaleIds>,
    /// 系统默认的locale
    #[prost(message, optional, tag = "2")]
    pub s_locale: ::core::option::Option<LocaleIds>,
    /// sim卡的国家码+运营商码
    #[prost(string, tag = "3")]
    pub sim_code: ::prost::alloc::string::String,
    /// 时区
    #[prost(string, tag = "4")]
    pub timezone: ::prost::alloc::string::String,
}
/// Defined by <https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPInternational/LanguageandLocaleIDs/LanguageandLocaleIDs.html>
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocaleIds {
    /// A language designator is a code that represents a language.
    #[prost(string, tag = "1")]
    pub language: ::prost::alloc::string::String,
    /// Writing systems.
    #[prost(string, tag = "2")]
    pub script: ::prost::alloc::string::String,
    /// A region designator is a code that represents a country or an area.
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
}
