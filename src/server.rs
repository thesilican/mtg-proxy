use crate::{
    print::{Printer, DEFAULT_PAGE_OPTIONS},
    CardOptions,
};
use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use log::{info, warn};
use serde::Deserialize;
use std::time::Instant;

/// GET /api/ping
pub async fn get_ping() -> Response {
    "pong!".into_response()
}

#[derive(Deserialize)]
pub struct PostPrintReq {
    cards: Vec<CardOptions>,
}

/// POST /api/print
pub async fn post_print(Json(req): Json<PostPrintReq>) -> Response {
    let res = Printer::print(&req.cards, &DEFAULT_PAGE_OPTIONS).await;
    match res {
        Ok(res) => {
            let mut response = Response::new(Body::from(res));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
            response
        }
        Err(err) => {
            warn!("Server error: {err}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Middleware to log HTTP requests
pub async fn log_middleware(request: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone().to_string();
    let uri = request.uri().clone().to_string();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status().as_u16() as i32;
    info!("{method} {uri} {status} {duration:?}");
    response
}
