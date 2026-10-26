//! Chapter 21: HTTP Servers and REST-Style APIs
//!
//! The library holds everything testable; `main.rs` is four lines of wiring.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Clone, Default)]
pub struct AppState {
    items: Arc<RwLock<BTreeMap<String, Item>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list(&self) -> Vec<Item> {
        // BTreeMap keeps the order stable, so responses diff cleanly.
        self.items.read().unwrap().values().cloned().collect()
    }

    pub fn get(&self, id: &str) -> Option<Item> {
        self.items.read().unwrap().get(id).cloned()
    }

    pub fn insert(&self, mut item: Item) -> Item {
        let mut map = self.items.write().unwrap();
        item.id = format!("item-{:04}", map.len() + 1);
        map.insert(item.id.clone(), item.clone());
        item
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item))
        .with_state(state)
        .layer(middleware::from_fn(log_requests))
}

async fn health() -> &'static str {
    "ok"
}

async fn list_items(State(state): State<AppState>) -> Json<Vec<Item>> {
    Json(state.list())
}

async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Item>, ApiError> {
    let item = state.get(&id).ok_or(ApiError::NotFound)?;
    Ok(Json(item))
}

async fn create_item(
    State(state): State<AppState>,
    Json(body): Json<CreateItem>,
) -> Result<(StatusCode, Json<Item>), ApiError> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(ApiError::BadRequest("name must not be empty"));
    }
    if name.chars().count() > 200 {
        return Err(ApiError::BadRequest("name is too long"));
    }

    let status = body.status.unwrap_or_else(|| "new".to_string());
    if !["new", "active", "archived"].contains(&status.as_str()) {
        return Err(ApiError::BadRequest("status must be new, active or archived"));
    }

    let item = state.insert(Item {
        id: String::new(),
        name: name.to_string(),
        status,
    });
    Ok((StatusCode::CREATED, Json(item)))
}

/// One error type for the whole API. `IntoResponse` is what lets `?` work inside
/// a handler and still produce a well-formed HTTP response.
#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest(&'static str),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found"),
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            ApiError::Internal(ref _detail) => {
                // Log the detail; never send it to the client.
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error")
            }
        };
        (code, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}

/// Middleware: an ordinary async fn whose last argument is `Next`.
async fn log_requests(req: axum::extract::Request, next: middleware::Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = std::time::Instant::now();
    let resp = next.run(req).await;
    println!("{method} {uri} -> {} in {:?}", resp.status(), start.elapsed());
    resp
}
