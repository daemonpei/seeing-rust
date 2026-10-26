# Chapter 31 — Project 3: A Small REST API Server

```bash
cargo run -p ch31-rest-api
# other terminal
curl localhost:8080/health
curl -X POST localhost:8080/items -H 'content-type: application/json' -d '{"name":"widget"}'
curl localhost:8080/items
curl -X POST localhost:8080/items -H 'content-type: application/json' -d '{"name":"  "}'

cargo test -p ch31-rest-api
RUST_LOG=debug cargo run -p ch31-rest-api
```

## The seams, from the inside out

```
HTTP handler  (async, axum types)      -- knows about StatusCode
    |
create_item   (pure, &dyn Store)       -- knows about business rules only
    |
Store trait   (Send + Sync + 'static)  -- knows nothing
    |
MemoryStore   (RwLock<HashMap>)        -- replaceable
```

Every boundary is a type the compiler enforces. `create_item` takes `&dyn Store`,
so it cannot accidentally reach for `RwLock` or for `StatusCode`; the tests call
it directly with a 12-line fake and no tokio.

## Errors carry a field name

```json
{ "error": "must be 200 characters or fewer", "field": "name" }
```

A client that has to parse English to find out *which* field failed will break
the first time the message is reworded. `ApiError::Invalid { field, reason }`
puts the machine-readable part in its own key, and `field: null` for errors that
are not about a field.

## Character counts, not byte counts

```rust
if name.chars().count() > 200
```

`name.len()` is bytes. `"测".repeat(101)` is 101 characters and 303 bytes — a
`len() > 200` check rejects a 101-character name. For any user-facing length
limit, `chars().count()` is the one that matches what the user sees.

## Graceful shutdown, and the `cfg(unix)` trap

```rust
#[cfg(unix)]
let term = async { tokio::signal::unix::signal(SignalKind::terminate())... };
#[cfg(not(unix))]
let term = std::future::pending::<()>();
```

`tokio::signal::unix` does not exist on Windows. Without the cfg attribute the
crate does not compile there at all — and if you develop on Linux you find out
from CI, or from a user. The pattern above keeps SIGTERM on Unix and a future
that never resolves elsewhere.

`with_graceful_shutdown` lets in-flight requests finish. On Kubernetes that is
the difference between a clean rolling update and 502s on every deploy.

## What is deliberately missing

A real service would add: `PATCH`, pagination on `list`, an `ETag`, auth
middleware, and a `Store` that is not in memory. None of them change the shape
above — they are more routes, more rules in `create_item`, and a different
`impl Store`. The structure is the part worth copying.
