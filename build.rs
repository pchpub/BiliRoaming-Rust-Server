fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 身份认证相关
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/metadata/device/device.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/metadata/fawkes/fawkes.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/metadata/locale/locale.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/metadata/network/network.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/metadata/parabox/pararbox.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/metadata/restriction/restriction.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false) // 是否编译生成用于服务端的代码
        .build_client(true) // 是否编译生成用于客户端的代码
        .out_dir("src/mods/grpc_api") // 输出的路径，此处指定为项目根目录下的./src/protos目录
        // 指定要编译的proto文件路径列表，第二个参数是提供protobuf的扩展路径，
        // 因为protobuf官方提供了一些扩展功能，自己也可能会写一些扩展功能，
        // 如存在，则指定扩展文件路径，如果没有，则指定为proto文件所在目录即可
        .compile(
            &["./src/protos/bilibili/metadata/metadata.proto"],
            &["./src/protos"],
        )?;
    // 实际功能实现
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/pagination/pagination.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/app/archive/middleware/v1/preload.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        .compile(
            &["./src/protos/bilibili/pgc/gateway/player/v2/playurl.proto"],
            &["./src/protos"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/grpc_api")
        // 涉及import其他proto, 注意此处extern_path, 类比即可
        .extern_path(
            ".bilibili.app.archive.middleware.v1",
            "crate::mods::grpc_api::bilibili_app_archive_middleware_v1",
        )
        .extern_path(
            ".bilibili.pagination",
            "crate::mods::grpc_api::bilibili_pagination",
        )
        .compile(
            &["./src/protos/bilibili/polymer/app/search/v1/search.proto"],
            &["./src/protos"],
        )?;
    Ok(())
}
