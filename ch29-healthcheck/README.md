# Chapter 29 — Project 1: A Health Check CLI Tool

```bash
cargo run -p ch29-healthcheck -- --selftest
cargo run -p ch29-healthcheck -- --selftest --json
printf 'http://127.0.0.1:9/\n' | cargo run -p ch29-healthcheck -- - --workers 1
cargo test -p ch29-healthcheck
```

`--selftest` starts a local server with two good and two bad endpoints, so the
whole tool runs offline and exits with a non-zero code you can inspect.

## The layout

```
src/lib.rs        pub use checker::{Config, CheckResult, check_all};
src/checker.rs    the logic -- Config, CheckResult, check_one, check_all
src/mock.rs       a test server, used by --selftest and the integration test
src/main.rs       clap, exit codes, and nothing else
tests/cli.rs      the binary's contract
```

`main.rs` is 100 lines of argument handling plus one call to `check_all`. That
split is the whole point of chapter 13: everything worth testing is in the
library, and the binary is the part you cannot unit-test.

## The worker pool

```rust
let queue = Arc::new(Mutex::new(targets.iter().cloned().collect::<IntoIter<String>>()));
for _ in 0..cfg.workers {
    handles.push(thread::spawn(move || loop {
        let next = queue.lock().unwrap().next();          // lock, take, release
        match next { Some(url) => { tx.send(check_one(&client, &url)); } None => break }
    }));
}
drop(tx);
```

Four things to notice:

1. **`queue.lock().unwrap().next()`** — the guard is a temporary and drops at the
   end of the statement, so the lock is held for one `next()`, not for the HTTP
   request.
2. **`drop(tx)`** — the original sender must go, or `rx.into_iter()` never ends.
   This is the classic mpsc hang from chapter 24.
3. **`client.clone()`** — shares the connection pool. One client per worker would
   mean one pool per worker, and no keep-alive reuse.
4. **`results.sort_by(url)`** — a HashMap or a channel gives you completion
   order, which changes between runs. Sorting makes the diff of two runs mean
   something.

## Exit codes

| Code | Meaning | Who cares |
|---|---|---|
| 0 | every target healthy | cron, CI |
| 1 | ran fine, something is down | alerting |
| 2 | could not run (bad args, no targets) | the operator |

`0` versus `1` is the distinction between "the tool broke" and "the system
broke". A health checker that exits 2 when a target is down cannot be used in an
`if` statement, because it looks identical to a misconfiguration.

## Error classification

```rust
if e.is_timeout() { "timeout" }
else if e.is_connect() { "connection failed" }
else if e.is_decode() { "invalid response" }
```

`e.to_string()` for a reqwest error is a paragraph, and the paragraph contains
the URL again. Three short categories cover 95% of failures and are greppable in
a log.
