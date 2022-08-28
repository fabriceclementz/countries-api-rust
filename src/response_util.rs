use hyper::{header::HeaderValue, Body, Response, StatusCode};
use serde_json::json;

pub fn internal_server_error() -> Response<Body> {
    let data = json!({
        "error": 500 as usize
    })
    .to_string();
    response(data, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn not_found() -> Response<Body> {
    let data = json!({
        "error": 404 as usize
    })
    .to_string();

    response(data, StatusCode::NOT_FOUND)
}

pub fn ok(data: String) -> Response<Body> {
    response(data, StatusCode::OK)
}

fn response(data: String, status_code: StatusCode) -> Response<Body> {
    let body = Body::from(data);
    let mut response = Response::new(body);
    *response.status_mut() = status_code;
    response
        .headers_mut()
        .insert("Content-Type", HeaderValue::from_static("application/json"));
    response
}
