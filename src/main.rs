use curl::easy::Easy;
use deadpool_redis::{redis::cmd, Config, Runtime, Pool};
use std::collections::HashMap;
use std::string::String;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::{http::header::ContentType};
use qstring::QString;
use md5;
use chrono::prelude::*;
use std::fs::File;
use serde_json;
use serde::{Deserialize, Serialize};

async fn get_playurl(req: &HttpRequest,is_app: bool) -> impl Responder {
    let (pool,config) = req.app_data::<(Pool,BiliConfig)>().unwrap();
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

    let mut appkey = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436", //为了应对新的appkey,应该设定默认值
        // _ => {
        //     return HttpResponse::Ok()
        //         .content_type(ContentType::plaintext())
        //         .body("{\"code\":-2333,\"message\":\"差不多得了,appkey都没\"}");
        // }
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

    let mut appsec = match appkey_to_sec(appkey){
        Ok(value) => value,
        Err(()) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2336,\"message\":\"未知设备\"}");
        }
    };

    let user_info = match getuser_list(pool, access_key, appkey, &appsec,&user_agent).await{
        Ok(value)=> value,
        Err(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(format!("{{\"code\":-2337,\"message\":\"{value}\"}}"));
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
            query_vec.push(("cid",&cid));
            //add_pair(("cid",query.get("device").unwrap_or("114514")));
        }
        match area_num {
            4 => {
                appkey = "7d089525d3611b1c";
                appsec = appkey_to_sec(&appkey).unwrap();
                query_vec.push(("s_locale","zh_SG"));
            }
            _ => (),
        }
        query_vec.sort_by_key(|v| v.0);
        let unsigned_url = qstring::QString::new(query_vec);
        let unsigned_url = format!("{unsigned_url}");
        let signed_url = format!("{unsigned_url}&sign={:x}",md5::compute(format!("{unsigned_url}{appsec}")));
        let proxy_open = match area_num {
            1 => &config.cn_proxy_open,
            2 => &config.hk_proxy_open,
            3 => &config.tw_proxy_open,
            4 => &config.th_proxy_open,
            _ => &config.tw_proxy_open,
        };
        let proxy_url = match area_num{
            1 => &config.cn_proxy_url,
            2 => &config.hk_proxy_url,
            3 => &config.tw_proxy_url,
            4 => &config.th_proxy_url,
            _ => &config.tw_proxy_url,
        };
        let api = match is_app {
            true => {
                match area_num{
                    1 => &config.cn_app_playurl_api,
                    2 => &config.hk_app_playurl_api,
                    3 => &config.tw_app_playurl_api,
                    4 => &config.th_app_playurl_api,
                    _ => &config.tw_app_playurl_api,
                }
            },
            false => {
                match area_num {
                    1 => &config.cn_web_playurl_api,
                    2 => &config.hk_web_playurl_api,
                    3 => &config.tw_web_playurl_api,
                    4 => &config.th_web_playurl_api,
                    _ => &config.tw_web_playurl_api,
                }
            },
        };

        let body_data = match getwebpage(&format!("{api}?{signed_url}"), proxy_open, &proxy_url,&user_agent) {
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
        .content_type(ContentType::json())
        .insert_header(("From", "biliroaming-rust-server"))
        .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
        .insert_header(("Access-Control-Allow-Credentials","true"))
        .insert_header(("Access-Control-Allow-Methods", "GET"))
        .body(response_body)
}

async fn get_search(req: &HttpRequest,is_app: bool) -> impl Responder {
    let (pool,config) = req.app_data::<(Pool,BiliConfig)>().unwrap();
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
            .body("{\"code\":-2332,\"message\":\"草,没登陆你搜个der,让我凭空拿到你账号是吧\"}");
        }
    };

    let appkey = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436", //为了应对新的appkey,应该设定默认值
    };

    let keyword = match query.get("keyword") {
        Option::Some(key) => key.clone(),
        _ => ""
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
        Err(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(format!("{{\"code\":-2337,\"message\":\"{value}\"}}"));
        }
    };
    
    let (_,white) = match auth_user(pool,&user_info.uid,&access_key).await {
        Ok(value) => value,
        Err(_) => (false,false)
    };

    if white {
        // TODO: resign
    }

    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec = vec![
        ("access_key", access_key),
        ("appkey", appkey),
        ("build",query.get("build").unwrap_or("6400000")),
        ("c_locale","zh_CN"),
        ("channel","master"),
        ("device", query.get("device").unwrap_or("android")),
        ("disable_rcmd","0"),
        ("fnval","4048"),
        ("fnver","0"),
        ("fourk","1"),
        ("highlight","1"),
        ("keyword",keyword),
        ("mobi_app","android"),
        ("platform","android"),
        ("pn","1"),
        ("ps","20"),
        ("qn","120"),
        ("s_locale","zh_CN"),
        ("ts",&ts_string),
        ("type","7"),
    ];

    match query.get("statistics") {
        Some(value) => {
            query_vec.push(("statistics",value));
        }
        _ => (),
    }
    query_vec.sort_by_key(|v| v.0);
    //let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{}",qstring::QString::new(query_vec));
    let signed_url = format!("{unsigned_url}&sign={:x}",md5::compute(format!("{unsigned_url}{appsec}")));
    let api = match (area_num,is_app) {
        (1,true) => &config.cn_app_search_api,
        (2,true) => &config.hk_app_search_api,
        (3,true) => &config.tw_app_search_api,
        (4,true) => &config.th_app_search_api,
        (1,false) => &config.cn_web_search_api,
        (2,false) => &config.hk_web_search_api,
        (3,false) => &config.tw_web_search_api,
        (4,false) => &config.th_web_search_api,
        _ => &config.hk_app_search_api,
    };

    let proxy_open = match area_num {
        1 => &config.cn_proxy_open,
        2 => &config.hk_proxy_open,
        3 => &config.tw_proxy_open,
        4 => &config.th_proxy_open,
        _ => &config.hk_proxy_open,
    };

    let proxy_url = match area_num {
        1 => &config.cn_proxy_url,
        2 => &config.hk_proxy_url,
        3 => &config.tw_proxy_url,
        4 => &config.th_proxy_url,
        _ => &config.hk_proxy_url,
    };

    let body_data = match getwebpage(&format!("{api}?{signed_url}"), proxy_open, &proxy_url,&user_agent) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2338,\"message\":\"获取失败喵\"}");
        }
    };

    if !is_app {
        return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("From", "biliroaming-rust-server"))
                .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                .insert_header(("Access-Control-Allow-Credentials","true"))
                .insert_header(("Access-Control-Allow-Methods", "GET"))
                .body(body_data);
    }

    let host = match req.headers().get("Host") {
        Some(host) => host.to_str().unwrap(),
        _ => match req.headers().get("authority"){
            Some(host) => host.to_str().unwrap(),
            _ => ""
        }
    };

    match config.search_remake.get(host) {
        Some(value) => {
            let mut body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
            if body_data_json["code"].as_i64().unwrap_or(233) != 0 {
                return HttpResponse::Ok()
                    .content_type(ContentType::plaintext())
                    .body("{\"code\":-2338,\"message\":\"获取失败喵\"}");
            }

            body_data_json["data"]["items"].as_array_mut().unwrap().insert(0, serde_json::from_str(&value).unwrap());
            let body_data = body_data_json.to_string();
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("From", "biliroaming-rust-server"))
                .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                .insert_header(("Access-Control-Allow-Credentials","true"))
                .insert_header(("Access-Control-Allow-Methods", "GET"))
                .body(body_data);
        },
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("From", "biliroaming-rust-server"))
                .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                .insert_header(("Access-Control-Allow-Credentials","true"))
                .insert_header(("Access-Control-Allow-Methods", "GET"))
                .body(body_data);
        }
    };

}

#[get("/")]
async fn hello() -> impl Responder {
    //println!("{:?}",req.headers().get("Host").unwrap());
    HttpResponse::Ok().body("Rust server is online!")
}

#[get("/pgc/player/api/playurl")]
async fn zhplayurl_app(req:HttpRequest) -> impl Responder {
    get_playurl(&req, true).await
}

#[get("/pgc/player/web/playurl")]
async fn zhplayurl_web(req:HttpRequest) -> impl Responder {
    get_playurl(&req, false).await
}

#[get("/intl/gateway/v2/ogv/playurl")]
async fn thplayurl_app(req:HttpRequest) -> impl Responder {
    get_playurl(&req, true).await
}

#[get("/x/v2/search/type")]
async fn zhsearch_app(req:HttpRequest) -> impl Responder {
    get_search(&req, true).await
}

#[get("/x/web-interface/search/type")]
async fn zhsearch_web(req:HttpRequest) -> impl Responder {
    get_search(&req, false).await
}

#[get("/intl/gateway/v2/app/search/type")]
async fn thsearch_app(req:HttpRequest) -> impl Responder {
    get_search(&req, true).await //emmmm 油猴脚本也用的这个
}

// #[get("/x/intl/passport-login/oauth2/refresh_token")]
// async fn th_refresh_token( ) -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }





fn getwebpage(url: &str,proxy_open: &bool,proxy_url: &str,user_agent: &str) -> Result<String, ()> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(false).unwrap();
    
    if *proxy_open { 
        handle.proxy_type(curl::easy::ProxyType::Socks5Hostname).unwrap();
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

async fn getuser_list(redis: &Pool,access_key: &str,appkey:&str,appsec:&str,user_agent: &str) -> Result<UserInfo,String> {
    let info: String = match redis_get(&redis,&format!("{access_key}20501")).await {
        Some(value) => value,
        None => {
            let dt = Local::now();
            let ts = dt.timestamp_millis() as u64;
            let ts_min = dt.timestamp() as u64;
            let sign = md5::compute(format!("access_key={}&appkey={}&ts={}{}",access_key,appkey,ts_min,appsec));
            let url:String = format!("https://app.bilibili.com/x/v2/account/myinfo?access_key={}&appkey={}&ts={}&sign={:x}",access_key,appkey,ts_min,sign);
            //println!("{}",url);
            let output = match getwebpage(&url,&false,"",user_agent){
                Ok(data) => data,
                Err(_) => {
                    println!("getuser_list函数寄了 url:{}",url);
                    return Err("emmmm解析服务器的网络问题".to_string());
                }
            };
            //println!("{}",output);
            let output_json = json::parse(&output).unwrap();
            let output_struct: UserInfo;
            if output_json["code"].as_i32().unwrap() == 0 {
                output_struct = UserInfo{
                    access_key: String::from(access_key),
                    uid: output_json["data"]["mid"].as_u64().unwrap(),
                    vip_expire_time: output_json["data"]["vip"]["due_date"].as_u64().unwrap(),
                    expire_time: ts+25*24*60*60*1000,//用户状态25天强制更新
                };
            }else if output_json["code"].as_i32().unwrap() == -400{
                println!("getuser_list函数寄了 output_json:{}",output_json);
                return Err("可能你用的不是手机".to_string());
            }else if output_json["code"].as_i32().unwrap() == -101{
                println!("getuser_list函数寄了 output_json:{}",output_json);
                return Err("账号未登录喵(b站api说的,估计你access_key过期了)".to_string());
            }else if output_json["code"].as_i32().unwrap() == -3{
                println!("getuser_list函数寄了 output_json:{}",output_json);
                return Err("可能我sign参数算错了,非常抱歉喵".to_string());
            }else if output_json["code"].as_i32().unwrap() == -412{
                println!("getuser_list函数寄了 output_json:{}",output_json);
                return Err("被草到风控了.....".to_string());
            }else{
                println!("getuser_list函数寄了 output_json:{}",output_json);
                return Err(format!("鼠鼠说:{}",output_json["code"].as_i32().unwrap()));
            }
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
        let getwebpage_data = match getwebpage(&format!("https://black.qimo.ink/status.php?access_key={access_key}"), &false, "",""){
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
        "178cf125136ca8ea" => Ok("34381a26236dd1171185c0beb042e1c6".to_string()), // AndroidB
        "27eb53fc9058f8c3" => Ok("c2ed53a74eeefe3cf99fbd01d8c9c375".to_string()), // ios
        "57263273bc6b67f6" => Ok("a0488e488d1567960d3a765e8d129f90".to_string()), // Android
        "7d336ec01856996b" => Ok("a1ce6983bc89e20a36c37f40c4f1a0dd".to_string()), // AndroidB
        "85eb6835b0a1034e" => Ok("2ad42749773c441109bdc0191257a664".to_string()), // unknown
        "8e16697a1b4f8121" => Ok("f5dd03b752426f2e623d7badb28d190a".to_string()), // AndroidI
        "aae92bc66f3edfab" => Ok("af125a0d5279fd576c1b4418a3e8276d".to_string()), // PC	投稿工具
        "ae57252b0c09105d" => Ok("c75875c596a69eb55bd119e74b07cfe3".to_string()), // AndroidI
        "bca7e84c2d947ac6" => Ok("60698ba2f68e01ce44738920a0ffe768".to_string()), // login
        "iVGUTjsxvpLeuDCf" => Ok("aHRmhWMLkdeMuILqORnYZocwMBpMEOdt".to_string()), //Android	取流专用
        "YvirImLGlLANCLvM" => Ok("JNlZNgfNGKZEpaDTkCdPQVXntXhuiJEM".to_string()), //ios	取流专用
        //_ => Ok("560c52ccd288fed045859ed18bffd973".to_string()),
        _ => Err(())
    }
}

#[derive(Serialize, Deserialize,Clone)]
struct BiliConfig {
    redis : String,
    woker_num : usize,
    port : u16,
    cn_app_playurl_api : String,
    tw_app_playurl_api : String,
    hk_app_playurl_api : String,
    th_app_playurl_api : String,
    cn_web_playurl_api : String,
    tw_web_playurl_api : String,
    hk_web_playurl_api : String,
    th_web_playurl_api : String,
    cn_app_search_api : String,
    tw_app_search_api : String,
    hk_app_search_api : String,
    th_app_search_api : String,
    cn_web_search_api : String,
    tw_web_search_api : String,
    hk_web_search_api : String,
    th_web_search_api : String,
    cn_proxy_url : String,
    tw_proxy_url : String,
    hk_proxy_url : String,
    th_proxy_url : String,
    cn_proxy_open : bool,
    tw_proxy_open : bool,
    hk_proxy_open : bool,
    th_proxy_open : bool,
    search_remake : HashMap<String, String>,
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
    })
    .bind(("0.0.0.0", port))?
    .workers(woker_num)
    .keep_alive(None)
    .run()
    .await
}