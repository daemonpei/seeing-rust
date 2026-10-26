//! Chapter 31: Project 3 — A Small REST API Server
//!
//! The service layer depends on a trait, not on a `HashMap`. That one decision is
//! what lets the tests run in microseconds with a fake store, and what lets you
//! swap in Postgres later without touching a handler.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
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

#[derive(Debug, PartialEq)]
pub enum StoreError {
    Duplicate,
    Backend(String),
}

/// `Send + Sync + 'static` because `Arc<dyn Store>` is shared across tasks.
pub trait Store: Send + Sync + 'static {
    fn list(&self) -> Vec<Item>;
    fn get(&self, id: &str) -> Option<Item>;
    fn insert(&self, item: Item) -> Result<(), StoreError>;
}

// --------------------------------------------------------------------- memory
pub struct MemoryStore {
    items: RwLock<HashMap<String, Item>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            items: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Store for MemoryStore {
    fn list(&self) -> Vec<Item> {
        let mut all: Vec<_> = self.items.read().unwrap().values().cloned().collect();
        all.sort_by(|a, b| a.id.cmp(&b.id)); // deterministic output
        all
    }
    fn get(&self, id: &str) -> Option<Item> {
        self.items.read().unwrap().get(id).cloned()
    }
    fn insert(&self, item: Item) -> Result<(), StoreError> {
        let mut map = self.items.write().unwrap();
        if map.contains_key(&item.id) {
            return Err(StoreError::Duplicate);
        }
        map.insert(item.id.clone(), item);
        Ok(())
    }
}

// ---------------------------------------------------------------------- errors
#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Invalid {
        field: &'static str,
        reason: String,
    },
    Conflict(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, field, msg) = match &self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, None, "not found".to_string()),
            ApiError::Invalid { field, reason } => {
                (StatusCode::BAD_REQUEST, Some(*field), reason.clone())
            }
            ApiError::Conflict(_) => (StatusCode::CONFLICT, None, "already exists".into()),
            ApiError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                "internal error".into(),
            ),
        };

        // Log the detail; never send it to the client.
        if let ApiError::Internal(detail) = &self {
            tracing::error!(detail, "request failed");
        }

        let body = axum::Json(serde_json::json!({
            "error": msg,
            "field": field,
        }));
        (code, body).into_response()
    }
}

// ------------------------------------------------------------------- service
/// The business rule, with no HTTP in it. Note the `&dyn Store` -- this
/// function can be called from a test with any store at all.
pub fn create_item(store: &dyn Store, body: CreateItem) -> Result<Item, ApiError> {
    let name = body.name.trim();

    if name.is_empty() {
        return Err(ApiError::Invalid {
            field: "name",
            reason: "must not be empty".into(),
        });
    }
    if name.chars().count() > 200 {
        return Err(ApiError::Invalid {
            field: "name",
            reason: "must be 200 characters or fewer".into(),
        });
    }

    let status = body.status.unwrap_or_else(|| "new".to_string());
    if !["new", "active", "archived"].contains(&status.as_str()) {
        return Err(ApiError::Invalid {
            field: "status",
            reason: "must be new, active or archived".into(),
        });
    }

    let item = Item {
        id: new_id(),
        name: name.to_string(),
        status,
    };

    store.insert(item.clone()).map_err(|e| match e {
        StoreError::Duplicate => ApiError::Conflict(item.id.clone()),
        StoreError::Backend(d) => ApiError::Internal(d),
    })?;

    Ok(item)
}

fn new_id() -> String {
    static N: AtomicU64 = AtomicU64::new(1);
    format!("item-{:04}", N.fetch_add(1, Ordering::Relaxed))
}

// ---------------------------------------------------------------------- http
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<dyn Store>,
}

pub fn router(store: Arc<dyn Store>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/items", get(list).post(create))
        .route("/items/:id", get(one))
        .with_state(AppState { store })
        .layer(middleware::from_fn(log_requests))
}

async fn health() -> &'static str {
    "ok"
}

async fn list(State(st): State<AppState>) -> Json<Vec<Item>> {
    Json(st.store.list())
}

async fn one(State(st): State<AppState>, Path(id): Path<String>) -> Result<Json<Item>, ApiError> {
    st.store.get(&id).map(Json).ok_or(ApiError::NotFound)
}

async fn create(
    State(st): State<AppState>,
    Json(body): Json<CreateItem>,
) -> Result<(StatusCode, Json<Item>), ApiError> {
    let item = create_item(st.store.as_ref(), body)?;
    Ok((StatusCode::CREATED, Json(item)))
}

async fn log_requests(req: axum::extract::Request, next: middleware::Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = std::time::Instant::now();
    let resp = next.run(req).await;
    tracing::info!(%method, %uri, status = resp.status().as_u16(), ms = start.elapsed().as_millis() as u64, "request");
    resp
}
