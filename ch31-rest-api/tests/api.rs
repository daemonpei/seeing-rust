//! The service layer is tested with a fake store; the HTTP layer with `oneshot`.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ch31_rest_api::{create_item, router, ApiError, CreateItem, Item, Store, StoreError};
use http_body_util::BodyExt;
use std::sync::Mutex;
use tower::ServiceExt;

/// 12 lines, no RwLock, no HashMap -- and it proves the handler does not care
/// how storage works.
#[derive(Default)]
struct FakeStore {
    items: Mutex<Vec<Item>>,
}

impl Store for FakeStore {
    fn list(&self) -> Vec<Item> {
        self.items.lock().unwrap().clone()
    }
    fn get(&self, id: &str) -> Option<Item> {
        self.items
            .lock()
            .unwrap()
            .iter()
            .find(|i| i.id == id)
            .cloned()
    }
    fn insert(&self, item: Item) -> Result<(), StoreError> {
        self.items.lock().unwrap().push(item);
        Ok(())
    }
}

#[test]
fn create_rejects_an_empty_name() {
    let store = FakeStore::default();
    let err = create_item(
        &store,
        CreateItem {
            name: "  ".into(),
            status: None,
        },
    )
    .unwrap_err();
    assert!(matches!(err, ApiError::Invalid { field: "name", .. }));
}

#[test]
fn create_rejects_an_unknown_status() {
    let store = FakeStore::default();
    let err = create_item(
        &store,
        CreateItem {
            name: "ok".into(),
            status: Some("deleted".into()),
        },
    )
    .unwrap_err();
    assert!(matches!(err, ApiError::Invalid { field: "status", .. }));
}

#[test]
fn create_counts_characters_not_bytes() {
    let store = FakeStore::default();
    let name = "测".repeat(201); // 201 chars, 603 bytes
    let err = create_item(
        &store,
        CreateItem {
            name,
            status: None,
        },
    )
    .unwrap_err();
    assert!(matches!(err, ApiError::Invalid { field: "name", .. }));
}

#[tokio::test]
async fn get_returns_404_for_an_unknown_id() {
    let app = router(std::sync::Arc::new(FakeStore::default()));
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/items/nope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_then_get_round_trips() {
    let app = router(std::sync::Arc::new(FakeStore::default()));

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/items")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"widget"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let id = created["id"].as_str().unwrap().to_string();
    assert_eq!(created["status"], "new");

    let resp = app
        .oneshot(
            Request::builder()
                .uri(format!("/items/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
