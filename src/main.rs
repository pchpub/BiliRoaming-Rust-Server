use actix_files::Files;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::header::ContentType;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use async_channel::{Receiver, Sender};
use biliroaming_rust_server::mods::background_tasks::*;
use biliroaming_rust_server::mods::config::{init_config, prepare_before_start};
use biliroaming_rust_server::mods::handler::{
    errorurl_reg, handle_api_access_key_request, handle_playurl_request, handle_search_request,
    handle_th_season_request, handle_th_subtitle_request,
};
use biliroaming_rust_server::mods::rate_limit::BiliUserToken;
use biliroaming_rust_server::mods::types::{BackgroundTaskType, BiliConfig, BiliRuntime};
use deadpool_redis::{Config, Pool, Runtime};
use futures::join;
use lazy_static::lazy_static;
use log::{debug, error, info};
use std::fs;
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
        7 => handle_th_season_request(&req, true, true).await,
        8 => handle_th_subtitle_request(&req, false, true).await,
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
    handle_th_season_request(&req, true, true).await
}

#[get("/intl/gateway/v2/app/subtitle")]
async fn thsubtitle_web(req: HttpRequest) -> impl Responder {
    handle_th_subtitle_request(&req, false, true).await
}

#[get("/api/accesskey")]
async fn api_accesskey(req: HttpRequest) -> impl Responder {
    handle_api_access_key_request(&req).await
}

lazy_static! {
    pub static ref SERVER_CONFIG: BiliConfig = init_config();
    pub static ref REDIS_POOL: Pool = Config::from_url(&SERVER_CONFIG.redis)
        .create_pool(Some(Runtime::Tokio1))
        .unwrap();
    pub static ref CHANNEL: (Sender<BackgroundTaskType>, Receiver<BackgroundTaskType>) =
        async_channel::bounded(120);
    pub static ref BILISENDER: Arc<Sender<BackgroundTaskType>> = Arc::new(CHANNEL.0.clone());
}

fn main() -> std::io::Result<()> {
    // 拿来生成signed_url挺方便的 此处测试用
    // let req_params = "access_key=ecffae5ae699fad2653d99120b2f5d11&appkey=27eb53fc9058f8c3&ep_id=508404&fnval=4048&fnver=0&fourk=1&otype=json&qn=112&ts=1673168456811";
    // let mut signed_params = format!("{req_params}&sign=");
    // let mut sign = crypto::md5::Md5::new();
    // crypto::digest::Digest::input_str(&mut sign, &format!("{req_params}c2ed53a74eeefe3cf99fbd01d8c9c375"));
    // let md5_sign = crypto::digest::Digest::result_str(&mut sign);
    // signed_params.push_str(&md5_sign);
    // println!("{signed_params}");

    // init log
    use chrono::Local;
    use std::io::Write;
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}][{:>5}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                buf.default_styled_level(record.level()),
                &record.args()
            )
        })
        .init();

    info!("你好喵~");
    ctrlc::set_handler(move || {
        //目前来看这个已经没用了,但以防万一卡死,还是留着好了
        error!("已关闭 biliroaming_rust_server");
        std::process::exit(0);
    })
    .unwrap();
    // //init server_config => BiliConfig
    //fs::write("config.example.yml", serde_yaml::to_string(&config).unwrap()).unwrap(); //Debug 方便生成示例配置
    let rt = tokio::runtime::Runtime::new().unwrap();
    let server_config: BiliConfig = SERVER_CONFIG.clone();
    let woker_num = server_config.worker_num;
    let port = server_config.port.clone();
    let bilisender = Arc::clone(&*BILISENDER);
    {
        let bili_runtime = BiliRuntime::new(&*SERVER_CONFIG, &*REDIS_POOL, &*BILISENDER);
        rt.block_on(prepare_before_start(bili_runtime));
    }
    let web_background = async move {
        let r = &CHANNEL.1;
        loop {
            let receive_data = match r.recv().await {
                Ok(it) => it,
                _ => {
                    debug!("[Channel] failed to receive data");
                    break;
                }
            };
            let bili_runtime = BiliRuntime::new(&*SERVER_CONFIG, &*REDIS_POOL, &*BILISENDER);
            //println!("[Debug] r:{}",r.len());
            tokio::spawn(async move {
                match background_task_run(receive_data, &bili_runtime).await {
                    Ok(_) => (),
                    Err(value) => error!("{value}"),
                };
            });
        }
        //println!("[Debug] exit web_background");
    };

    let rate_limit_per_second = if server_config.rate_limit_per_second == 0 {
        1
    } else {
        server_config.rate_limit_per_second
    };
    let rate_limit_burst = if server_config.rate_limit_burst == 0 {
        // 并发数
        1919810
    } else {
        server_config.rate_limit_burst
    };
    let rate_limit_conf = GovernorConfigBuilder::default()
        .per_second(rate_limit_per_second)
        .burst_size(rate_limit_burst)
        .key_extractor(BiliUserToken)
        .finish()
        .unwrap();

    let web_main = HttpServer::new(move || {
        let rediscfg = Config::from_url(&server_config.redis);
        let pool = rediscfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        App::new()
            .app_data((pool, server_config.clone(), bilisender.clone()))
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
    rt.block_on(async { join!(web_background, web_main).1 })
}
