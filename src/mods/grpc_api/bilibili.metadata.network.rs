/// 网络类型标识
/// gRPC头部:x-bili-network-bin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// 网络类型
    #[prost(enumeration = "NetworkType", tag = "1")]
    pub r#type: i32,
    /// 免流类型
    #[prost(enumeration = "TfType", tag = "2")]
    pub tf: i32,
    /// 运营商
    #[prost(string, tag = "3")]
    pub oid: ::prost::alloc::string::String,
}
/// 网络类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NetworkType {
    /// 未知
    NtUnknown = 0,
    /// WIFI
    Wifi = 1,
    /// 蜂窝网络
    Cellular = 2,
    /// 未连接
    Offline = 3,
    /// 其他网络
    Othernet = 4,
    /// 以太网
    Ethernet = 5,
}
impl NetworkType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NetworkType::NtUnknown => "NT_UNKNOWN",
            NetworkType::Wifi => "WIFI",
            NetworkType::Cellular => "CELLULAR",
            NetworkType::Offline => "OFFLINE",
            NetworkType::Othernet => "OTHERNET",
            NetworkType::Ethernet => "ETHERNET",
        }
    }
}
/// 免流类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TfType {
    /// 正常计费
    TfUnknown = 0,
    /// 联通卡
    UCard = 1,
    /// 联通包
    UPkg = 2,
    /// 移动卡
    CCard = 3,
    /// 移动包
    CPkg = 4,
    /// 电信卡
    TCard = 5,
    /// 电信包
    TPkg = 6,
}
impl TfType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TfType::TfUnknown => "TF_UNKNOWN",
            TfType::UCard => "U_CARD",
            TfType::UPkg => "U_PKG",
            TfType::CCard => "C_CARD",
            TfType::CPkg => "C_PKG",
            TfType::TCard => "T_CARD",
            TfType::TPkg => "T_PKG",
        }
    }
}
