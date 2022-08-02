use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;
use biliroaming_rust_server::mods::get_bili_res::{
    get_playurl, get_search, get_season, get_subtitle_th,
};
use biliroaming_rust_server::mods::types::BiliConfig;
use deadpool_redis::{Config, Runtime};
use serde_json;
use std::fs::{self, File};

#[get("/")]
async fn hello() -> impl Responder {
    match fs::read_to_string("index.html") {
        Ok(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(value);
        }
        Err(_) => {
            return HttpResponse::Ok()
                .body("Rust server is online. Powered by BiliRoaming-Rust-Server");
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
    match File::open("config.json") {
        Ok(value) => config_file = value,
        Err(_) => {
            println!("缺少配置文件喵");
            std::process::exit(78);
        }
    }
    let config: BiliConfig = serde_json::from_reader(config_file).unwrap();
    let woker_num = config.woker_num;
    let port = config.port.clone();
    HttpServer::new(move || {
        let rediscfg = Config::from_url(&config.redis);
        let pool = rediscfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        App::new()
            .app_data((pool, config.clone()))
            .service(hello)
            .service(zhplayurl_app)
            .service(zhplayurl_web)
            .service(thplayurl_app)
            .service(zhsearch_app)
            .service(zhsearch_web)
            .service(thsearch_app)
            .service(thseason_app)
            .service(thsubtitle_web)
    })
    .bind(("0.0.0.0", port))?
    .workers(woker_num)
    .keep_alive(None)
    .run()
    .await
}
