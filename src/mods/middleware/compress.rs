use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AcceptEncoding,
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct ChangeCompressPriority;

impl<S, B> Transform<S, ServiceRequest> for ChangeCompressPriority
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ChangeCompressPriorityMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ChangeCompressPriorityMiddleware { service }))
    }
}

pub struct ChangeCompressPriorityMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ChangeCompressPriorityMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let accept_encoding = req.get_header::<AcceptEncoding>();
        if let Some(_) = accept_encoding {
            let headers = req.headers_mut();
            let mut accept_encodings = headers
                .get("Accept-Encoding")
                .unwrap()
                .to_str()
                .unwrap()
                .split(',')
                .map(|header_value| {
                    let temp = header_value
                        .split(';')
                        .map(|value| value.trim())
                        .collect::<Vec<&str>>();
                    let mut temp2 = ((&temp[0]).to_string(), 0);
                    if temp.len() == 1 {
                        temp2.1 = {
                            match temp[0] {
                                "br" => 5,
                                "zstd" => 4,
                                "gzip" => 3,
                                "deflate" => 2,
                                "identity" => 1,
                                "*" => 1,
                                _ => 0,
                            }
                        };
                    }else{
                        temp2.1 = (&temp[1].char_indices().filter(|value| value.0 >= 2).map(|value| value.1).collect::<String>().parse::<f32>().unwrap() * 10.0) as i32;
                    }
                    temp2
                })
                .filter(|header_value| header_value.1 != 0)
                .collect::<Vec<_>>();
            accept_encodings.sort_by_key(|key| key.1);
            accept_encodings.reverse();
            let accept_encodings = accept_encodings
                .iter()
                .map(|header_value| &header_value.0[..])
                .collect::<Vec<_>>();
            if accept_encodings.len() >= 2 {
                *headers.get_mut("Accept-Encoding").unwrap() =
                    accept_encodings[0].parse().unwrap();
            } else if accept_encodings.len() == 0 {
                headers.remove("Accept-Encoding");
            } else if accept_encodings[0] == "*" {
                *headers.get_mut("Accept-Encoding").unwrap() = "br".parse().unwrap();
            } else {
                *headers.get_mut("Accept-Encoding").unwrap() = accept_encodings[0].parse().unwrap();
            }
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
