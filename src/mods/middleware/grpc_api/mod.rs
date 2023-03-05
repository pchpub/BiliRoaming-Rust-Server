/// 构建gRPC头部所需
pub mod grpc_headers {
    include!("bilibili.metadata.device.rs");
    include!("bilibili.metadata.fawkes.rs");
    include!("bilibili.metadata.locale.rs");
    include!("bilibili.metadata.network.rs");
    include!("bilibili.metadata.parabox.rs");
    include!("bilibili.metadata.restriction.rs");
}

/// metadata
pub mod grpc_metadata {
    include!("bilibili.metadata.rs");
}

/*
 * 业务代码
 */
/// V1 版本gRPC playurl, fallback
pub mod grpc_playurl_v1 {
    include!("bilibili.pgc.gateway.player.v2.rs");
}
/// V2版本gRPC playurl
pub mod grpc_playurl_v2 {
    include!("bilibili.pgc.gateway.player.v2.rs");
}

/// gRPC 搜索
pub mod grpc_search {
    include!("bilibili.polymer.app.search.v1.rs");
}
pub mod bilibili_pagination {
    include!("bilibili.pagination.rs");
}
pub mod bilibili_app_archive_middleware_v1 {
    include!("bilibili.app.archive.middleware.v1.rs");
}

/// gRPC RPC Status
pub mod bilibili_rpc {
    include!("bilibili.rpc.rs");
}

/// 自定义server端auth相关
pub mod server_auth_v1 {
    include!("server.auth.v1.rs");
}

/// 公共API: 番剧信息服务(TODO)
pub mod server_api_public_bangumi {
    include!("server.api.public.bangumi.rs");
}
