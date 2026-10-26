# Chapter 21 — HTTP Servers and REST-Style APIs

```bash
cargo run -p ch21-http-server          # then, in another terminal:
curl localhost:8080/health
curl localhost:8080/items
curl -X POST localhost:8080/items -H 'content-type: application/json' -d '{"name":"widget"}'
curl localhost:8080/items/item-0001
```

Or run the tests, which need no socket and no server:

```bash
cargo test -p ch21-http-server
```

Dependencies:

```toml
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Routing

```rust
Router::new()
    .route("/health", get(health))
    .route("/items", get(list_items).post(create_item))
    .route("/items/:id", get(get_item))
    .with_state(state)
```

One path, several methods, each with its own handler. `:id` is a named
parameter extracted by `Path<String>`; axum 0.8 changed the syntax to `{id}`.

## State is a value you pass in

`AppState` is `Clone`, holds an `Arc<RwLock<..>>`, and is handed to every handler
by `State(state): State<AppState>`. There is no global, so a test can build a
fresh one per case — which is what makes `tests/api.rs` able to run in parallel.

## Errors as a type

```rust
impl IntoResponse for ApiError {
    fn into_response(self) -> Response { ... }
}
```

With that impl, a handler can return `Result<Json<Item>, ApiError>` and use `?`.
The error's `Display` never reaches the client; `Internal`'s payload is logged and
replaced with a generic message. That split — detail in logs, generic text on the
wire — is the whole reason to have a custom error type here.

## Middleware

```rust
.layer(middleware::from_fn(log_requests))
```

`log_requests` is an ordinary `async fn(Request, Next) -> Response`. It can read
the request, time the call, inspect the response, and short-circuit by returning
without calling `next.run(req)`. Layers apply outside-in, so the last `.layer()`
runs first.

## Testing without a port

```rust
app.oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap()).await
```

`tower::ServiceExt::oneshot` sends one request through the whole middleware stack
in-process. No port to pick, no race to wait for, no cleanup, and tests run in
parallel. Reserve real sockets for the one test that proves the server binds.
