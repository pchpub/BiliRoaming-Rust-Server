/// 其他业务信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusinessInfo {
    /// 当前视频是否是预览
    #[prost(bool, tag = "1")]
    pub is_preview: bool,
    /// 用户是否承包过
    #[prost(bool, tag = "2")]
    pub bp: bool,
    /// drm使用
    #[prost(string, tag = "3")]
    pub marlin_token: ::prost::alloc::string::String,
}
/// 事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// 震动
    #[prost(message, optional, tag = "1")]
    pub shake: ::core::option::Option<Shake>,
}
/// 播放信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LivePlayInfo {
    ///
    #[prost(int32, tag = "1")]
    pub current_qn: i32,
    ///
    #[prost(message, repeated, tag = "2")]
    pub quality_description: ::prost::alloc::vec::Vec<QualityDescription>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub durl: ::prost::alloc::vec::Vec<ResponseDataUrl>,
}
/// 直播播放页信息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LivePlayViewReply {
    /// 房间信息
    #[prost(message, optional, tag = "1")]
    pub room_info: ::core::option::Option<RoomInfo>,
    /// 播放信息
    #[prost(message, optional, tag = "2")]
    pub play_info: ::core::option::Option<LivePlayInfo>,
}
/// 直播播放页信息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LivePlayViewReq {
    /// 剧集epid
    #[prost(int64, tag = "1")]
    pub ep_id: i64,
    /// 清晰度
    /// 0,10000:原画 400:蓝光 250:超清 150:高清 80:流畅
    #[prost(uint32, tag = "2")]
    pub quality: u32,
    /// 类型
    /// 0:音频 2:hevc 4:dash 8:p2p, 16:蒙版
    #[prost(uint32, tag = "3")]
    pub ptype: u32,
    /// 是否请求https
    #[prost(bool, tag = "4")]
    pub https: bool,
    /// 0:默认直播间播放 1:投屏播放
    #[prost(uint32, tag = "5")]
    pub play_type: u32,
    /// 投屏设备
    /// 0:默认其他 1:OTT设备
    #[prost(int32, tag = "6")]
    pub device_type: i32,
}
/// 禁用功能配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayAbilityConf {
    /// 后台播放
    #[prost(bool, tag = "1")]
    pub background_play_disable: bool,
    /// 镜像反转
    #[prost(bool, tag = "2")]
    pub flip_disable: bool,
    /// 投屏
    #[prost(bool, tag = "3")]
    pub cast_disable: bool,
    /// 反馈
    #[prost(bool, tag = "4")]
    pub feedback_disable: bool,
    /// 字幕
    #[prost(bool, tag = "5")]
    pub subtitle_disable: bool,
    /// 播放速度
    #[prost(bool, tag = "6")]
    pub playback_rate_disable: bool,
    /// 定时停止
    #[prost(bool, tag = "7")]
    pub time_up_disable: bool,
    /// 播放方式
    #[prost(bool, tag = "8")]
    pub playback_mode_disable: bool,
    /// 画面尺寸
    #[prost(bool, tag = "9")]
    pub scale_mode_disable: bool,
    /// 赞
    #[prost(bool, tag = "10")]
    pub like_disable: bool,
    /// 踩
    #[prost(bool, tag = "11")]
    pub dislike_disable: bool,
    /// 投币
    #[prost(bool, tag = "12")]
    pub coin_disable: bool,
    /// 充电
    #[prost(bool, tag = "13")]
    pub elec_disable: bool,
    /// 分享
    #[prost(bool, tag = "14")]
    pub share_disable: bool,
    /// 截图
    #[prost(bool, tag = "15")]
    pub screen_shot_disable: bool,
    /// 锁定
    #[prost(bool, tag = "16")]
    pub lock_screen_disable: bool,
    /// 相关推荐
    #[prost(bool, tag = "17")]
    pub recommend_disable: bool,
    /// 播放速度
    #[prost(bool, tag = "18")]
    pub playback_speed_disable: bool,
    /// 清晰度
    #[prost(bool, tag = "19")]
    pub definition_disable: bool,
    /// 选集
    #[prost(bool, tag = "20")]
    pub selections_disable: bool,
    /// 下一集
    #[prost(bool, tag = "21")]
    pub next_disable: bool,
    /// 编辑弹幕
    #[prost(bool, tag = "22")]
    pub edit_dm_disable: bool,
    /// 小窗
    #[prost(bool, tag = "23")]
    pub small_window_disable: bool,
    /// 震动
    #[prost(bool, tag = "24")]
    pub shake_disable: bool,
    /// 外层面板弹幕设置
    #[prost(bool, tag = "25")]
    pub outer_dm_disable: bool,
    /// 三点内弹幕设置
    #[prost(bool, tag = "26")]
    pub inner_dm_disable: bool,
    /// 一起看入口
    #[prost(bool, tag = "27")]
    pub freya_enter_disable: bool,
    /// 杜比音效
    #[prost(bool, tag = "28")]
    pub dolby_disable: bool,
    /// 全屏一起看入口
    #[prost(bool, tag = "29")]
    pub freya_full_disable: bool,
    ///
    #[prost(bool, tag = "30")]
    pub skip_oped_switch_disable: bool,
}
///   播放页信息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewReply {
    /// 视频流信息
    #[prost(message, optional, tag = "1")]
    pub video_info: ::core::option::Option<
        super::super::super::super::app::playurl::v1::VideoInfo,
    >,
    /// 播放控件用户自定义配置
    #[prost(message, optional, tag = "2")]
    pub play_conf: ::core::option::Option<PlayAbilityConf>,
    /// 业务需要的其他信息
    #[prost(message, optional, tag = "3")]
    pub business: ::core::option::Option<BusinessInfo>,
    /// 事件
    #[prost(message, optional, tag = "4")]
    pub event: ::core::option::Option<Event>,
}
/// 播放页信息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewReq {
    /// 剧集epid
    #[prost(int64, tag = "1")]
    pub epid: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 清晰度
    #[prost(int64, tag = "3")]
    pub qn: i64,
    /// 视频流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流格式
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 下载模式
    /// 0:播放 1:flv下载 2:dash下载
    #[prost(uint32, tag = "6")]
    pub download: u32,
    /// 流url强制是用域名
    /// 0:允许使用ip 1:使用http 2:使用https
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4K
    #[prost(bool, tag = "8")]
    pub fourk: bool,
    /// 当前页spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 青少年模式
    #[prost(int32, tag = "11")]
    pub teenagers_mode: i32,
    /// 视频编码
    #[prost(
        enumeration = "super::super::super::super::app::playurl::v1::CodeType",
        tag = "12"
    )]
    pub prefer_codec_type: i32,
    /// 是否强制请求预览视频
    #[prost(bool, tag = "13")]
    pub is_preview: bool,
    /// 一起看房间id
    #[prost(int64, tag = "14")]
    pub room_id: i64,
}
/// 投屏地址-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectReply {
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<
        super::super::super::super::app::playurl::v1::PlayUrlReply,
    >,
}
/// 投屏地址-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectReq {
    /// 剧集epid
    #[prost(int64, tag = "1")]
    pub ep_id: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 清晰度
    #[prost(int64, tag = "3")]
    pub qn: i64,
    /// 视频流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流格式
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 下载模式
    /// 0:播放 1:flv下载 2:dash下载
    #[prost(uint32, tag = "6")]
    pub download: u32,
    /// 流url强制是用域名
    /// 0:允许使用ip 1:使用http 2:使用https
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4K
    #[prost(bool, tag = "8")]
    pub fourk: bool,
    /// 当前页spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 使用协议
    /// 0:默认乐播 1:自建协议 2:云投屏 3:airplay
    #[prost(int32, tag = "11")]
    pub protocol: i32,
    /// 投屏设备
    /// 0:默认其他 1:OTT设备
    #[prost(int32, tag = "12")]
    pub device_type: i32,
    ///
    #[prost(bool, tag = "13")]
    pub use_new_project_code: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityDescription {
    ///
    #[prost(int32, tag = "1")]
    pub qn: i32,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDataUrl {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// 表示stream类型,按位表示
    /// Value|  1   |  1  |  1   |  1   |     1
    /// --------------------------------------------
    /// desc | mask | p2p | dash | hevc | only-audio
    #[prost(uint32, tag = "2")]
    pub stream_type: u32,
    /// 表示支持p2p的cdn厂商,按位表示
    /// 值   | 1  |  1  |  1  | 1  |  1  | 1  | 1  | 1
    /// -----------------------------------------------
    /// CDN	| hw | bdy | bsy | ws | txy | qn | js | bvc
    #[prost(uint32, tag = "3")]
    pub ptag: u32,
}
/// 房间信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomInfo {
    /// 房间长号
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    /// 主播mid
    #[prost(int64, tag = "2")]
    pub uid: i64,
    /// 状态相关
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<RoomStatusInfo>,
    /// 展示相关
    #[prost(message, optional, tag = "4")]
    pub show: ::core::option::Option<RoomShowInfo>,
}
/// 房间信息-展示相关
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomShowInfo {
    /// 短号
    #[prost(int64, tag = "1")]
    pub short_id: i64,
    /// 人气值
    #[prost(int64, tag = "8")]
    pub popularity_count: i64,
    /// 最近一次开播时间戳
    #[prost(int64, tag = "10")]
    pub live_start_time: i64,
}
/// 房间信息-状态相关
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomStatusInfo {
    /// 直播间状态
    /// 0:未开播 1:直播中 2:轮播中
    #[prost(int64, tag = "1")]
    pub live_status: i64,
    /// 横竖屏方向
    /// 0:横屏 1:竖屏
    #[prost(int64, tag = "2")]
    pub live_screen_type: i64,
    /// 是否开播过标识
    #[prost(int64, tag = "3")]
    pub live_mark: i64,
    /// 封禁状态
    /// 0:未封禁 1:审核封禁 2:全网封禁
    #[prost(int64, tag = "4")]
    pub lock_status: i64,
    /// 封禁时间戳
    #[prost(int64, tag = "5")]
    pub lock_time: i64,
    /// 隐藏状态
    /// 0:不隐藏 1:隐藏
    #[prost(int64, tag = "6")]
    pub hidden_status: i64,
    /// 隐藏时间戳
    #[prost(int64, tag = "7")]
    pub hidden_time: i64,
    /// 直播类型
    /// 0:默认 1:摄像头直播 2;录屏直播 3:语音直播
    #[prost(int64, tag = "8")]
    pub live_type: i64,
    ///
    #[prost(int64, tag = "9")]
    pub room_shield: i64,
}
/// 震动
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shake {
    /// 文件地址
    #[prost(string, tag = "1")]
    pub file: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod play_url_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 播放地址
    #[derive(Debug, Clone)]
    pub struct PlayUrlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayUrlClient<tonic::transport::Channel> {
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
    impl<T> PlayUrlClient<T>
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
        ) -> PlayUrlClient<InterceptedService<T, F>>
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
            PlayUrlClient::new(InterceptedService::new(inner, interceptor))
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
        /// 播放页信息
        pub async fn play_view(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayViewReq>,
        ) -> Result<tonic::Response<super::PlayViewReply>, tonic::Status> {
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
                "/bilibili.pgc.gateway.player.v1.PlayURL/PlayView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 获取投屏地址
        pub async fn project(
            &mut self,
            request: impl tonic::IntoRequest<super::ProjectReq>,
        ) -> Result<tonic::Response<super::ProjectReply>, tonic::Status> {
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
                "/bilibili.pgc.gateway.player.v1.PlayURL/Project",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 直播播放页信息
        pub async fn live_play_view(
            &mut self,
            request: impl tonic::IntoRequest<super::LivePlayViewReq>,
        ) -> Result<tonic::Response<super::LivePlayViewReply>, tonic::Status> {
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
                "/bilibili.pgc.gateway.player.v1.PlayURL/LivePlayView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod play_url_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayUrlServer.
    #[async_trait]
    pub trait PlayUrl: Send + Sync + 'static {
        /// 播放页信息
        async fn play_view(
            &self,
            request: tonic::Request<super::PlayViewReq>,
        ) -> Result<tonic::Response<super::PlayViewReply>, tonic::Status>;
        /// 获取投屏地址
        async fn project(
            &self,
            request: tonic::Request<super::ProjectReq>,
        ) -> Result<tonic::Response<super::ProjectReply>, tonic::Status>;
        /// 直播播放页信息
        async fn live_play_view(
            &self,
            request: tonic::Request<super::LivePlayViewReq>,
        ) -> Result<tonic::Response<super::LivePlayViewReply>, tonic::Status>;
    }
    /// 播放地址
    #[derive(Debug)]
    pub struct PlayUrlServer<T: PlayUrl> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayUrl> PlayUrlServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayUrlServer<T>
    where
        T: PlayUrl,
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
                "/bilibili.pgc.gateway.player.v1.PlayURL/PlayView" => {
                    #[allow(non_camel_case_types)]
                    struct PlayViewSvc<T: PlayUrl>(pub Arc<T>);
                    impl<T: PlayUrl> tonic::server::UnaryService<super::PlayViewReq>
                    for PlayViewSvc<T> {
                        type Response = super::PlayViewReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlayViewReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).play_view(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PlayViewSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bilibili.pgc.gateway.player.v1.PlayURL/Project" => {
                    #[allow(non_camel_case_types)]
                    struct ProjectSvc<T: PlayUrl>(pub Arc<T>);
                    impl<T: PlayUrl> tonic::server::UnaryService<super::ProjectReq>
                    for ProjectSvc<T> {
                        type Response = super::ProjectReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProjectReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).project(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bilibili.pgc.gateway.player.v1.PlayURL/LivePlayView" => {
                    #[allow(non_camel_case_types)]
                    struct LivePlayViewSvc<T: PlayUrl>(pub Arc<T>);
                    impl<T: PlayUrl> tonic::server::UnaryService<super::LivePlayViewReq>
                    for LivePlayViewSvc<T> {
                        type Response = super::LivePlayViewReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LivePlayViewReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).live_play_view(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LivePlayViewSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
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
    impl<T: PlayUrl> Clone for PlayUrlServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PlayUrl> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayUrl> tonic::server::NamedService for PlayUrlServer<T> {
        const NAME: &'static str = "bilibili.pgc.gateway.player.v1.PlayURL";
    }
}
