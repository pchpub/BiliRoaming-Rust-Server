use super::types::{EType, UpstreamRawResp};
use deadpool_redis::{redis::cmd, Pool};
use log::debug;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::string::String;
use std::time::Duration;

// 弃用 curl 原因: 不支持异步, 不支持brotli

// /// `getwebpage` GET请求
// /// - 返回 Result<String, bool>
// /// - E value指是否为网络问题
// pub fn getwebpage(
//     url: String,
//     proxy_open: bool,
//     proxy_url: String,
//     user_agent: String,
//     cookie: String,
//     headers: Option<List>,
// ) -> Result<UpstreamRawResp, bool> {
//     let mut resp_data = Vec::new();
//     let mut resp_header_data = Vec::new();
//     let mut handle = Easy::new();
//     handle.url(&url).unwrap();
//     handle.follow_location(true).unwrap();
//     handle.ssl_verify_peer(false).unwrap();
//     handle.post(false).unwrap();
//     handle.useragent(&user_agent).unwrap();
//     handle.cookie(&cookie).unwrap();
//     match headers {
//         Some(value) => handle.http_headers(value).unwrap(),
//         None => (),
//     }
//     handle.connect_timeout(Duration::new(20, 0)).unwrap();
//     if proxy_open {
//         if proxy_url.contains("://") {
//             handle.proxy(&proxy_url).unwrap();
//         } else {
//             handle
//                 .proxy_type(curl::easy::ProxyType::Socks5Hostname)
//                 .unwrap();
//             handle.proxy(&proxy_url).unwrap();
//         }
//     }

//     {
//         let mut transfer = handle.transfer();
//         transfer
//             .header_function(|header| {
//                 resp_header_data.extend(header.to_owned()); //为了保证速度不解析之
//                 resp_header_data.extend([226u8, 128, 161]); //分隔符: ‡
//                 true
//             })
//             .unwrap();
//         transfer
//             .write_function(|new_data| {
//                 resp_data.extend_from_slice(new_data);
//                 Ok(new_data.len())
//             })
//             .unwrap();
//         match transfer.perform() {
//             Ok(()) => (),
//             Err(value) => {
//                 debug!("[GET WEBPAGE] PROXY {proxy_open} | {proxy_url} -> ERROR: {value}",);
//                 return Err(true);
//             }
//         }
//     }

//     let getwebpage_string: String = match String::from_utf8(resp_data) {
//         Ok(value) => value,
//         Err(_) => {
//             return Err(false);
//         }
//     };
//     // debug!("测试header: \n{}", resp_header_data.join("\n"));
//     let upstream_resp = UpstreamRawResp::new(getwebpage_string, resp_header_data);
//     Ok(upstream_resp)
// }

// /// `async_getwebpage` 异步GET请求
// /// - 返回 Result<String, EType<T>>, 可 `return bili_error(E)` 将错误信息返回用户
// pub async fn async_getwebpage(
//     url: &str,
//     proxy_open: bool,
//     proxy_url: &str,
//     user_agent: &str,
//     cookie: &str,
//     headers: Option<List>,
// ) -> Result<UpstreamRawResp, EType> {
//     let url = url.to_owned();
//     let proxy_open = proxy_open.to_owned();
//     let proxy_url = proxy_url.to_owned();
//     let user_agent = user_agent.to_owned();
//     let cookie = cookie.to_owned();
//     match spawn_blocking(move || {
//         getwebpage(url, proxy_open, proxy_url, user_agent, cookie, headers)
//     })
//     .await
//     {
//         Ok(value) => match value {
//             Ok(value) => return Ok(value),
//             Err(is_network_problem) => {
//                 if is_network_problem {
//                     return Err(EType::ServerNetworkError("上游错误"));
//                 } else {
//                     return Err(EType::ServerGeneral);
//                 }
//             }
//         },
//         _ => return Err(EType::ServerGeneral),
//     }
// }

// /// `postwebpage` POST请求
// /// - 返回 Result<String, bool>
// /// - E value指是否为网络问题
// pub fn postwebpage(
//     url: String,
//     content: String,
//     proxy_open: bool,
//     proxy_url: String,
//     user_agent: String,
// ) -> Result<String, bool> {
//     let mut data = Vec::new();
//     let mut handle = Easy::new();
//     let mut request_data = content.as_bytes();
//     let mut headers = List::new();
//     headers
//         .append("Content-Type: application/x-www-form-urlencoded")
//         .unwrap();
//     headers.append("charset=utf-8").unwrap();
//     handle.http_headers(headers).unwrap();
//     handle.url(&url).unwrap();
//     handle.follow_location(true).unwrap();
//     handle.ssl_verify_peer(false).unwrap();
//     handle.post(true).unwrap();
//     handle.post_field_size(request_data.len() as u64).unwrap();
//     handle.useragent(&user_agent).unwrap();
//     handle.connect_timeout(Duration::new(20, 0)).unwrap();

//     if proxy_open {
//         if proxy_url.contains("://") {
//             handle.proxy(&proxy_url).unwrap();
//         } else {
//             handle
//                 .proxy_type(curl::easy::ProxyType::Socks5Hostname)
//                 .unwrap();
//             handle.proxy(&proxy_url).unwrap();
//         }
//     }

//     {
//         let mut transfer = handle.transfer();
//         transfer
//             .read_function(|into| Ok(request_data.read(into).unwrap()))
//             .unwrap();
//         transfer
//             .write_function(|new_data| {
//                 data.extend_from_slice(new_data);
//                 Ok(new_data.len())
//             })
//             .unwrap();
//         match transfer.perform() {
//             Ok(()) => (),
//             Err(value) => {
//                 debug!("[POST WEBPAGE] PROXY {proxy_open} | {proxy_url} -> ERROR: {value}");
//                 return Err(true);
//             }
//         }
//     }

//     let getwebpage_string: String = match String::from_utf8(data) {
//         Ok(value) => value,
//         Err(_) => {
//             return Err(false);
//         }
//     };
//     Ok(getwebpage_string)
// }

// /// `async_postwebpage` 异步POST请求
// /// - 返回 Result<String, EType<T>>, 可 `return bili_error(E)` 将错误信息返回用户
// pub async fn async_postwebpage(
//     url: &str,
//     content: &str,
//     proxy_open: bool,
//     proxy_url: &str,
//     user_agent: &str,
// ) -> Result<String, EType> {
//     let url = url.to_owned();
//     let content = content.to_owned();
//     let proxy_open = proxy_open.to_owned();
//     let proxy_url = proxy_url.to_owned();
//     let user_agent = user_agent.to_owned();
//     match spawn_blocking(move || postwebpage(url, content, proxy_open, proxy_url, user_agent)).await
//     {
//         Ok(value) => match value {
//             Ok(value) => return Ok(value),
//             Err(is_network_problem) => {
//                 if is_network_problem {
//                     return Err(EType::ServerNetworkError("上游错误"));
//                 } else {
//                     return Err(EType::ServerGeneral);
//                 }
//             }
//         },
//         _ => return Err(EType::ServerGeneral),
//     }
// }

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
    Ok(UpstreamRawResp::new(rsp_headers, rsp_body))
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
    Ok(UpstreamRawResp::new(rsp_headers, rsp_body))
}

// pub fn download<P: AsRef<Path>>(
//     url: String,
//     proxy_open: bool,
//     proxy_url: String,
//     user_agent: String,
//     file_name: P,
// ) -> Result<(), ()> {
//     let mut data = if let Ok(value) = OpenOptions::new().write(true).open(file_name.as_ref()) {
//         value
//     } else {
//         error!("[Error] 无法打开文件,无法自动更新,请检查权限");
//         return Err(());
//     };
//     //let mut data = Vec::new();
//     let mut handle = Easy::new();
//     handle.url(&url).unwrap();
//     handle.follow_location(true).unwrap();
//     handle.ssl_verify_peer(false).unwrap();
//     handle.post(false).unwrap();
//     handle.useragent(&user_agent).unwrap();
//     handle.connect_timeout(Duration::new(20, 0)).unwrap();

//     if proxy_open {
//         if proxy_url.contains("://") {
//             handle.proxy(&proxy_url).unwrap();
//         } else {
//             handle
//                 .proxy_type(curl::easy::ProxyType::Socks5Hostname)
//                 .unwrap();
//             handle.proxy(&proxy_url).unwrap();
//         }
//     }

//     {
//         let mut transfer = handle.transfer();
//         transfer
//             .write_function(|new_data| {
//                 std::io::Write::write(&mut data, new_data).unwrap();
//                 Ok(new_data.len())
//             })
//             .unwrap();
//         match transfer.perform() {
//             Ok(()) => (),
//             Err(err) => {
//                 error!("[Error] download failed: {}", err);
//                 return Err(());
//             }
//         }
//     }

//     Ok(())
// }

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
