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
                .get_all("Accept-Encoding")
                .map(|header_value| {
                    let mut temp = header_value
                        .to_str()
                        .unwrap()
                        .split(';')
                        .collect::<Vec<&str>>();
                    if temp.len() == 1 {
                        temp.push({
                            match temp[0] {
                                "br" => &"4",
                                "gzip" => &"3",
                                "deflate" => &"2",
                                _ => &"1",
                            }
                        });
                    }
                    temp
                })
                .filter(|header_value| header_value[1] != "0")
                .collect::<Vec<_>>();
            accept_encodings.sort_by_key(|key| (key[1].parse::<f32>().unwrap() * 10.0) as i32);
            accept_encodings.reverse();
            let accept_encodings = accept_encodings
                .iter()
                .map(|header_value| header_value[0])
                .collect::<Vec<_>>();
            *headers.get_mut("Accept-Encoding").unwrap() =
                accept_encodings.join(",").parse().unwrap();
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
