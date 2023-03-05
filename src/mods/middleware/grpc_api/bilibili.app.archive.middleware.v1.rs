/// 视频秒开参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerArgs {
    /// 清晰度
    #[prost(int64, tag = "1")]
    pub qn: i64,
    /// 流版本
    #[prost(int64, tag = "2")]
    pub fnver: i64,
    /// 流类型
    #[prost(int64, tag = "3")]
    pub fnval: i64,
    /// 返回url是否强制使用域名
    /// 0:不强制使用域名 1:http域名 2:https域名
    #[prost(int64, tag = "4")]
    pub force_host: i64,
    /// 音量均衡
    #[prost(int64, tag = "5")]
    pub voice_balance: i64,
}
