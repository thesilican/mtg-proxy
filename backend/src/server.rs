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
use std::{cmp::Reverse, collections::BTreeSet, sync::Arc};
use tower_http::services::ServeDir;

use crate::{database::Database, downloader::Downloader};

#[derive(Clone)]
pub struct AppState {
    pub downloader: Arc<Downloader>,
    pub database: Arc<Database>,
}

impl AppState {
    pub fn new(downloader: Downloader, database: Database) -> Self {
        AppState {
            downloader: Arc::new(downloader),
            database: Arc::new(database),
        }
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

/// Check if the API is working
pub async fn get_ping() -> impl MyResponse {
    Ok("pong!")
}

#[derive(Deserialize)]
pub struct GetSearchRequest {
    q: String,
}

/// Get cards by exact card name
pub async fn get_search(
    State(state): State<AppState>,
    Query(params): Query<GetSearchRequest>,
) -> impl MyResponse {
    let mut cards = state
        .database
        .get_cards_by_name(&params.q)
        .await
        .server_err()?;
    cards.sort_by_key(|card| Reverse(card.released_at));
    res_json!({ "cards": cards })
}

#[derive(Deserialize)]
pub struct GetAutocompleteRequest {
    q: String,
}

/// Get card names given a search term
pub async fn get_autocomplete(
    State(state): State<AppState>,
    Query(params): Query<GetAutocompleteRequest>,
) -> impl MyResponse {
    const MAX_RESPONSE_LEN: usize = 200;
    if params.q.len() == 0 {
        return res_json!({ "names": [], "exact": [] });
    }
    let search = Database::normalize_name(&params.q);
    let results = state.database.get_normal_name(&search).await.server_err()?;
    let mut output_set = BTreeSet::<String>::new();
    let mut exact_set = BTreeSet::<String>::new();
    for result in results {
        output_set.insert(result.name.clone());
        if search.name == result.name {
            exact_set.insert(result.name.clone());
        }
    }
    let output: Vec<String> = output_set.into_iter().take(MAX_RESPONSE_LEN).collect();
    let exact: Vec<String> = exact_set.into_iter().take(MAX_RESPONSE_LEN).collect();

    res_json!({ "names": output, "exact": exact })
}

pub fn build_router(app_state: AppState, public_dir: &str) -> Router {
    let serve_dir = ServeDir::new(public_dir);

    Router::new()
        .route("/api/ping", get(get_ping))
        .route("/api/autocomplete", get(get_autocomplete))
        .route("/api/search", get(get_search))
        .with_state(app_state)
        .fallback_service(serve_dir)
}
