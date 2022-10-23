use curl::easy::{Easy, List};
use deadpool_redis::{redis::cmd, Pool};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::string::String;
use std::time::Duration;
use tokio::task::spawn_blocking;

pub fn getwebpage(
    url: String,
    proxy_open: bool,
    proxy_url: String,
    user_agent: String,
    cookie: String,
) -> Result<String, ()> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(false).unwrap();
    handle.useragent(&user_agent).unwrap();
    handle.connect_timeout(Duration::new(20, 0)).unwrap();
    handle.cookie(&cookie).unwrap();

    if proxy_open {
        if proxy_url.contains("://") {
            handle.proxy(&proxy_url).unwrap();
        } else {
            handle
                .proxy_type(curl::easy::ProxyType::Socks5Hostname)
                .unwrap();
            handle.proxy(&proxy_url).unwrap();
        }
    }

    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        match transfer.perform() {
            Ok(()) => (),
            Err(value) => {
                println!("getwebpage error -> {}", value);
                return Err(());
            }
        }
    }

    let getwebpage_string: String = match String::from_utf8(data) {
        Ok(value) => value,
        Err(_) => {
            return Err(());
        }
    };
    Ok(getwebpage_string)
}

pub async fn async_getwebpage(
    url: &str,
    proxy_open: &bool,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
) -> Result<String, ()> {
    let url = url.to_owned();
    let proxy_open = proxy_open.to_owned();
    let proxy_url = proxy_url.to_owned();
    let user_agent = user_agent.to_owned();
    let cookie = cookie.to_owned();
    match spawn_blocking(move || getwebpage(url, proxy_open, proxy_url, user_agent, cookie)).await {
        Ok(value) => value,
        _ => return Err(()),
    }
}

pub fn postwebpage(
    url: String,
    content: String,
    proxy_open: bool,
    proxy_url: String,
    user_agent: String,
) -> Result<String, ()> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let mut request_data = content.as_bytes();
    let mut headers = List::new();
    headers
        .append("Content-Type: application/x-www-form-urlencoded")
        .unwrap();
    headers.append("charset=utf-8").unwrap();
    handle.http_headers(headers).unwrap();
    handle.url(&url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(true).unwrap();
    handle.post_field_size(request_data.len() as u64).unwrap();
    handle.useragent(&user_agent).unwrap();
    handle.connect_timeout(Duration::new(20, 0)).unwrap();

    if proxy_open {
        if proxy_url.contains("://") {
            handle.proxy(&proxy_url).unwrap();
        } else {
            handle
                .proxy_type(curl::easy::ProxyType::Socks5Hostname)
                .unwrap();
            handle.proxy(&proxy_url).unwrap();
        }
    }

    {
        let mut transfer = handle.transfer();
        transfer
            .read_function(|into| Ok(request_data.read(into).unwrap()))
            .unwrap();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        match transfer.perform() {
            Ok(()) => (),
            Err(value) => {
                println!("postwebpage error -> {}", value);
                return Err(());
            }
        }
    }

    let getwebpage_string: String = match String::from_utf8(data) {
        Ok(value) => value,
        Err(_) => {
            return Err(());
        }
    };
    Ok(getwebpage_string)
}

pub async fn async_postwebpage(
    url: &str,
    content: &str,
    proxy_open: &bool,
    proxy_url: &str,
    user_agent: &str,
) -> Result<String, ()> {
    let url = url.to_owned();
    let content = content.to_owned();
    let proxy_open = proxy_open.to_owned();
    let proxy_url = proxy_url.to_owned();
    let user_agent = user_agent.to_owned();
    match spawn_blocking(move || postwebpage(url, content, proxy_open, proxy_url, user_agent)).await
    {
        Ok(value) => value,
        _ => return Err(()),
    }
}

pub fn download<P: AsRef<Path>>(
    url: String,
    proxy_open: bool,
    proxy_url: String,
    user_agent: String,
    file_name: P,
) -> Result<(), ()> {
    let mut data = if let Ok(value) = OpenOptions::new().write(true).open(file_name.as_ref()) {
        value
    } else {
        println!("[Error] 无法打开文件,无法自动更新,请检查权限");
        return Err(());
    };
    //let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(false).unwrap();
    handle.useragent(&user_agent).unwrap();
    handle.connect_timeout(Duration::new(20, 0)).unwrap();

    if proxy_open {
        if proxy_url.contains("://") {
            handle.proxy(&proxy_url).unwrap();
        } else {
            handle
                .proxy_type(curl::easy::ProxyType::Socks5Hostname)
                .unwrap();
            handle.proxy(&proxy_url).unwrap();
        }
    }

    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                std::io::Write::write(&mut data, new_data).unwrap();
                Ok(new_data.len())
            })
            .unwrap();
        match transfer.perform() {
            Ok(()) => (),
            Err(err) => {
                println!("[Error] download failed: {}", err);
                return Err(());
            }
        }
    }

    Ok(())
}

pub async fn redis_get(redis: &Pool, key: &str) -> Option<String> {
    let mut conn = redis.get().await.unwrap();
    let value: String = match cmd("GET").arg(key).query_async(&mut conn).await {
        Ok(value) => value,
        Err(_) => return None,
    };
    Some(value)
}

pub async fn redis_set(redis: &Pool, key: &str, value: &str, expire_time: u64) -> Option<()> {
    // println!("key:{} value:{}", key,value);
    let mut conn = redis.get().await.unwrap();
    if expire_time != 0 {
        match cmd("SETEX")
            .arg(&[key, &format!("{expire_time}"), value])
            .query_async::<_, ()>(&mut conn)
            .await
        {
            Ok(_) => Some(()),
            _ => None,
        }
    } else {
        match cmd("SET")
            .arg(&[key, value])
            .query_async::<_, ()>(&mut conn)
            .await
        {
            Ok(_) => Some(()),
            _ => None,
        }
    }
}
