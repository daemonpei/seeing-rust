# Chapter 28 — Logging, Configuration and Observability

```bash
cargo run -p ch28-observability
RUST_LOG=debug cargo run -p ch28-observability
RUST_LOG=ch28_observability=trace cargo run -p ch28-observability
SERVICE=inventory PORT=notanumber cargo run -p ch28-observability
PORT=9000 cargo run -p ch28-observability
```

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## `tracing`, not `log`

`log` gives you strings at five levels. `tracing` gives you strings **plus
structured fields plus spans**:

```rust
info!(service = "inventory", port = 8080, "server starting");
```

renders as `INFO server starting service="inventory" port=8080`. Same line for a
human, one JSON object for a collector, and `service` can be filtered or
aggregated without a regex.

`#[instrument]` opens a span for the duration of the function and records its
arguments as fields. In an async service that is the difference between "a
request failed" and "this request, inside this span, at this depth, failed".

## Redact at the type level

```rust
impl std::fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClient")
            .field("base_url", &self.base_url)
            .field(
                "token",
                &if self.token.is_empty() {
                    "[unset]"
                } else {
                    "[redacted]"
                },
            )
            .finish()
    }
}
```

`#[derive(Debug)]` on a struct holding a token means every `?client`, every
`error!(?e)` and every panic message can leak it. Hand-writing `Debug` makes the
leak impossible to write by accident — and the same trick works for `Display`.

Chapter 33 adds a `Secret` newtype so the redaction travels with the value.

## Configuration: read once, at startup

```rust
fn from_env() -> Result<Self, String>
```

Environment variables are strings; the parse error is the only interesting case.
`PORT=abc` should print one clear line and exit, not panic inside a request
handler ten minutes later.

Never log the whole environment (`debug!(?env::vars())`). It contains every
credential the process was given, and log lines are the least protected data you
produce.

## Liveness versus readiness

| Probe | Question | On failure |
|---|---|---|
| liveness | is the process alive? | restart it |
| readiness | can it serve right now? | take it out of the load balancer |

A liveness probe that checks the database will restart your healthy process
during a database blip, which turns one outage into a restart loop. Keep liveness
trivial; put dependencies in readiness.

## The four metrics to add first

```
requests_total{route, status}
request_duration_seconds
in_flight_requests
errors_total{kind}
```

That set answers "is it up", "is it slow", "is it overloaded" and "what is
failing" — which covers most pages you will ever receive. Add them before the
incident, not during.
