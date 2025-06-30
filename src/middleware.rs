use hyper::{Body, Request, Response};
use log::info;
use std::time::Instant;

pub async fn logging_middleware(
    req: Request<Body>,
    handler: impl FnOnce(Request<Body>) -> Response<Body>,
) -> Response<Body> {
    let start = Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    
    let response = handler(req);
    
    let duration = start.elapsed();
    info!(
        "{} {} - {} - {:?}",
        method,
        path,
        response.status(),
        duration
    );
    
    response
}

pub fn cors_headers(mut response: Response<Body>) -> Response<Body> {
    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());
    response
}
