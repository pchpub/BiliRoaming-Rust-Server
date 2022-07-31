use deadpool_redis::Pool;
use std::string::String;
use md5;
use chrono::prelude::*;
use super::request::{redis_get,redis_set,getwebpage};
use super::types::{UserCerinfo, UserInfo, BiliConfig};

pub async fn getuser_list(redis: &Pool,access_key: &str,appkey:&str,appsec:&str,user_agent: &str) -> Result<UserInfo,String> {
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
                    // println!("getuser_list函数寄了 url:{}",url);
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
            //     token 11
            //     th subtitle 12
            //版本 ：用于处理版本更新后导致的格式变更
            //     now 01
            return Ok(output_struct);
        }
    };
    Ok(to_user_info(&info))
}

pub fn to_usercer_info(usercer_info_str: &str) -> UserCerinfo {
    serde_json::from_str(usercer_info_str).unwrap()
}

pub async fn getusercer_list(redis: &Pool,uid: &u64,access_key: &str) -> Result<UserCerinfo,()> {
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

pub async fn auth_user(redis: &Pool,uid: &u64,access_key: &str,config: &BiliConfig) -> Result<(bool,bool),String> {
    //TODO: local white&black list
    match config.local_wblist.get(&uid.to_string()) {
        Some(value) => {return Ok((value.0, value.1));},
        None => (),
    }

    match getusercer_list(redis, uid, access_key).await{
        Ok(data) => {
            if config.one_click_run {
                return Ok((!data.white, data.white));
            }
            return Ok((data.black, data.white));
        },
        Err(_) => {
            return Err("鉴权失败了喵".to_string());
        }
    };
}

pub fn appkey_to_sec(appkey:&str) -> Result<String, ()> {
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

pub fn to_user_info(user_info_str: &str) -> UserInfo {
    serde_json::from_str(user_info_str).unwrap()
    // let user_info = json::parse(user_info_str).unwrap();
    // UserInfo{
    //     access_key: user_info["access_key"].to_string(),
    //     uid: user_info["uid"].as_u64().unwrap(),
    //     vip_expire_time: user_info["vip_expire_time"].as_u64().unwrap(),
    //     expire_time: user_info["expire_time"].as_u64().unwrap(),
    // }
}