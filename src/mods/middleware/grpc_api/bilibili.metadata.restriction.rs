/// 限制条件
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Restriction {
    /// 青少年模式开关状态
    #[prost(bool, tag = "1")]
    pub teenagers_mode: bool,
    /// 课堂模式开关状态
    #[prost(bool, tag = "2")]
    pub lessons_mode: bool,
    /// 模式类型(旧版)
    #[prost(enumeration = "ModeType", tag = "3")]
    pub mode: i32,
    /// app 审核review状态
    #[prost(bool, tag = "4")]
    pub review: bool,
    /// 客户端是否选择关闭个性化推荐
    #[prost(bool, tag = "5")]
    pub disable_rcmd: bool,
}
/// 模式类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModeType {
    /// 正常模式
    Normal = 0,
    /// 青少年模式
    Teenagers = 1,
    /// 课堂模式
    Lessons = 2,
}
impl ModeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModeType::Normal => "NORMAL",
            ModeType::Teenagers => "TEENAGERS",
            ModeType::Lessons => "LESSONS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "TEENAGERS" => Some(Self::Teenagers),
            "LESSONS" => Some(Self::Lessons),
            _ => None,
        }
    }
}
