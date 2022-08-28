use actix_files::Files;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::header::ContentType;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use async_channel::{Receiver, Sender};
use biliroaming_rust_server::mods::get_bili_res::{
    errorurl_reg, get_playurl, get_playurl_background, get_search, get_season, get_subtitle_th,
};
use biliroaming_rust_server::mods::pub_api::get_api_accesskey;
use biliroaming_rust_server::mods::rate_limit::BiliUserToken;
use biliroaming_rust_server::mods::tools::update_server;
use biliroaming_rust_server::mods::types::{BiliConfig, SendData};
use deadpool_redis::{Config, Pool, Runtime};
use futures::join;
use serde_json;
use std::fs::{self, File};
use std::path::Path;
use std::sync::Arc;

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
                .body(r#"<html><head><meta charset="utf-8"><title>200 OK</title></head><body><div style="margin:0px auto;text-align:center;"><h1>BiliRoaming-Rust-Server</h1><p>[online] 200 OK</p><br>Powered by <a href="https://github.com/pchpub/BiliRoaming-Rust-Server">BiliRoaming-Rust-Server</a></div></body></html>"#)
        }
    }
}

async fn web_default(req: HttpRequest) -> impl Responder {
    let path = format!("{}", req.path());
    let res_type = if let Some(value) = errorurl_reg(&path).await {
        value
    } else {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body("{\"code\":-404,\"message\":\"请检查填入的服务器地址是否有效\"}");
    };
    match res_type {
        1 => get_playurl(&req, true, false).await,
        2 => get_playurl(&req, false, false).await,
        3 => get_playurl(&req, true, true).await,
        4 => get_search(&req, true, false).await,
        5 => get_search(&req, false, false).await,
        6 => get_search(&req, true, true).await,
        7 => get_season(&req, true, true).await,
        8 => get_subtitle_th(&req, false, true).await,
        _ => {
            println!("[Error] 未预期的行为 match res_type");
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("From", "biliroaming-rust-server"))
                .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                .insert_header(("Access-Control-Allow-Credentials", "true"))
                .insert_header(("Access-Control-Allow-Methods", "GET"))
                .body("{\"code\":-500,\"message\":\"未预期的行为\"}")
        }
    }
}

#[get("/donate")]
async fn donate(req: HttpRequest) -> impl Responder {
    let (_, config, _) = req
        .app_data::<(Pool, BiliConfig, Arc<Sender<SendData>>)>()
        .unwrap();
    return HttpResponse::Found()
        .insert_header(("Location", &config.donate_url[..]))
        .body("");
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

#[get("/api/accesskey")]
async fn api_accesskey(req: HttpRequest) -> impl Responder {
    get_api_accesskey(&req).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("你好喵~");
    let config_file: File;
    let mut config_type: Option<&str> = None;
    let config_suffix = ["json", "yml"];
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
        }
        Some(value) => {
            match File::open(format!("config.{}", value)) {
                Ok(value) => {
                    config_file = value;
                }
                Err(_) => {
                    println!("[error] 配置文件打开失败");
                    std::process::exit(78);
                }
            }
            match value {
                "json" => config = serde_json::from_reader(config_file).unwrap(),
                "yml" => config = serde_yaml::from_reader(config_file).unwrap(),
                _ => {
                    println!("[error] 未预期的错误-1");
                    std::process::exit(78);
                }
            }
        }
    }
    match config_type.unwrap() {
        "json" => fs::write(
            "config.json",
            serde_json::to_string_pretty(&config).unwrap(),
        )
        .unwrap(),
        "yml" => fs::write("config.yml", serde_yaml::to_string(&config).unwrap()).unwrap(),
        _ => {
            println!("[error] 未预期的错误-2");
        }
    }
    ctrlc::set_handler(move || { //目前来看这个已经没用了,但以防万一卡死,还是留着好了
        println!("\n已关闭 biliroaming_rust_server");
        std::process::exit(0);
    })
    .unwrap();

    println!(r#"[Tips] 在代理设置处添加"socks5://"以平滑过渡至新版"#);
    //fs::write("config.example.yml", serde_yaml::to_string(&config).unwrap()).unwrap(); //Debug 方便生成示例配置

    let anti_speedtest_cfg = config.clone();
    let woker_num = config.woker_num;
    let port = config.port.clone();

    if config.auto_update {
        update_server(config.auto_close.clone());
    }

    let (s, r): (Sender<SendData>, Receiver<SendData>) = async_channel::bounded(30);
    let bilisender = Arc::new(s);
    let anti_speedtest_redis_cfg = Config::from_url(&config.redis);
    let pool_background = anti_speedtest_redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .unwrap();
    let web_background = actix_web::rt::spawn(async move {
        //a thread try to update cache
        //println!("[Debug] spawn web_background");
        loop {
            let receive_data = match r.recv().await {
                Ok(it) => it,
                _ => break,
            };
            //println!("[Debug] r:{}",r.len());
            match receive_data.data_type {
                1 => {
                    match get_playurl_background(
                        &pool_background,
                        &receive_data,
                        &anti_speedtest_cfg,
                    )
                    .await
                    {
                        Ok(_) => (),
                        Err(value) => println!("{value}"),
                    };
                }
                // 2 => {
                // TODO: add another data cache
                // },
                _ => {}
            }
        }
        //println!("[Debug] exit web_background");
    });

    let rate_limit_conf = GovernorConfigBuilder::default()
        .per_second(12)
        .burst_size(12)
        .key_extractor(BiliUserToken)
        .finish()
        .unwrap();

    let web_main = HttpServer::new(move || {
        let rediscfg = Config::from_url(&config.redis);
        let pool = rediscfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        App::new()
            .app_data((pool, config.clone(), bilisender.clone()))
            .wrap(Governor::new(&rate_limit_conf))
            .service(hello)
            .service(zhplayurl_app)
            .service(zhplayurl_web)
            .service(thplayurl_app)
            .service(zhsearch_app)
            .service(zhsearch_web)
            .service(thsearch_app)
            .service(thseason_app)
            .service(thsubtitle_web)
            .service(api_accesskey)
            .service(donate)
            .service(Files::new("/", "./web/").index_file("index.html"))
            .default_service(web::route().to(web_default))
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .workers(woker_num)
    .keep_alive(None)
    .run();
    join!(web_background, web_main).1
}
