/// 响应gRPC Status
/// 当status code是[UNKNOWN = 2]时，details为业务详细的错误信息，进行proto any转换成业务码结构体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// 业务错误码
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// 业务错误信息
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// 扩展信息嵌套(相当于该messasge的套娃)
    #[prost(message, repeated, tag = "3")]
    pub details: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
