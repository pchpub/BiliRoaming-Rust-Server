use actix_files::Files;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::header::ContentType;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use async_channel::{Receiver, Sender};
use biliroaming_rust_server::mods::background_tasks::*;
use biliroaming_rust_server::mods::config::load_biliconfig;
use biliroaming_rust_server::mods::handler::{
    errorurl_reg, get_season, get_subtitle_th, handle_playurl_request, handle_search_request,
};
use biliroaming_rust_server::mods::pub_api::get_api_accesskey;
use biliroaming_rust_server::mods::rate_limit::BiliUserToken;
use biliroaming_rust_server::mods::tools::update_server;
use biliroaming_rust_server::mods::types::{BackgroundTaskType, BiliConfig};
use deadpool_redis::{Config, Pool, Runtime};
use futures::join;
use std::fs;
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
        1 => handle_playurl_request(&req, true, false).await,
        2 => handle_playurl_request(&req, false, false).await,
        3 => handle_playurl_request(&req, true, true).await,
        4 => handle_search_request(&req, true, false).await,
        5 => handle_search_request(&req, false, false).await,
        6 => handle_search_request(&req, true, true).await,
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
        .app_data::<(Pool, BiliConfig, Arc<Sender<BackgroundTaskType>>)>()
        .unwrap();
    return HttpResponse::Found()
        .insert_header(("Location", &config.donate_url[..]))
        .body("");
}

#[get("/pgc/player/api/playurl")]
async fn zhplayurl_app(req: HttpRequest) -> impl Responder {
    handle_playurl_request(&req, true, false).await
}

#[get("/pgc/player/web/playurl")]
async fn zhplayurl_web(req: HttpRequest) -> impl Responder {
    handle_playurl_request(&req, false, false).await
}

#[get("/intl/gateway/v2/ogv/playurl")]
async fn thplayurl_app(req: HttpRequest) -> impl Responder {
    handle_playurl_request(&req, true, true).await
}

#[get("/x/v2/search/type")]
async fn zhsearch_app(req: HttpRequest) -> impl Responder {
    handle_search_request(&req, true, false).await
}

#[get("/x/web-interface/search/type")]
async fn zhsearch_web(req: HttpRequest) -> impl Responder {
    handle_search_request(&req, false, false).await
}

#[get("/intl/gateway/v2/app/search/type")]
async fn thsearch_app(req: HttpRequest) -> impl Responder {
    handle_search_request(&req, true, true).await //emmmm 油猴脚本也用的这个
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

fn main() -> std::io::Result<()> {
    println!("你好喵~");
    let mut config_type: Option<&str> = None;
    let config_suffix = ["json", "yml"];
    for suffix in config_suffix {
        if Path::new(&format!("config.{suffix}")).exists() {
            config_type = Some(suffix);
        }
    }
    let config = match load_biliconfig(config_type) {
        Ok(value) => value,
        Err(value) => {
            println!("{value}");
            std::process::exit(78);
        }
    };
    ctrlc::set_handler(move || {
        //目前来看这个已经没用了,但以防万一卡死,还是留着好了
        println!("\n已关闭 biliroaming_rust_server");
        std::process::exit(0);
    })
    .unwrap();

    //fs::write("config.example.yml", serde_yaml::to_string(&config).unwrap()).unwrap(); //Debug 方便生成示例配置

    let background_server_config = config.clone();
    let woker_num = config.woker_num;
    let port = config.port.clone();

    if config.auto_update {
        update_server(config.auto_close.clone());
    }

    let (s, r): (Sender<BackgroundTaskType>, Receiver<BackgroundTaskType>) =
        async_channel::bounded(120);
    let bilisender = Arc::new(s);
    // let bilisender_live = bilisender.clone();
    let background_redis_cfg = Config::from_url(&config.redis);
    let background_redis_pool = background_redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .unwrap();
    let mut report_config = background_server_config.report_config.clone();
    if background_server_config.report_open {
        match report_config.init() {
            Ok(_) => (),
            Err(value) => {
                println!("{}", value);
                // background_server_config.report_open = false;
            }
        }
    }
    let web_background = async move {
        //a thread try to update cache
        // println!("[Debug] spawn web_background");
        // if bilisender_live.is_closed() {
        //     println!("[Error] channel was closed");
        // }
        // // 热点路径大量使用.clone(), 大忌大忌, 关键是延长生命周期, but how
        let background_server_config = Arc::new(background_server_config);
        let background_redis_pool = Arc::new(background_redis_pool);
        let report_config = Arc::new(report_config);
        loop {
            let receive_data = match r.recv().await {
                Ok(it) => it,
                _ => {
                    //println!("[Debug] failed to receive data");
                    break;
                }
            };
            //println!("[Debug] r:{}",r.len());
            let background_server_config = Arc::clone(&background_server_config);
            let background_redis_pool = Arc::clone(&background_redis_pool);
            let report_config = Arc::clone(&report_config);
            tokio::spawn(async move {
                match background_task_run(
                    receive_data,
                    background_server_config,
                    report_config,
                    background_redis_pool,
                )
                .await
                {
                    Ok(_) => (),
                    Err(value) => println!("{value}"),
                };
            });
        }
        //println!("[Debug] exit web_background");
    };

    let rate_limit_conf = GovernorConfigBuilder::default()
        .per_second(3)
        .burst_size(20)
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
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { join!(web_background, web_main).1 })
}
