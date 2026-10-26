# Chapter 20 — HTTP Clients and API Consumption

```bash
cargo run -p ch20-http-client
```

```
local test server on 127.0.0.1:5xxxx

client built: timeouts 5s / 2s, UA healthcheck/0.1

/health      -> ok up 42s
/nope        -> 404 Not Found (body {"error":"not found"})
error_for_status turns that into: Err(reqwest::Error { kind: Status(404), .. })

  attempt 1: 503 Service Unavailable -- backing off 50ms
  attempt 2: 503 Service Unavailable -- backing off 100ms
  attempt 3: 200
/flaky       -> recovered: {"status":"ok","uptime_seconds":1}
```

The server is 60 lines at the bottom of `main.rs`, bound to port 0 so it never
collides with anything. That is the whole trick to testing HTTP code offline.

## Set both timeouts

```rust
.timeout(Duration::from_secs(5))         // whole request
.connect_timeout(Duration::from_secs(2)) // just the handshake
```

`timeout` alone is not enough: a TCP connect to a blackholed address can hang for
the OS default (75 seconds on Linux) before the request timeout ever starts.
`connect_timeout` bounds that phase separately.

Reuse the `Client`. It owns the connection pool, and building one per request
throws away every kept-alive socket — the most common reqwest performance bug.

## Status codes are not errors

`send()` returns `Ok` for a 404, a 500 and a 302. That is deliberate: the
transport succeeded. Call `.error_for_status()` when you want 4xx/5xx converted
into `Err`, or match on `.status()` when you need to distinguish.

## What to retry

| Response | Retry? |
|---|---|
| 5xx | yes, with backoff |
| 429 | yes — but honour `Retry-After` |
| timeout, connect error | yes |
| 4xx (other) | no — the request is wrong |
| decode error | no — the contract is wrong |

The `get_with_retry` in this package follows exactly that table, and it prints
each attempt so you can watch the 50 → 100 → 200 ms ladder.

`delay *= 2` works because `Duration: Mul<u32>`. Add jitter in production
(`delay * rand`), or a fleet of clients retries in lockstep and the spike does
the work of a small DoS.

## Testing without a network

Two options, and the chapter prefers the second:

1. Point the client at a local server (what this package does).
2. Put the call behind a trait and fake it:

```rust
trait Fetcher { fn get(&self, url: &str) -> Result<String, Error>; }
```

Option 1 tests the real client — TLS, redirects, timeouts. Option 2 tests your
parsing and classification logic, which is where the bugs actually are, at
microsecond speed. Use both; option 1 for a smoke test, option 2 for the cases.
