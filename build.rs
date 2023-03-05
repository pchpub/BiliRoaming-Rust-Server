use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ! 版本号
    let version = get_git_version();
    let mut f = File::create(Path::new(&env::var("OUT_DIR")?).join("VERSION"))?;
    f.write_all(version.trim().as_bytes())?;
    // 服务端
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/server/api/public/bangumi.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/server/auth/v1/auth.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("src/mods/middleware/error")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/server/rpc/error.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("src/mods/middleware/grpc_api")
        // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/rpc/status.proto"],
            &["./src/proto"],
        )?;
    // 身份认证相关
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/metadata/device/device.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/metadata/fawkes/fawkes.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/metadata/locale/locale.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/metadata/network/network.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/metadata/parabox/pararbox.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/metadata/restriction/restriction.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(false) // 是否编译生成用于服务端的代码
        .build_client(true) // 是否编译生成用于客户端的代码
        .out_dir("src/mods/middleware/grpc_api") // 输出的路径，此处指定为项目根目录下的./src/proto目录
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]") // 需要解析客户端传来的bin metadata, 添加此项
        // 指定要编译的proto文件路径列表，第二个参数是提供protobuf的扩展路径，
        // 因为protobuf官方提供了一些扩展功能，自己也可能会写一些扩展功能，
        // 如存在，则指定扩展文件路径，如果没有，则指定为proto文件所在目录即可
        .compile(
            &["./src/proto/bilibili/metadata/metadata.proto"],
            &["./src/proto"],
        )?;
    // 实际功能实现
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/pagination/pagination.proto"],
            &["./src/proto"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        .compile(
            &["./src/proto/bilibili/app/archive/middleware/v1/preload.proto"],
            &["./src/proto"],
        )?;
    // for gRPC playurl
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/pgc/gateway/player/v1/playurl.proto"],
            &["./src/proto"],
        )?;
    // for gRPC playurl
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["./src/proto/bilibili/pgc/gateway/player/v2/playurl.proto"],
            &["./src/proto"],
        )?;
    // for gRPC search
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/mods/middleware/grpc_api")
        // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        // 涉及import其他proto, 注意此处extern_path, 类比即可
        .extern_path(
            ".bilibili.app.archive.middleware.v1",
            "crate::mods::middleware::grpc_api::bilibili_app_archive_middleware_v1",
        )
        .extern_path(
            ".bilibili.pagination",
            "crate::mods::middleware::grpc_api::bilibili_pagination",
        )
        .compile(
            &["./src/proto/bilibili/polymer/app/search/v1/search.proto"],
            &["./src/proto"],
        )?;
    Ok(())
}

fn get_git_version() -> String {
    let version = env::var("CARGO_PKG_VERSION").unwrap().to_string();

    let child = Command::new("git").args(&["describe", "--always"]).output();
    match child {
        Ok(child) => {
            let buf = String::from_utf8(child.stdout).expect("failed to read stdout");
            return version + "-b" + &buf;
        }
        Err(err) => {
            eprintln!("`git describe` err: {}", err);
            return version;
        }
    }
}
