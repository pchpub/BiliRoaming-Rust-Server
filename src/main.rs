use actix_files::Files;
use actix_web::http::header::ContentType;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use async_channel::{Receiver, Sender};
use biliroaming_rust_server::mods::get_bili_res::{
    get_playurl, get_search, get_season, get_subtitle_th, get_playurl_background,
};
use biliroaming_rust_server::mods::types::{BiliConfig, SendData};
use deadpool_redis::{Config, Runtime};
use serde_json;
use std::fs::{self, File};
use std::sync::Arc;
use std::thread::spawn;
use std::path::Path;
use futures::executor::block_on;

#[get("/")]
async fn hello() -> impl Responder {
    match fs::read_to_string("./web/index.html") {
        Ok(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(value);
        }
        Err(_) => {
            return HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(r#"<html><head><meta charset="utf-8"><title>200 OK</title></head><body><div style="margin:0px auto;text-align:center;"><h1>BiliRoaming-Rust-Server</h1><p>[online] 200 OK</p><br>Powered by<a href="https://github.com/pchpub/BiliRoaming-Rust-Server">BiliRoaming-Rust-Server</a></div></body></html>"#)
        }
    }
}

#[get("/pgc/player/api/playurl")]
async fn zhplayurl_app(req: HttpRequest) -> impl Responder {
    get_playurl(&req, true, false).await
}

#[get("/pgc/player/web/playurl")]
async fn zhplayurl_web(req: HttpRequest) -> impl Responder {
    get_playurl(&req, false, false).await
}

#[get("/intl/gateway/v2/ogv/playurl")]
async fn thplayurl_app(req: HttpRequest) -> impl Responder {
    get_playurl(&req, true, true).await
}

#[get("/x/v2/search/type")]
async fn zhsearch_app(req: HttpRequest) -> impl Responder {
    get_search(&req, true, false).await
}

#[get("/x/web-interface/search/type")]
async fn zhsearch_web(req: HttpRequest) -> impl Responder {
    get_search(&req, false, false).await
}

#[get("/intl/gateway/v2/app/search/type")]
async fn thsearch_app(req: HttpRequest) -> impl Responder {
    get_search(&req, true, true).await //emmmm 油猴脚本也用的这个
}

#[get("/intl/gateway/v2/ogv/view/app/season")]
async fn thseason_app(req: HttpRequest) -> impl Responder {
    get_season(&req, true, true).await
}

#[get("/intl/gateway/v2/app/subtitle")]
async fn thsubtitle_web(req: HttpRequest) -> impl Responder {
    get_subtitle_th(&req, false, true).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("你好喵~");
    let config_file: File;
    let mut config_type: Option<&str> = None;
    let config_suffix = ["json","yaml"];
    for suffix in config_suffix {
        if Path::new(&format!("config.{suffix}")).exists() {
            config_type = Some(suffix);
        }
    }
    let config: BiliConfig;
    match config_type {
        None => {
            println!("[error] 无配置文件");
            std::process::exit(78);
        },
        Some(value) => {
            match File::open(format!("config.{}",value)) {
                Ok(value) => {config_file = value;},
                Err(_) => {
                    println!("[error] 配置文件打开失败");
                    std::process::exit(78);
                },
            }
            match value {
                "json" => config = serde_json::from_reader(config_file).unwrap(),
                "yaml" => config = serde_yaml::from_reader(config_file).unwrap(),
                _ => {
                    println!("[error] 未预期的错误-1");
                    std::process::exit(78);
                }
            }
        },
    }

    let anti_speedtest_cfg = config.clone();
    let woker_num = config.woker_num;
    let port = config.port.clone();

    let (s, r): (Sender<SendData>, Receiver<SendData>) = async_channel::unbounded();
    let bilisender = Arc::new(s);
    let anti_speedtest_redis_cfg = Config::from_url(&config.redis);
    spawn(move || {
        let pool = anti_speedtest_redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        loop {
            if let Ok(receive_data) = block_on(r.recv()) {
                match receive_data.data_type {
                    1 => {
                        if let Ok(_) = block_on(get_playurl_background(&pool, &receive_data, &anti_speedtest_cfg)) {
                            println!("[Test] cache Ok");
                        }else {
                            println!("[Test] cache Err");
                        }
                    },
                    // 2 => { 
                    // TODO: add another data cache
                    // },
                    _ => {},
                }
                
            }
        }   
    });
    HttpServer::new(move || {
        let rediscfg = Config::from_url(&config.redis);
        let pool = rediscfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        App::new()
            .app_data((pool, config.clone(), bilisender.clone()))
            .service(hello)
            .service(zhplayurl_app)
            .service(zhplayurl_web)
            .service(thplayurl_app)
            .service(zhsearch_app)
            .service(zhsearch_web)
            .service(thsearch_app)
            .service(thseason_app)
            .service(thsubtitle_web)
            .service(Files::new("/", "./web/").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .workers(woker_num)
    .keep_alive(None)
    .run()
    .await
}
