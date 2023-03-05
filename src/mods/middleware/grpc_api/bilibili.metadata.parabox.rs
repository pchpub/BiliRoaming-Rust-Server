///
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exp {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub bucket: i32,
}
///
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exps {
    ///
    #[prost(message, repeated, tag = "1")]
    pub exps: ::prost::alloc::vec::Vec<Exp>,
}
