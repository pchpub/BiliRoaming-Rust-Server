use biliroaming_rust_server::mods::get_bili_res::{get_playurl, get_search,get_season};
use biliroaming_rust_server::mods::types::BiliConfig;
use deadpool_redis::{Config, Runtime};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest};
use std::fs::File;
use serde_json;

#[get("/")]
async fn hello() -> impl Responder {
    //println!("{:?}",req.headers().get("Host").unwrap());
    HttpResponse::Ok().body("Rust server is online!")
}

#[get("/pgc/player/api/playurl")]
async fn zhplayurl_app(req:HttpRequest) -> impl Responder {
    get_playurl(&req, true,false).await
}

#[get("/pgc/player/web/playurl")]
async fn zhplayurl_web(req:HttpRequest) -> impl Responder {
    get_playurl(&req, false,false).await
}

#[get("/intl/gateway/v2/ogv/playurl")]
async fn thplayurl_app(req:HttpRequest) -> impl Responder {
    get_playurl(&req, true,true).await
}

#[get("/x/v2/search/type")]
async fn zhsearch_app(req:HttpRequest) -> impl Responder {
    get_search(&req, true,false).await
}

#[get("/x/web-interface/search/type")]
async fn zhsearch_web(req:HttpRequest) -> impl Responder {
    get_search(&req, false,false).await
}

#[get("/intl/gateway/v2/app/search/type")]
async fn thsearch_app(req:HttpRequest) -> impl Responder {
    get_search(&req, true,true).await //emmmm 油猴脚本也用的这个
}

#[get("/intl/gateway/v2/ogv/view/app/season")]
async fn thseason_app(req:HttpRequest) -> impl Responder {
    get_season(&req, true,true).await
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
        },
    }
    let config: BiliConfig = serde_json::from_reader(config_file).unwrap();
    // println!("{}", config.search_remake.get("bili.pch.pub").unwrap());
    let woker_num = config.woker_num;
    let port = config.port.clone();
    HttpServer::new(move || {
        let rediscfg = Config::from_url(&config.redis);
        let pool = rediscfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        App::new()
            .app_data((pool,config.clone()))
            .service(hello)
            .service(zhplayurl_app)
            .service(zhplayurl_web)
            .service(thplayurl_app)
            .service(zhsearch_app)
            .service(zhsearch_web)
            .service(thsearch_app)
            .service(thseason_app)
    })
    .bind(("0.0.0.0", port))?
    .workers(woker_num)
    .keep_alive(None)
    .run()
    .await
}