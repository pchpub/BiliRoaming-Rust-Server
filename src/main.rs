use curl::easy::Easy;
use deadpool_redis::{redis::cmd, Config, Runtime, Pool};
use std::string::String;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::{http::header::ContentType};
use qstring::QString;
use md5;
use chrono::prelude::*;

async fn get_playurl(req: &HttpRequest,is_app: bool) -> impl Responder {
    let pool = req.app_data::<Pool>().unwrap();
    match req.headers().get("user-agent") {
        Option::Some(_ua) => (),
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2331,\"message\":\"草,没ua你看个der\"}");
        }
    }
    let user_agent = format!("{}",req.headers().get("user-agent").unwrap().to_str().unwrap());
    let query = QString::from(req.query_string());

    let access_key = match query.get("access_key") {
      Option::Some(key) => key.clone(),
      _ => {
        return HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("{\"code\":-2332,\"message\":\"草,没登陆你看个der,让我凭空拿到你账号是吧\"}");
    }
    };

    let appkey = match query.get("appkey") {
        Option::Some(key) => key,
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2333,\"message\":\"差不多得了,appkey都没\"}");
        }
    };

    let area = match query.get("area") {
        Option::Some(area) => area.clone(),
        _ => "hk",
    };

    let area_num = match area {
        "cn" => 1,
        "hk" => 2,
        "tw" => 3,
        "th" => 4,
        _ => 2,
    };

    let ep_id = match query.get("ep_id") {
        Option::Some(key) => key.clone(),
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2334,\"message\":\"无ep_id\"}");
        }
    };

    let cid:String;
    let cid_open: bool;
    match query.get("cid") {
        Option::Some(key) => {
            cid = key.to_string();
            cid_open = true;
        },
        _ => {
            cid = "".to_string();
            cid_open = false;
            // return HttpResponse::Ok()
            //     .content_type(ContentType::plaintext())
            //     .body("{\"code\":-2335,\"message\":\"无cid\"}")
        }
    };

    let appsec = match appkey_to_sec(appkey){
        Ok(value) => value,
        Err(()) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2336,\"message\":\"未知设备\"}");
        }
    };
    let user_info = match getuser_list(pool, access_key, appkey, &appsec,&user_agent).await{
        Ok(value)=> value,
        Err(()) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2337,\"message\":\"获取用户信息失败\"}");
        }
    };
    
    let (_,white) = match auth_user(pool,&user_info.uid,&access_key).await {
        Ok(value) => value,
        Err(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(value);
        }
    };
    if white {
        // TODO: resign
    }
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let mut is_vip = 0;
    if user_info.vip_expire_time >= ts {
        is_vip = 1;
    }
    let key = match is_app {
        true => format!("e{ep_id}c{cid}v{is_vip}{area_num}0101"),
        false => format!("e{ep_id}c{cid}v{is_vip}{area_num}0701"),
    };
    //查询数据+地区（1位）+类型（2位）+版本（2位）
    //地区 cn 1
    //     hk 2
    //     tw 3
    //     th 4 （不打算支持，切割泰区，没弹幕我为什么不看nc-raw?）
    //     default 2
    //类型 app playurl 01
    //     app search 02
    //     app subtitle 03
    //     app season 04 (留着备用)
    //     user_info 05
    //     user_cerinfo 06
    //     web playurl 07
    //     web search 08
    //     web subtitle 09
    //     web season 10
    //版本 ：用于处理版本更新后导致的格式变更
    //     now 01
    let is_expire: bool;
    let mut redis_get_data = String::new();
    match redis_get(&pool, &key).await {
        Some(value) => {
            if &value[..13].parse::<u64>().unwrap() < &ts {
                is_expire = true;
            }else{
                redis_get_data = value[13..].to_string();
                is_expire = false;
            }
        },
        None => {
            is_expire = true;
        }
    };
    let response_body: String;
    if is_expire {
        //println!("is_expire");
        let base_url = match area_num {
            1 => "https://api.bilibili.com.proxy.pch.pub",
            _ => "https://api.bilibili.com",
        };
        let ts_string = ts.to_string();
        let mut query_vec = vec![
            ("access_key", access_key),
            ("appkey", appkey),
            ("build",query.get("build").unwrap_or("6800300")),
            ("device", query.get("device").unwrap_or("android")),
            ("ep_id",ep_id),
            ("fnval","4048"),
            ("fnver","0"),
            ("fourk","1"),
            ("platform","android"),
            ("qn","125"),
            ("ts",&ts_string),
        ];
        if cid_open {
            query_vec.insert(3, ("cid",&cid));
            //add_pair(("cid",query.get("device").unwrap_or("114514")));
        }
        let unsigned_url = qstring::QString::new(query_vec);
        let unsigned_url = format!("{unsigned_url}");
        let signed_url = format!("{unsigned_url}&sign={:x}",md5::compute(format!("{unsigned_url}{appsec}")));
        let proxy_open = match area_num {
            1 => false,
            2 => false,
            _ => true,
        };
        let proxy_url = "127.0.0.1:7890";
        let api = match is_app {
            true => "/pgc/player/api/playurl",
            false => "/pgc/player/web/playurl",
        };

        let body_data = match getwebpage(&format!("{base_url}{api}?{signed_url}"), proxy_open, &proxy_url,&user_agent) {
            Ok(data) => data,
            Err(_) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::plaintext())
                    .body("{\"code\":-2338,\"message\":\"获取播放地址失败喵\"}");
            }
        };
        let value = format!("{}{body_data}",ts+6480*1000);
        let _: () = redis_set(&pool, &key, &value, 6480).await.unwrap_or_default();
        response_body = body_data;
    }else{
        response_body = redis_get_data;
    }
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("From", "biliroaming-rust-server"))
        .body(response_body)
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/pgc/player/api/playurl")]
async fn zhplayurl_app(req:HttpRequest) -> impl Responder {
    get_playurl(&req, true).await
}

#[get("/pgc/player/web/playurl")]
async fn zhplayurl_web(req:HttpRequest) -> impl Responder {
    get_playurl(&req, false).await
}

// #[get("/pgc/view/web/season")]
// async fn zhseason_web( ) -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[get("/x/intl/passport-login/oauth2/refresh_token")]
// async fn th_refresh_token( ) -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[get("/x/v2/search/type")]
// async fn zh_search2( ) -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[get("/x/web-interface/search/type")]
// async fn zh_search3( ) -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

fn getwebpage(url: &str,proxy_open: bool,proxy_url: &str,user_agent: &str) -> Result<String, ()> {
    //println!("url:{url}");

    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    if proxy_open { 
        handle.proxy_type(curl::easy::ProxyType::Socks5).unwrap();
        handle.proxy(proxy_url).unwrap();
        handle.useragent(user_agent).unwrap();
    }

    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        match transfer.perform() {
            Ok(()) => (()),
            _error => {
                return Err(());
            }
        }
    }

    let getwebpage_string: String = String::from_utf8(data).expect("error");
    //println!("getwebpage_string:{getwebpage_string}");
    Ok(getwebpage_string)

}

struct UserInfo {
    access_key: String,
    uid: u64,
    vip_expire_time: u64,
    expire_time: u64,
}

impl UserInfo {
    fn to_json(&self) -> String {
        format!("{{\"access_key\":\"{}\",\"uid\":{},\"vip_expire_time\":{},\"expire_time\":{}}}", self.access_key,self.uid,self.vip_expire_time,self.expire_time)
    }
}

fn to_user_info(user_info_str: &str) -> UserInfo {
    let user_info = json::parse(user_info_str).unwrap();
    UserInfo{
        access_key: user_info["access_key"].to_string(),
        uid: user_info["uid"].as_u64().unwrap(),
        vip_expire_time: user_info["vip_expire_time"].as_u64().unwrap(),
        expire_time: user_info["expire_time"].as_u64().unwrap(),
    }
}

async fn redis_get(redis: &Pool,key: &String) -> Option<String> {
    let mut conn = redis.get().await.unwrap();
    let value: String = match cmd("GET")
        .arg(key)
        .query_async(&mut conn)
        .await {
            Ok(value) => value,
            Err(_) => return None,
        };
    Some(value)
} 

async fn redis_set(redis: &Pool,key: &String,value: &String,expire_time: u64) -> Option<()> {
    let mut conn = redis.get().await.unwrap();
    //let mut return_data: Option<()>;
    match cmd("SET")
        .arg(&[key, value])
        .query_async::<_, ()>(&mut conn)
        .await {
            Ok(_) => (),
            _ => return None,
        }
    match cmd("EXPIRE")
        .arg(&[key, &format!("{expire_time}")])
        .query_async::<_, ()>(&mut conn)
        .await {
            Ok(_) => (),
            _ => return None,
        }
    Some(())
} 

async fn getuser_list(redis: &Pool,access_key: &str,appkey:&str,appsec:&str,user_agent: &str) -> Result<UserInfo,()> {
    let info: String = match redis_get(&redis,&format!("{access_key}20501")).await {
        Some(value) => value,
        None => {
            let dt = Local::now();
            let ts = dt.timestamp_millis() as u64;
            let sign = md5::compute(format!("access_key={}&appkey={}&ts={}{}",access_key,appkey,ts,appsec));
            let url:String = format!("https://app.bilibili.com/x/v2/account/myinfo?access_key={}&appkey={}&ts={}&sign={:x}",access_key,appkey,ts,sign);
            let output = match getwebpage(&url,false,"",user_agent){
                Ok(data) => data,
                Err(_) => {return Err(());}
            };
            let output_json = json::parse(&output).unwrap();
            let output_struct = UserInfo{
                access_key: String::from(access_key),
                uid: output_json["data"]["mid"].as_u64().unwrap(),
                vip_expire_time: output_json["data"]["vip"]["due_date"].as_u64().unwrap(),
                expire_time: ts+25*24*60*60*1000,//用户状态25天强制更新
            };
            let key  = format!("{access_key}20501");
            let value = output_struct.to_json();
            let _ : () = redis_set(&redis,&key, &value,25*24*60*60).await.unwrap_or_default();
            //查询数据+地区（1位）+类型（2位）+版本（2位）
            //地区 cn 1
            //     hk 2
            //     tw 3
            //     th 4 （不打算支持，切割泰区，没弹幕我为什么不看nc-raw?）
            //     default 2
            //类型 app playurl 01
            //     app search 02
            //     app subtitle 03
            //     app season 04 (留着备用)
            //     user_info 05
            //     user_cerinfo 06
            //     web playurl 07
            //     web search 08
            //     web subtitle 09
            //     web season 10
            //版本 ：用于处理版本更新后导致的格式变更
            //     now 01
            return Ok(output_struct);
        }
    };
    Ok(to_user_info(&info))
}

struct UserCerinfo {
    uid: u64,
    black:bool,
    white:bool,
    status_expire_time: u64,
}

impl UserCerinfo {
    fn to_json(&self) -> String {
        format!("{{\"uid\":{},\"black\":{},\"white\":{},\"status_expire_time\":{}}}", self.uid,self.black,self.white,self.status_expire_time).to_string()
    }
}

fn to_usercer_info(usercer_info_str: &str) -> UserCerinfo {
    //println!("{}", user_info_str);
    let usercer_info = json::parse(usercer_info_str).unwrap();
    UserCerinfo{
        uid: usercer_info["uid"].as_u64().unwrap(),
        black: usercer_info["black"].as_bool().unwrap_or(false),
        white: usercer_info["white"].as_bool().unwrap_or(false),
        status_expire_time: usercer_info["status_expire_time"].as_u64().unwrap_or(0),
    }
}

async fn getusercer_list(redis: &Pool,uid: &u64,access_key: &str) -> Result<UserCerinfo,()> {
    //let user_cerinfo_str = String::new();
    let is_expire: bool;
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let user_cerinfo: UserCerinfo;
    let key = format!("{uid}20601");
    match redis_get(redis, &key).await{
        Some(value) => {
            user_cerinfo = to_usercer_info(&value);
            if user_cerinfo.status_expire_time < ts {
                is_expire = true;
            }else{
                is_expire = false;
            }
        },
        None => {
            user_cerinfo = UserCerinfo{
                uid: 233,
                black: false,
                white: false,
                status_expire_time: 233,
            };
            is_expire = true;
        },
    };
    
    if is_expire {
        let getwebpage_data = match getwebpage(&format!("https://black.qimo.ink/status.php?access_key={access_key}"), false, "",""){
            Ok(data) => data,
            Err(_) => {return Err(())}
        };
        let getwebpage_json = json::parse(&getwebpage_data).unwrap();
        //{"code":0,"message":"0","data":{"uid":35828134,"is_blacklist":false,"is_whitelist":false,"reason":""}}
        if getwebpage_json["code"].as_i16().unwrap_or(233) == 0 {
            let return_data = UserCerinfo {
                uid: getwebpage_json["data"]["uid"].as_u64().unwrap(),
                black: getwebpage_json["data"]["is_blacklist"].as_bool().unwrap_or(false),
                white: getwebpage_json["data"]["is_whitelist"].as_bool().unwrap_or(false),
                status_expire_time: ts+1*24*60*60*1000,
            };
            redis_set(redis, &key, &return_data.to_json(), 1*24*60*60*1000).await;
            return Ok(return_data);
        }else{
            return Err(());
        }
    }else{
        return Ok(user_cerinfo);
    }
}

async fn auth_user(redis: &Pool,uid: &u64,access_key: &str) -> Result<(bool,bool),String> {
    match uid {
        357458529 => {
            return Ok((false,true));
        }, 
        374764010 => {
            return Ok((false,true));
        }, 
        384556554 => {
            return Ok((false,true));
        },
        1136877640 => {
            return Ok((false,true));
        },
        113980518 => {
            return Ok((false,true));
        },
        _ => (),
    }
    match getusercer_list(redis, uid, access_key).await{
        Ok(data) => {
            return Ok((data.black, data.white));
        },
        Err(_) => {
            return Err("鉴权失败了喵".to_string());
        }
    };
}

fn appkey_to_sec(appkey:&str) -> Result<String, ()> {
	match appkey {
        "9d5889cf67e615cd" => Ok("8fd9bb32efea8cef801fd895bef2713d".to_string()), // Ai4cCreatorAndroid
		"1d8b6e7d45233436" => Ok("560c52ccd288fed045859ed18bffd973".to_string()), // Android
		"07da50c9a0bf829f" => Ok("25bdede4e1581c836cab73a48790ca6e".to_string()), // AndroidB
		"8d23902c1688a798" => Ok("710f0212e62bd499b8d3ac6e1db9302a".to_string()), // AndroidBiliThings
		"dfca71928277209b" => Ok("b5475a8825547a4fc26c7d518eaaa02e".to_string()), // AndroidHD
		"bb3101000e232e27" => Ok("36efcfed79309338ced0380abd824ac1".to_string()), // AndroidI
		"4c6e1021617d40d9" => Ok("e559a59044eb2701b7a8628c86aa12ae".to_string()), // AndroidMallTicket
		"c034e8b74130a886" => Ok("e4e8966b1e71847dc4a3830f2d078523".to_string()), // AndroidOttSdk
		"4409e2ce8ffd12b8" => Ok("59b43e04ad6965f34319062b478f83dd".to_string()), // AndroidTV
		"37207f2beaebf8d7" => Ok("e988e794d4d4b6dd43bc0e89d6e90c43".to_string()), // BiliLink
		"9a75abf7de2d8947" => Ok("35ca1c82be6c2c242ecc04d88c735f31".to_string()), // BiliScan
		"7d089525d3611b1c" => Ok("acd495b248ec528c2eed1e862d393126".to_string()), // BstarA
        _ => Err(())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("你好喵~");
    HttpServer::new(|| {
        let rediscfg = Config::from_url("redis://default:********@127.0.0.1:6379");//明天写读取配置文件（
        let pool = rediscfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        App::new()
            .app_data(pool)
            .service(hello)
            .service(zhplayurl_app)
            .service(zhplayurl_web)
    })
    .bind(("127.0.0.1", 2661))?
    .workers(4)
    .keep_alive(None)
    .run()
    .await
}