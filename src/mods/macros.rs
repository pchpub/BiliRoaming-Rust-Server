#[macro_export]
/// + 解析binary类型metadata.
/// + 传入参数: 头部名称, 待解析到的struct, `request`
/// + 返回struct
macro_rules! parse_grpc_header_bin {
    ($bin_name:expr, $struct:path, $request:expr) => {{
        let req_grpc_metadata_bin = $request
            .metadata()
            .get_bin($bin_name)
            .unwrap()
            .as_encoded_bytes();
        let req_grpc_metadata_bin = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD_NO_PAD,
            req_grpc_metadata_bin,
        )
        .unwrap();
        // ? 相对应地可以encode
        let req_grpc_metadata: $struct =
            prost::Message::decode(req_grpc_metadata_bin.as_slice()).unwrap();
        req_grpc_metadata
    }};
}

#[macro_export]
/// + 解析binary类型metadata.
/// + 传入参数: 头部名称, 待解析到的struct, `request`
/// + 返回struct
macro_rules! parse_grpc_bin {
    ($u8:expr, $struct:path) => {{
        let req_grpc_metadata: $struct = prost::Message::decode($u8).unwrap();
        req_grpc_metadata
    }};
}

#[macro_export]
/// + 生成binary类型metadata.
/// + 传入参数: struct
/// + 返回: MetadataValue<Ascii>
macro_rules! grpc_gen_binary_metadata_value {
    ($raw_data:expr) => {{
        let mut buffer = vec![];
        $raw_data.encode(&mut buffer).unwrap();
        tonic::metadata::BinaryMetadataValue::from_bytes(&buffer)
    }};
}

#[macro_export]
/// 便捷计算MD5
macro_rules! calc_md5 {
    ($input_str: expr) => {{
        let mut md5_instance = crypto::md5::Md5::new();
        crypto::digest::Digest::input_str(&mut md5_instance, &($input_str));
        crypto::digest::Digest::result_str(&mut md5_instance)
    }};
}

#[macro_export]
/// 从指定charset中生成指定长度的随机字符串
/// 不提供CHARSET则默认为"0123456789abcdef"
macro_rules! gen_random_string {
    ($range: expr, $charset: expr) => {{
        let mut rng = rand::thread_rng();
        (0..$range)
            .map(|_| {
                let idx = rand::Rng::gen_range(&mut rng, 0..$charset.len());
                $charset[idx] as char
            })
            .collect::<String>()
    }};
    ($range: expr) => {{
        const CHARSET: &[u8] = b"0123456789abcdef";
        let mut rng = rand::thread_rng();
        (0..$range)
            .map(|_| {
                let idx = rand::Rng::gen_range(&mut rng, 0..16);
                CHARSET[idx] as char
            })
            .collect::<String>()
    }};
}

#[macro_export]
/// 慎用
macro_rules! unsafe_str_copy {
    ($r_string: expr) => {{
        unsafe { String::from_utf8_unchecked($r_string.as_bytes().to_vec()) }
    }};
}

#[macro_export]
macro_rules! str_concat {
    ($($x:expr),*) => {
        {
            let mut string_final = String::with_capacity(512);
            $(
                string_final.push_str($x);
            )*
            string_final
        }
    };
}

#[macro_export]
/// 非空默认
macro_rules! non_nil {
    ($r_string: expr, $default_string: expr) => {{
        if $r_string.is_empty() {
            $default_string
        } else {
            $r_string
        }
    }};
}

#[macro_export]
macro_rules! javaBytetoHex {
    ($input_byte: expr) => {{
        let mut ts = Local::now().timestamp();
        for i in (0..($input_byte.len())).rev() {
            ts >>= 8;
            b_arr[i] = {
                // 滑天下之大稽...
                // 应该这样没有问题吧?
                if ((ts / 128) % 2) == 0 {
                    (ts % 256) as i8
                } else {
                    (ts % 256 - 256) as i8
                }
            }
        }
        let mut final_string = String::with_capacity(40);
        for i in 0..16 {
            // 性能耗损严重, 遍历+format!, 不是很好. 只对第12~15位进行处理为佳.
            final_string.push_str(&format!("{:0>2x}", b_arr[i]))
        }
        final_string
    }}
}

#[macro_export]
/// `build_result_response` accept Result<String, EType>
macro_rules! build_result_response {
    ($resp:ident) => {
        match $resp {
            Ok(value) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials", "true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(value);
            }
            Err(value) => {
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .insert_header(("From", "biliroaming-rust-server"))
                    .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
                    .insert_header(("Access-Control-Allow-Credentials", "true"))
                    .insert_header(("Access-Control-Allow-Methods", "GET"))
                    .body(value.to_string());
            }
        }
    };
}

#[macro_export]
/// `build_response` accepts &str, String, EType, or any that has method `to_string()`
macro_rules! build_response {
    // support enum
    ($resp:path) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body($resp.to_string())
    };
    // support value.to_string(), etc.
    ($resp:expr) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body($resp)
    };
    ($resp:ident) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body($resp)
    };
    // support like `build_response!(-412, "什么旧版本魔人,升下级");`
    ($err_code:expr, $err_msg:expr) => {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("From", "biliroaming-rust-server"))
            .insert_header(("Access-Control-Allow-Origin", "https://www.bilibili.com"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .insert_header(("Access-Control-Allow-Methods", "GET"))
            .body(format!(
                "{{\"code\":{},\"message\":\"其他错误: {}\"}}",
                $err_code, $err_msg
            ))
    };
}

#[macro_export]
/// support like `build_signed_url!(unsigned_url, vec![query_param], "sign_secret");`, return tuple (signed_url, md5_sign), mg5_sign for debug
macro_rules! build_signed_url {
    ($unsigned_url:expr, $query_vec:expr, $sign_secret:expr) => {{
        let req_params = qstring::QString::new($query_vec).to_string();
        let mut signed_url = String::with_capacity(600);
        signed_url.push_str(&($unsigned_url));
        signed_url.push_str("?");
        signed_url.push_str(&req_params);
        signed_url.push_str("&sign=");
        let mut sign = crypto::md5::Md5::new();
        crypto::digest::Digest::input_str(&mut sign, &(req_params + $sign_secret));
        let md5_sign = crypto::digest::Digest::result_str(&mut sign);
        signed_url.push_str(&md5_sign);
        (signed_url, md5_sign)
    }};
}
#[macro_export]
/// support like `build_signed_url!(unsigned_url, vec![query_param], "sign_secret");`, return tuple (signed_url, md5_sign), mg5_sign for debug
macro_rules! build_signed_params {
    ($query_vec:expr, $sign_secret:expr) => {{
        let req_params = qstring::QString::new($query_vec).to_string();
        let mut signed_params = String::with_capacity(600);
        signed_params.push_str(&req_params);
        signed_params.push_str("&sign=");
        let mut sign = crypto::md5::Md5::new();
        crypto::digest::Digest::input_str(&mut sign, &(req_params + $sign_secret));
        let md5_sign = crypto::digest::Digest::result_str(&mut sign);
        signed_params.push_str(&md5_sign);
        (signed_params, md5_sign)
    }};
}

#[macro_export]
macro_rules! build_grpc_client {
    ($grpc: path, $req_type: expr, $bili_client: expr, $bili_runtime: expr) => {{
        let hyper_https_client = {
            use crate::mods::init::GRPC_CLIENT_POOL;
            // ! 从dashmap取出连接池
            let pool = &GRPC_CLIENT_POOL.get($req_type).unwrap();
            // ! 从池中获取可用的client
            // ! 理论上是不可能失败的
            pool.get().await.unwrap()
        };
        let bili_upstream_uri = {
            let grpc_api = $req_type.grpc_api($bili_runtime.config);
            Uri::try_from(grpc_api).unwrap_or(Uri::from_static("https://grpc.biliapi.net"))
        };
        <$grpc>::with_interceptor(
            tower::service_fn(move |mut req: hyper::Request<tonic::body::BoxBody>| {
                let uri = Uri::builder()
                    .scheme(bili_upstream_uri.scheme().unwrap().clone())
                    .authority(bili_upstream_uri.authority().unwrap().clone())
                    .path_and_query(req.uri().path_and_query().unwrap().clone())
                    .build()
                    .unwrap();
                *req.uri_mut() = uri;
                hyper_https_client.request(req)
            }),
            $bili_client,
        )
        .accept_compressed(tonic::codec::CompressionEncoding::Gzip)
    }};
}
