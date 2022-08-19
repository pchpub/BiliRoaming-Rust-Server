use curl::easy::Easy;
use deadpool_redis::{redis::cmd, Pool};
use tokio::task::spawn_blocking;
use std::string::String;
use std::time::Duration;

pub fn getwebpage(url: &str,proxy_open: &bool,proxy_url: &str,user_agent: &str) -> Result<String, ()> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(false).unwrap();
    handle.useragent(user_agent).unwrap();
    handle.connect_timeout(Duration::new(10, 0)).unwrap();
    
    if *proxy_open { 
        handle.proxy_type(curl::easy::ProxyType::Socks5Hostname).unwrap();
        handle.proxy(proxy_url).unwrap();
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

    let getwebpage_string: String = match String::from_utf8(data){
        Ok(value) => value,
        Err(_) => {
            return Err(());
        },
    };
    Ok(getwebpage_string)

}

pub async fn async_getwebpage(url: &str,proxy_open: &bool,proxy_url: &str,user_agent: &str) -> Result<String, ()> {
    let url = url.to_owned();
    let proxy_open = proxy_open.to_owned();
    let proxy_url = proxy_url.to_owned();
    let user_agent = user_agent.to_owned();
    match spawn_blocking(move || getwebpage(&url,&proxy_open,&proxy_url,&user_agent)).await {
        Ok(value) => value,
        _ => return Err(()),
    }
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

pub async fn redis_set(redis: &Pool,key: &str,value: &String,expire_time: u64) -> Option<()> {
    let mut conn = redis.get().await.unwrap();
    //let mut return_data: Option<()>;
    match cmd("SET")
        .arg(&[key, value])
        .query_async::<_, ()>(&mut conn)
        .await {
            Ok(_) => (),
            _ => return None,
        }
    if expire_time != 0 {
        match cmd("EXPIRE")
            .arg(&[key, &format!("{expire_time}")])
            .query_async::<_, ()>(&mut conn)
            .await {
                Ok(_) => (),
                _ => return None,
            }
        Some(())
    }else{
        Some(())
    }
    
} 