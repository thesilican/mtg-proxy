use anyhow::Result;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use log::warn;
use reqwest::StatusCode;
use serde::Deserialize;
use std::{cmp::Reverse, sync::Arc};
use tokio::sync::{Mutex, MutexGuard};
use tower_http::services::ServeDir;

use crate::bulk_data::{BulkData, NormalString};

#[derive(Clone)]
pub struct AppState {
    bulk_data: Arc<Mutex<BulkData>>,
}

impl AppState {
    pub fn new(bulk_data: BulkData) -> Self {
        AppState {
            bulk_data: Arc::new(Mutex::new(bulk_data)),
        }
    }

    pub async fn lock(&self) -> MutexGuard<'_, BulkData> {
        self.bulk_data.lock().await
    }
}

// More narrow version of IntoResponse that only allows certain types
pub trait MyResponse: IntoResponse {}
impl<T> MyResponse for Result<T, (StatusCode, Json<String>)> where T: IntoResponse {}

// Trait to convert an anyhow::Result<T> to a HTTP 500 error
pub trait ServerErr<T>: Sized {
    fn server_err(self) -> Result<T, (StatusCode, Json<String>)>;
}

impl<T> ServerErr<T> for Result<T> {
    fn server_err(self) -> Result<T, (StatusCode, Json<String>)> {
        match self {
            Ok(x) => Ok(x),
            Err(err) => {
                warn!("{err}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ::axum::Json("internal server error".to_string()),
                ))
            }
        }
    }
}

/// Creates an Ok server result with a json object from an object that can be serialized
macro_rules! res_json {
    ($($json:tt)+) => {
        ::std::result::Result::Ok(::axum::Json(::serde_json::json!($($json)+)))
    };
}

#[derive(Deserialize)]
pub struct GetSearchRequest {
    q: String,
}

pub async fn get_search(
    State(state): State<AppState>,
    Query(params): Query<GetSearchRequest>,
) -> impl MyResponse {
    let bulk_data = state.lock().await;
    let mut output = Vec::new();
    for card in bulk_data.cards.iter() {
        if card.name == params.q {
            output.push(card.clone());
        }
    }
    output.sort_by_key(|card| Reverse(card.released_at));
    res_json!({ "data": output })
}

#[derive(Deserialize)]
pub struct GetAutocompleteRequest {
    q: String,
}

pub async fn get_autocomplete(
    State(state): State<AppState>,
    Query(params): Query<GetAutocompleteRequest>,
) -> impl MyResponse {
    if params.q.len() == 0 {
        return res_json!({ "data": [] });
    }

    let bulk_data = state.lock().await;
    let mut output = Vec::new();

    let search = NormalString::new(&params.q);
    for name in bulk_data.name_index.iter() {
        if name.normal.starts_with(&search.normal) {
            output.push(name.original.clone());
        }
    }
    output.sort();

    res_json!({ "data": output })
}

pub fn build_router(app_state: AppState, public_dir: &str) -> Router {
    let serve_dir = ServeDir::new(public_dir);

    Router::new()
        .route("/api/autocomplete", get(get_autocomplete))
        .route("/api/search", get(get_search))
        .with_state(app_state)
        .fallback_service(serve_dir)
}
