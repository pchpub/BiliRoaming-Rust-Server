use super::types::{EType, UpstreamRawResp};
use deadpool_redis::{redis::cmd, Pool};
use log::debug;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::string::String;
use std::time::Duration;

pub async fn async_getwebpage(
    url: &str,
    proxy_open: bool,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    headers: Option<HeaderMap>,
) -> Result<UpstreamRawResp, EType> {
    let mut client_builder = reqwest::Client::builder();
    if proxy_open && proxy_url.len() != 0 {
        client_builder = client_builder.proxy(if proxy_url.contains("://") {
            if let Ok(value) = reqwest::Proxy::all(proxy_url) {
                value
            } else {
                return Err(EType::ServerReqError("服务器内部代理发生错误"));
            }
        } else {
            if let Ok(value) = reqwest::Proxy::all(format!("socks5://{}", proxy_url)) {
                value
            } else {
                return Err(EType::ServerReqError("服务器内部代理发生错误"));
            }
        });
    }
    let mut client = if let Ok(value) = client_builder
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .use_rustls_tls()
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()
    {
        value
    } else {
        return Err(EType::ServerReqError("Client build failed Step 1"));
    }
    .get(url);
    if let Some(value) = headers {
        client = client
            .headers(value)
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }else{
        client = client
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }
    let rsp_raw_data = if let Ok(value) = client.send().await {
        value
    } else {
        return Err(EType::ServerReqError("Client request failed Step 2"));
    };
    debug!(
        "[GET WEBPAGE] PROXY {proxy_open} | {proxy_url} -> STATUS CODE: {}",
        rsp_raw_data.status().as_u16()
    );
    match rsp_raw_data.status().as_u16() {
        404 | 429 => return Err(EType::ServerReqError("Client request failed Step 3")),
        _ => (),
    }
    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text().await {
        value
    } else {
        return Err(EType::ServerReqError("Client request failed Step 4"));
    };
    debug!("[GET WEBPAGE] URL {}", url);
    Ok(UpstreamRawResp::new(url, rsp_headers, rsp_body))
}

pub async fn async_postwebpage(
    url: &str,
    content: &str,
    proxy_open: bool,
    proxy_url: &str,
    user_agent: &str,
) -> Result<UpstreamRawResp, EType> {
    let mut client_builder = reqwest::Client::builder();
    if proxy_open && proxy_url.len() != 0 {
        client_builder = client_builder.proxy(if proxy_url.contains("://") {
            if let Ok(value) = reqwest::Proxy::all(proxy_url) {
                value
            } else {
                return Err(EType::ServerReqError("服务器内部代理发生错误"));
            }
        } else {
            if let Ok(value) = reqwest::Proxy::all(format!("socks5://{}", proxy_url)) {
                value
            } else {
                return Err(EType::ServerReqError("服务器内部代理发生错误"));
            }
        });
    }
    let client = if let Ok(value) = client_builder
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .use_rustls_tls()
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()
    {
        value
    } else {
        return Err(EType::ServerReqError("Client build failed Step 1"));
    }
    .post(url)
    .body(content.to_owned())
    .header("Accept-Encoding", "gzip, deflate, br")
    .header("Content-Type", "application/x-www-form-urlencoded");
    let rsp_raw_data = if let Ok(value) = client.send().await {
        value
    } else {
        return Err(EType::ServerReqError("Client request failed Step 2"));
    };
    debug!(
        "[POST WEBPAGE] PROXY {proxy_open} | {proxy_url} -> STATUS CODE: {}",
        rsp_raw_data.status().as_u16()
    );
    match rsp_raw_data.status().as_u16() {
        404 | 429 => return Err(EType::ServerReqError("Client request failed Step 3")),
        _ => (),
    }
    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text().await {
        value
    } else {
        return Err(EType::ServerReqError("Client request failed Step 4"));
    };
    Ok(UpstreamRawResp::new(url, rsp_headers, rsp_body))
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
    // debug!("key:{} value:{}", key,value);
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
