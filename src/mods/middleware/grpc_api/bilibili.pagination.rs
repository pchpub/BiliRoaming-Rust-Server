/// 分页信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedPagination {
    ///
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    ///
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub is_refresh: bool,
}
/// 分页信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedPaginationReply {
    ///
    #[prost(string, tag = "1")]
    pub next_offset: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub prev_offset: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub last_read_offset: ::prost::alloc::string::String,
}
/// 分页信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    ///
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    ///
    #[prost(string, tag = "2")]
    pub next: ::prost::alloc::string::String,
}
/// 分页信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginationReply {
    ///
    #[prost(string, tag = "1")]
    pub next: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub prev: ::prost::alloc::string::String,
}
