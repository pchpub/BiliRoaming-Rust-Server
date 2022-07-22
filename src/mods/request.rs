use curl::easy::Easy;
use deadpool_redis::{redis::cmd, Pool};
use std::string::String;

pub fn getwebpage(url: &str,proxy_open: &bool,proxy_url: &str,user_agent: &str) -> Result<String, ()> {
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
    Ok(getwebpage_string)

}

pub async fn redis_get(redis: &Pool,key: &String) -> Option<String> {
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

pub async fn redis_set(redis: &Pool,key: &String,value: &String,expire_time: u64) -> Option<()> {
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