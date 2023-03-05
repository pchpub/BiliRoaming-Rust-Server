#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiliBangumiInfoReq {
    #[prost(uint64, repeated, tag = "1")]
    pub epid: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "2")]
    pub season_id: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiliSeasonInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub season_info: ::prost::alloc::vec::Vec<BiliSeasonInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiliEpInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub ep_info: ::prost::alloc::vec::Vec<BiliEpInfo>,
}
/// season_info
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiliSeasonInfo {
    /// 番剧系列标题
    #[prost(string, tag = "1")]
    pub season_title: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub season_id: u64,
    #[prost(uint64, tag = "3")]
    pub media_id: u64,
    #[prost(int64, tag = "4")]
    pub update_time: i64,
    #[prost(message, repeated, tag = "5")]
    pub episodes: ::prost::alloc::vec::Vec<BiliEpInfo>,
    /// 服务端最后更新时间(毫秒时间戳)
    #[prost(int64, tag = "6")]
    pub last_updated: i64,
}
/// ep_info
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiliEpInfo {
    #[prost(uint64, tag = "1")]
    pub aid: u64,
    #[prost(uint64, tag = "2")]
    pub cid: u64,
    #[prost(string, tag = "3")]
    pub avid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub bvid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub ep_id: u64,
    #[prost(bool, tag = "6")]
    pub need_vip: bool,
    #[prost(uint64, tag = "7")]
    pub title: u64,
    /// 服务端最后更新时间(毫秒时间戳)
    #[prost(int64, tag = "8")]
    pub last_updated: i64,
}
/// Generated client implementations.
pub mod bangumi_info_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 番剧信息
    #[derive(Debug, Clone)]
    pub struct BangumiInfoClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BangumiInfoClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BangumiInfoClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BangumiInfoClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BangumiInfoClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn season_info(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::BiliBangumiInfoReq,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::BiliSeasonInfoReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/server.api.public.bangumi.BangumiInfo/SeasonInfo",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn ep_info(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::BiliBangumiInfoReq,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::BiliEpInfoReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/server.api.public.bangumi.BangumiInfo/EpInfo",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod bangumi_info_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BangumiInfoServer.
    #[async_trait]
    pub trait BangumiInfo: Send + Sync + 'static {
        /// Server streaming response type for the SeasonInfo method.
        type SeasonInfoStream: futures_core::Stream<
                Item = Result<super::BiliSeasonInfoReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn season_info(
            &self,
            request: tonic::Request<tonic::Streaming<super::BiliBangumiInfoReq>>,
        ) -> Result<tonic::Response<Self::SeasonInfoStream>, tonic::Status>;
        /// Server streaming response type for the EpInfo method.
        type EpInfoStream: futures_core::Stream<
                Item = Result<super::BiliEpInfoReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn ep_info(
            &self,
            request: tonic::Request<tonic::Streaming<super::BiliBangumiInfoReq>>,
        ) -> Result<tonic::Response<Self::EpInfoStream>, tonic::Status>;
    }
    /// 番剧信息
    #[derive(Debug)]
    pub struct BangumiInfoServer<T: BangumiInfo> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BangumiInfo> BangumiInfoServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BangumiInfoServer<T>
    where
        T: BangumiInfo,
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
                "/server.api.public.bangumi.BangumiInfo/SeasonInfo" => {
                    #[allow(non_camel_case_types)]
                    struct SeasonInfoSvc<T: BangumiInfo>(pub Arc<T>);
                    impl<
                        T: BangumiInfo,
                    > tonic::server::StreamingService<super::BiliBangumiInfoReq>
                    for SeasonInfoSvc<T> {
                        type Response = super::BiliSeasonInfoReply;
                        type ResponseStream = T::SeasonInfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::BiliBangumiInfoReq>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).season_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SeasonInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/server.api.public.bangumi.BangumiInfo/EpInfo" => {
                    #[allow(non_camel_case_types)]
                    struct EpInfoSvc<T: BangumiInfo>(pub Arc<T>);
                    impl<
                        T: BangumiInfo,
                    > tonic::server::StreamingService<super::BiliBangumiInfoReq>
                    for EpInfoSvc<T> {
                        type Response = super::BiliEpInfoReply;
                        type ResponseStream = T::EpInfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::BiliBangumiInfoReq>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ep_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EpInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
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
    impl<T: BangumiInfo> Clone for BangumiInfoServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BangumiInfo> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BangumiInfo> tonic::server::NamedService for BangumiInfoServer<T> {
        const NAME: &'static str = "server.api.public.bangumi.BangumiInfo";
    }
}
