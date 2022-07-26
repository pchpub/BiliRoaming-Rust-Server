use curl::easy::{Easy, List};
use deadpool_redis::{Pool};
use actix_web::{HttpResponse, Responder, HttpRequest};
use actix_web::{http::header::ContentType};
use qstring::QString;
use md5;
use chrono::prelude::*;
use serde_json::{self};
use std::io::Read;
use super::types::{BiliConfig, ResignInfo};
use super::get_user_info::{appkey_to_sec, getuser_list, auth_user};
use super::request::{redis_get, getwebpage, redis_set};

pub async fn get_playurl(req: &HttpRequest,is_app: bool,is_th: bool) -> impl Responder {

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

    let mut access_key = match query.get("access_key") {
      Option::Some(key) => key.to_string(),
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
        _ => {
            if is_th {
                "th"
            }else{
                "hk"
            }
        },
    };

    let area_num: i8 = match area {
        "cn" => 1,
        "hk" => 2,
        "tw" => 3,
        "th" => 4,
        _ => 2,
    };

    let ep_id = match query.get("ep_id") {
        Option::Some(key) => Some(key.clone()),
        _ => None,
    };

    let cid = match query.get("cid") {
        Option::Some(key) => Some(key.clone()),
        _ => None,
    };

    let mut appsec = match appkey_to_sec(appkey){
        Ok(value) => value,
        Err(()) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2336,\"message\":\"未知设备\"}");
        }
    };

    let user_info = match getuser_list(pool, &access_key, appkey, &appsec,&user_agent).await {
        Ok(value)=> value,
        Err(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(format!("{{\"code\":-2337,\"message\":\"{value}\"}}"));
        }
    };
    
    let (_,white) = match auth_user(pool,&user_info.uid,&access_key,&config).await {
        Ok(value) => value,
        Err(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(value);
        }
    };
    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let mut is_vip = 0;
    if is_th {
        is_vip = 0;
        if white || *config.resign_pub.get("4").unwrap_or(&false) {
            access_key = get_resign_accesskey(pool,&4,&user_agent,&config).await.unwrap_or(access_key);
            is_vip = 1;
        }
    }else{
        if user_info.vip_expire_time >= ts {
            is_vip = 1;
        }else if white || *config.resign_pub.get(&area_num.to_string()).unwrap_or(&false) {
            access_key = get_resign_accesskey(pool,&area_num,&user_agent,&config).await.unwrap_or(access_key);
            let user_info = match getuser_list(pool, &access_key, appkey, &appsec,&user_agent).await {
                Ok(value)=> value,
                Err(value) => {
                    return HttpResponse::Ok()
                        .content_type(ContentType::plaintext())
                        .body(format!("{{\"code\":-23372,\"message\":\"{value}\"}}"));
                }
            };
            if user_info.vip_expire_time >= ts {
                is_vip = 1;
            }
        }
    }

    let key = match is_app {
        true => format!("e{}c{}v{is_vip}{area_num}0101",ep_id.unwrap_or(""),cid.unwrap_or("")),
        false => format!("e{}c{}v{is_vip}{area_num}0701",ep_id.unwrap_or(""),cid.unwrap_or("")),
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
        //     resign_info 11
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
            ("access_key", &access_key[..]),
            ("appkey", appkey),
            ("build",query.get("build").unwrap_or("6800300")),
            ("device", query.get("device").unwrap_or("android")),
            ("fnval","4048"),
            ("fnver","0"),
            ("fourk","1"),
            ("platform","android"),
            ("qn","125"),
            ("ts",&ts_string),
        ];
        match ep_id {
            Some(value) => query_vec.push(("ep_id", value)),
            None => (),
        }
        match cid {
            Some(value) => query_vec.push(("cid", value)),
            None => (),
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
            1 => &config.cn_proxy_playurl_open,
            2 => &config.hk_proxy_playurl_open,
            3 => &config.tw_proxy_playurl_open,
            4 => &config.th_proxy_playurl_open,
            _ => &config.tw_proxy_playurl_open,
        };
        let proxy_url = match area_num{
            1 => &config.cn_proxy_playurl_url,
            2 => &config.hk_proxy_playurl_url,
            3 => &config.tw_proxy_playurl_url,
            4 => &config.th_proxy_playurl_url,
            _ => &config.tw_proxy_playurl_url,
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
        let body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
        let expire_time = match config.cache.get(&body_data_json["code"].as_i64().unwrap().to_string()) {
            Some(value) => value.clone(),
            None => {
                config.cache.get("other").unwrap().clone()
            },
        };
        let value = format!("{}{body_data}",ts+expire_time*1000);
        let _: () = redis_set(&pool, &key, &value, expire_time).await.unwrap_or_default();
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

pub async fn get_search(req: &HttpRequest,is_app: bool,is_th: bool) -> impl Responder {
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

    let mut appkey = match query.get("appkey") {
        Option::Some(key) => key,
        _ => "1d8b6e7d45233436", //为了应对新的appkey,应该设定默认值
    };

    let keyword = match query.get("keyword") {
        Option::Some(key) => key.clone(),
        _ => ""
    };

    let area = match query.get("area") {
        Option::Some(area) => area.clone(),
        _ => {
            if is_th {
                "th"
            }else{
                "hk"
            }
        },
    };

    let area_num = match area {
        "cn" => 1,
        "hk" => 2,
        "tw" => 3,
        "th" => {
            appkey = "7d089525d3611b1c";    
            4
        },
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
    
    let (_,white) = match auth_user(pool,&user_info.uid,&access_key,&config).await {
        Ok(value) => value,
        Err(_) => (false,false)
    };

    if white {
        // TODO: resign
    }

    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec: Vec<(&str, &str)>;
    if is_th {
        query_vec = vec![
            ("access_key", access_key),
            ("appkey", appkey),
            ("build",query.get("build").unwrap_or("1080003")),
            ("c_locale","zh_SG"),
            ("channel","master"),
            ("device", query.get("device").unwrap_or("android")),
            ("disable_rcmd","0"),
            ("fnval",query.get("fnval").unwrap_or("976")),
            ("fnver","0"),
            ("fourk","1"),
            ("highlight","1"),
            ("keyword",keyword),
            ("lang","hans"),
            ("mobi_app","bstar_a"),
            ("platform","android"),
            ("pn","1"),
            ("ps","20"),
            ("qn","120"),
            ("s_locale","zh_SG"),
            ("sim_code","52004"),
            ("ts",&ts_string),
            ("type","7"),
        ];
        match query.get("statistics") {
            Some(value) => {
                query_vec.push(("statistics",value));
            }
            _ => (),
        }
    }else{
        query_vec = vec![
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
        1 => &config.cn_proxy_search_open,
        2 => &config.hk_proxy_search_open,
        3 => &config.tw_proxy_search_open,
        4 => &config.th_proxy_search_open,
        _ => &config.hk_proxy_search_open,
    };

    let proxy_url = match area_num {
        1 => &config.cn_proxy_search_url,
        2 => &config.hk_proxy_search_url,
        3 => &config.tw_proxy_search_url,
        4 => &config.th_proxy_search_url,
        _ => &config.hk_proxy_search_url,
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

            match body_data_json["data"]["items"].as_array_mut(){
                Some(value2) => {value2.insert(0, serde_json::from_str(&value).unwrap());},
                None => {
                    //body_data_json["data"]["items"]
                    return HttpResponse::Ok() //TODO: fix bug
                        .content_type(ContentType::json())
                        .insert_header(("From", "biliroaming-rust-server"))
                        .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                        .insert_header(("Access-Control-Allow-Credentials","true"))
                        .insert_header(("Access-Control-Allow-Methods", "GET"))
                        .body(body_data);

                },
            }
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

pub async fn get_season(req: &HttpRequest,_is_app: bool,_is_th: bool) -> impl Responder {
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

    let user_info = match getuser_list(pool, access_key, "1d8b6e7d45233436", &appkey_to_sec("1d8b6e7d45233436").unwrap(),&user_agent).await{
        Ok(value)=> value,
        Err(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(format!("{{\"code\":-2337,\"message\":\"{value}\"}}"));
        }
    };
    
    let (_,_) = match auth_user(pool,&user_info.uid,&access_key,&config).await {
        Ok(value) => value,
        Err(_) => (false,false)
    };

    let dt = Local::now();
    let ts = dt.timestamp_millis() as u64;
    let ts_string = ts.to_string();
    let mut query_vec = vec![
        ("access_key", access_key),
        ("appkey", "7d089525d3611b1c"),
        ("build",query.get("build").unwrap_or("1080003")),
        ("mobi_app","bstar_a"),
        ("season_id",query.get("season_id").unwrap_or("114514")),
        ("s_locale","zh_SG"),
        ("ts",&ts_string),
    ];

    query_vec.sort_by_key(|v| v.0);
    //let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{}",qstring::QString::new(query_vec));
    let appsec = match appkey_to_sec("7d089525d3611b1c") {
        Ok(value) => value,
        _ => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(format!("{{\"code\":-2338,\"message\":\"没有对应的appsec\"}}"));
        }
    };
    let signed_url = format!("{unsigned_url}&sign={:x}",md5::compute(format!("{unsigned_url}{appsec}")));
    let proxy_open = &config.th_proxy_playurl_open;
    let proxy_url = &config.th_proxy_playurl_url;
    let api = &config.th_app_season_api;
    let body_data = match getwebpage(&format!("{api}?{signed_url}"), proxy_open, &proxy_url,&user_agent) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("{\"code\":-2338,\"message\":\"获取失败喵\"}");
        }
    };
    if config.th_app_season_sub_open {
        let mut body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
        let season_id: Option<u64>;
        let is_result: bool;
        match &body_data_json["result"] {
            serde_json::Value::Object(value) => {
                is_result = true;
                season_id = Some(value["season_id"].as_u64().unwrap());
            },
            serde_json::Value::Null => {
                is_result = false;
                match &body_data_json["data"] {
                    serde_json::Value::Null => {season_id = None;},
                    serde_json::Value::Object(value) => {
                        season_id = Some(value["season_id"].as_u64().unwrap());
                    },
                    _ => {season_id = None;},
                }
            },
            _ => {
                is_result = false;
                season_id = None;
            },
        }
        
        match season_id {
            None => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials","true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(body_data);
            },
            Some(_) => (),
        }

        let sub_replace_str = match getwebpage(&format!("{}{}",&config.th_app_season_sub_api,season_id.unwrap()), &false, "", &user_agent){
            Ok(value) => value,
            Err(_) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials","true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(body_data);
            },
        };
        let sub_replace_json: serde_json::Value = serde_json::from_str(&sub_replace_str).unwrap();
        match sub_replace_json["code"].as_i64().unwrap() {
            0 => (),
            _ => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Tips", "Failed-to-get-subs"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials","true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(body_data);
            }
        }
        let mut index_of_replace_json = 0;
        let len_of_replace_json = sub_replace_json["data"].as_array().unwrap().len();
        while index_of_replace_json < len_of_replace_json {
            let ep:usize = sub_replace_json["data"][index_of_replace_json]["ep"].as_u64().unwrap() as usize;
            let key = sub_replace_json["data"][index_of_replace_json]["key"].as_str().unwrap();
            let lang = sub_replace_json["data"][index_of_replace_json]["lang"].as_str().unwrap();
            let url = sub_replace_json["data"][index_of_replace_json]["url"].as_str().unwrap();
            if is_result {
                let element = format!("{{\"id\":1,\"key\":\"{key}\",\"title\":\"[非官方] {lang} {}\",\"url\":\"{url}\"}}",config.th_app_season_sub_name);
                body_data_json["result"]["modules"][0]["data"]["episodes"][ep]["subtitles"].as_array_mut().unwrap().insert(0, serde_json::from_str(&element).unwrap());
            }
            index_of_replace_json += 1;
        }

        if config.aid_replace_open {
            let len_of_episodes = body_data_json["result"]["modules"][0]["data"]["episodes"].as_array().unwrap().len();
            let mut index = 0;
            while index < len_of_episodes {
                body_data_json["result"]["modules"][0]["data"]["episodes"][index].as_object_mut().unwrap().insert("aid".to_string(), serde_json::json!(&config.aid));
                index += 1;
            }
        }

        let body_data = body_data_json.to_string();

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials","true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body(body_data);

    }else{
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials","true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body(body_data);
    }   
}

async fn get_resign_accesskey(redis: &Pool,area_num: &i8,user_agent: &str,config: &BiliConfig) -> Option<String> {
    let area_num = match area_num {
        4 => 4,
        _ => 1,
    };
    let resign_info_str = match redis_get(redis, &format!("a{area_num}1101")).await {
        Some(value) => value,
        None => return None,
    };
    let resign_info_json: ResignInfo = serde_json::from_str(&resign_info_str).unwrap();
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    if resign_info_json.expire_time > ts {
        return Some(resign_info_json.access_key);
    }else{
        match area_num {
            4 => get_accesskey_from_token_th(redis,user_agent,config).await,
            _ => get_accesskey_from_token_cn(redis,user_agent,config).await,
        }
    }

}

async fn get_accesskey_from_token_th(redis: &Pool,user_agent: &str,config: &BiliConfig) -> Option<String> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let resign_info = to_resign_info(&redis_get(redis, &format!("a41101")).await.unwrap()).await;
    let access_key = resign_info.access_key;
    let refresh_token = resign_info.refresh_token;
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let request_body_string = format!("access_token={access_key}&refresh_token={refresh_token}");
    let mut request_data = request_body_string.as_bytes();
    handle.url("https://passport.biliintl.com/x/intl/passport-login/oauth2/refresh_token").unwrap();
    let mut headers = List::new();
    headers.append("Content-Type: application/x-www-form-urlencoded").unwrap();
    headers.append("charset=utf-8").unwrap();
    handle.http_headers(headers).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(true).unwrap();
    handle.post_field_size(request_data.len() as u64).unwrap();
    handle.useragent(user_agent).unwrap();
    if config.th_proxy_token_open {
        handle.proxy_type(curl::easy::ProxyType::Socks5Hostname).unwrap();
        handle.proxy(&config.th_proxy_token_url).unwrap();
    }
    {
        let mut transfer = handle.transfer();
        transfer.read_function(|into| {
            Ok(request_data.read(into).unwrap())
        }).unwrap();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        match transfer.perform() {
            Ok(()) => (()),
            _error => {
                return None;
            }
        }
    }

    let getpost_string: String = match String::from_utf8(data){
        Ok(value) => value,
        Err(_) => return None,
    };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    let resign_info = ResignInfo {
        area_num : 4,
        access_key : getpost_json["data"]["token_info"]["access_token"].as_str().unwrap().to_string(),
        refresh_token : getpost_json["data"]["token_info"]["refresh_token"].as_str().unwrap().to_string(),
        expire_time: getpost_json["data"]["token_info"]["expires_in"].as_u64().unwrap() + ts,
    };
    redis_set(redis, "a41101", &resign_info.to_json(), 0).await;
    Some(getpost_json["data"]["token_info"]["access_token"].to_string())
}

async fn get_accesskey_from_token_cn(redis: &Pool,user_agent: &str,config: &BiliConfig) -> Option<String> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let resign_info = to_resign_info(&redis_get(redis, &format!("a11101")).await.unwrap()).await;
    let access_key = resign_info.access_key;
    let refresh_token = resign_info.refresh_token;
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let mut request_body_string = format!("access_token={access_key}&appkey=1d8b6e7d45233436&refresh_token={refresh_token}&ts={ts}");
    request_body_string = format!("{request_body_string}&sign={:x}",md5::compute(format!("{request_body_string}560c52ccd288fed045859ed18bffd973")));
    let mut request_data = request_body_string.as_bytes();
    handle.url("https://passport.bilibili.com/x/passport-login/oauth2/refresh_token").unwrap();
    let mut headers = List::new();
    headers.append("Content-Type: application/x-www-form-urlencoded").unwrap();
    headers.append("charset=utf-8").unwrap();
    handle.http_headers(headers).unwrap();
    handle.follow_location(false).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(true).unwrap();
    handle.post_field_size(request_data.len() as u64).unwrap();
    handle.useragent(user_agent).unwrap();
    if config.cn_proxy_token_open {
        handle.proxy_type(curl::easy::ProxyType::Socks5Hostname).unwrap();
        handle.proxy(&config.th_proxy_token_url).unwrap();
    }
    {
        let mut transfer = handle.transfer();
        transfer.read_function(|into| {
            Ok(request_data.read(into).unwrap())
        }).unwrap();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        match transfer.perform() {
            Ok(()) => (()),
            _error => {
                return None;
            }
        }
    }

    let getpost_string: String = match String::from_utf8(data){
        Ok(value) => value,
        Err(_) => return None,
    };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    let resign_info = ResignInfo {
        area_num : 1,
        access_key : getpost_json["data"]["token_info"]["access_token"].as_str().unwrap().to_string(),
        refresh_token : getpost_json["data"]["token_info"]["refresh_token"].as_str().unwrap().to_string(),
        expire_time: getpost_json["data"]["token_info"]["expires_in"].as_u64().unwrap() + ts,
    };
    redis_set(redis, "a11101", &resign_info.to_json(), 0).await;
    Some(getpost_json["data"]["token_info"]["access_token"].to_string())
}

async fn to_resign_info(resin_info_str: &str) -> ResignInfo{
    serde_json::from_str(resin_info_str).unwrap()
}
