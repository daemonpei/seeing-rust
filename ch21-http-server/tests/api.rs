//! Integration tests drive the router directly, with no socket and no server.
//! `oneshot` sends exactly one request through the tower service stack.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ch21_http_server::{router, AppState};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn body_json(resp: axum::response::Response) -> serde_json::Value {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn health_is_ok() {
    let app = router(AppState::new());
    let resp = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn unknown_item_is_404() {
    let app = router(AppState::new());
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
async fn create_then_fetch() {
    let state = AppState::new();
    let app = router(state);

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
    let created = body_json(resp).await;
    assert_eq!(created["name"], "widget");
    assert_eq!(created["status"], "new");

    let id = created["id"].as_str().unwrap();
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

#[tokio::test]
async fn empty_name_is_400() {
    let app = router(AppState::new());
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/items")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"   "}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(body_json(resp).await["error"], "name must not be empty");
}
