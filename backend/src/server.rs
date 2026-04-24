use crate::{AppState, database::Card, split_normalize_name};
use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::{get, post},
};
use log::warn;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, Reverse},
    collections::{HashMap, HashSet, hash_map::Entry},
};
use tower_http::services::ServeDir;

// Narrower version of IntoResponse that limits error type
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

macro_rules! res_err {
    ($status:expr, $($msg:tt)+) => {
        Err(($status, ::axum::Json(format!($($msg)+))))
    };
}

#[derive(Serialize)]
struct ApiCardImages {
    front_jpg: String,
    back_jpg: Option<String>,
    front_png: String,
    back_png: Option<String>,
}

#[derive(Serialize)]
struct ApiCard {
    pub id: String,
    pub name: String,
    pub flavor_name: Option<String>,
    pub set: String,
    pub set_name: String,
    pub collector_number: String,
    pub images: ApiCardImages,
    pub preferred: bool,
}

impl From<Card> for ApiCard {
    fn from(value: Card) -> Self {
        ApiCard {
            id: value.id,
            name: value.name,
            flavor_name: value.flavor_name,
            set: value.set,
            set_name: value.set_name,
            collector_number: value.collector_number,
            images: ApiCardImages {
                front_jpg: value.image_front_jpg,
                back_jpg: value.image_back_jpg,
                front_png: value.image_front_png,
                back_png: value.image_back_png,
            },
            preferred: value.preferred,
        }
    }
}

/// Check if the API is working
pub async fn get_ping() -> impl MyResponse {
    Ok("pong!")
}

#[derive(Deserialize)]
pub struct GetCardsRequest {
    name: Option<String>,
    ids: Option<String>,
}

/// Get cards by exact card name or by ids
pub async fn get_cards(
    State(state): State<AppState>,
    Query(params): Query<GetCardsRequest>,
) -> impl MyResponse {
    let mut cards = match (params.name, params.ids) {
        (Some(name), None) => state.database.get_cards_by_name(&name).await.server_err()?,
        (None, Some(ids)) => {
            let mut cards = Vec::new();
            for id in ids.split(",") {
                let Some(card) = state.database.get_card(id.trim()).await.server_err()? else {
                    return res_err!(StatusCode::NOT_FOUND, "card {id} not found");
                };
                cards.push(card);
            }
            cards
        }
        _ => {
            return res_err!(
                StatusCode::BAD_REQUEST,
                "expected exactly one of `ids`, `name`"
            );
        }
    };
    cards.sort_by_key(|card| Reverse(card.released_at));
    let cards = cards.into_iter().map(ApiCard::from).collect::<Vec<_>>();
    res_json!({ "cards": cards })
}

#[derive(Deserialize)]
pub struct GetSearchRequest {
    q: String,
}

/// Get card names given a search term
pub async fn get_search(
    State(state): State<AppState>,
    Query(params): Query<GetSearchRequest>,
) -> impl MyResponse {
    if params.q.len() <= 1 {
        return res_json!({ "cards": [] });
    }
    let mut cards = state
        .database
        .get_cards_by_search(&params.q)
        .await
        .server_err()?;

    cards.sort_by(get_card_sorter(&params.q));

    // Deduplicate names, choosing the preferred printing if available
    let mut order = Vec::new();
    let mut card_map = HashMap::new();
    for card in cards {
        match card_map.entry(card.name.clone()) {
            Entry::Vacant(entry) => {
                order.push(card.name.clone());
                entry.insert(card);
            }
            Entry::Occupied(mut entry) => {
                if card.preferred {
                    *entry.get_mut() = card;
                }
            }
        }
    }

    const MAX_RESPONSE_LEN: usize = 200;
    let cards = order
        .into_iter()
        .map(|name| ApiCard::from(card_map.remove(&name).unwrap()))
        .take(MAX_RESPONSE_LEN)
        .collect::<Vec<_>>();

    res_json!({ "cards": cards })
}

/// Given a search term, return a function that will sort cards based on relevance
/// to the supplied search query.
fn get_card_sorter(search: &str) -> impl Fn(&Card, &Card) -> Ordering {
    let (first, second) = split_normalize_name(search);
    let f = first.clone();
    let s = second.clone();
    let exact = move |card: &Card| {
        let (f, s) = (f.clone(), s.clone());
        card.name.to_lowercase() == search.to_lowercase()
            || card.flavor_name.as_ref().map(|x| x.to_lowercase()) == Some(search.to_lowercase())
            || (card.normal_name_front == f && card.normal_name_back == s)
            || (card.normal_flavor_name_front == Some(f) && card.normal_flavor_name_back == s)
    };
    let start = move |card: &Card| {
        card.normal_name_front.starts_with(&search)
            || card
                .normal_name_back
                .as_deref()
                .map_or(false, |n| n.starts_with(&search))
            || card
                .normal_flavor_name_front
                .as_deref()
                .map_or(false, |n| n.starts_with(&search))
            || card
                .normal_flavor_name_back
                .as_deref()
                .map_or(false, |n| n.starts_with(&search))
    };
    move |a: &Card, b: &Card| {
        let a_score = if exact(a) {
            0
        } else if start(a) {
            1
        } else {
            2
        };
        let b_score = if exact(b) {
            0
        } else if start(b) {
            1
        } else {
            2
        };
        (a_score, &a.name).cmp(&(b_score, &b.name))
    }
}

#[derive(Deserialize)]
pub struct ImportCard {
    name: String,
    set: Option<String>,
    collector_number: Option<String>,
}

#[derive(Deserialize)]
pub struct PostImportRequest {
    cards: Vec<ImportCard>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum PostImportResponse {
    Success { success: bool, card: ApiCard },
    Fail { success: bool, message: String },
}

pub async fn post_import(
    State(state): State<AppState>,
    Json(body): Json<PostImportRequest>,
) -> impl MyResponse {
    let mut output = Vec::new();
    for search in body.cards {
        let name = search.name;
        if name.len() == 0 {
            output.push(PostImportResponse::Fail {
                success: false,
                message: "Unexpected empty card name".to_string(),
            });
            continue;
        }

        let mut results = state.database.get_cards_by_name(&name).await.server_err()?;
        if results.len() == 0 {
            // Special case for matching only the front name of a dfc
            let results_front = state
                .database
                .get_cards_by_name_front(&name)
                .await
                .server_err()?
                .into_iter()
                // Don't include art cards in result
                .filter(|x| x.name != format!("{0} // {0}", &name))
                .collect::<Vec<_>>();
            let unique_names = results_front
                .iter()
                .map(|x| x.name.clone())
                .collect::<HashSet<_>>();
            if unique_names.len() == 1 {
                results.extend(results_front.into_iter());
            }
        }

        let mut filtered_results = results
            .iter()
            .filter(|x| {
                if let Some(set) = &search.set {
                    if x.set.to_lowercase() != set.to_lowercase() {
                        return false;
                    }
                }
                if let Some(collector) = &search.collector_number {
                    if x.collector_number.to_lowercase() != collector.to_lowercase() {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();
        filtered_results.sort_by_key(|x| Reverse((x.preferred, x.released_at)));

        if let Some(&card) = filtered_results.get(0) {
            output.push(PostImportResponse::Success {
                success: true,
                card: ApiCard::from(card.clone()),
            });
            continue;
        }

        if results.len() > 0 {
            output.push(PostImportResponse::Fail {
                success: false,
                message: format!("Could not find '{name}' with correct set / collector number.",),
            });
            continue;
        }

        let mut search_results = state
            .database
            .get_cards_by_search(&name)
            .await
            .server_err()?;

        search_results.sort_by(get_card_sorter(&name));
        if let Some(result) = search_results.get(0) {
            output.push(PostImportResponse::Fail {
                success: false,
                message: format!("Could not find '{name}' (did you mean '{}').", result.name),
            });
        } else {
            output.push(PostImportResponse::Fail {
                success: false,
                message: format!("Could not find '{name}'."),
            });
        }
    }

    res_json!({ "results": output })
}

pub fn build_router(app_state: AppState, public_dir: &str) -> Router {
    let serve_dir = ServeDir::new(public_dir);
    Router::new()
        .route("/api/ping", get(get_ping))
        .route("/api/search", get(get_search))
        .route("/api/cards", get(get_cards))
        .route("/api/import", post(post_import))
        .with_state(app_state)
        .fallback_service(serve_dir)
}
