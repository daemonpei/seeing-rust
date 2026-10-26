# Chapter 22 — Async Rust, Futures and Tokio

```bash
cargo run -p ch22-async
```

## Async is a state machine, not a thread

```rust
let future = fetch();   // builds a value; runs nothing
let out = future.await; // now it runs
```

A Rust `async fn` compiles to a state machine implementing `Future`. Nothing is
executed until something polls it, and polling only happens inside an executor.
`.await` is the point where the current task yields. This is why an `async fn`
that is never awaited produces a compiler *warning* (`unused_must_use`), not
work.

Consequence: async gives you concurrency without threads. Ten thousand
in-flight HTTP requests can be ten thousand state machines on four threads,
because each one costs bytes, not a stack.

## `JoinSet` for fan-out

```rust
let mut set = JoinSet::new();
for url in urls { set.spawn(async move { check(url).await }); }
while let Some(result) = set.join_next().await { ... }
```

`join_next` yields results **in completion order**, not spawn order — which is
what you want for "report as they finish". Every result is a `Result<Result<T,
E>, JoinError>`: the outer one is "did the task panic or get cancelled", the
inner is your own error.

Spawning requires `'static` + `Send`, so `async move` and owned data.

## Timeout and cancellation

```rust
timeout(Duration::from_millis(100), slow()).await
```

`timeout` races the future against a timer and returns `Err(Elapsed)` if the
timer wins. Cancellation works because the loser is **dropped**, and dropping a
future stops it at its last `.await`. There is no cancellation signal, no flag to
poll — drop is the mechanism, and it composes with everything else.

## `select!` versus `join!`

| Macro | Finishes when | Losers |
|---|---|---|
| `select!` | the first branch completes | dropped (cancelled) |
| `join!` | all branches complete | n/a |

`select!` in a server's shutdown path is the canonical use: whichever of "ctrl-C"
or "server returned" happens first wins, and the other is cancelled.

## The three rules that prevent most async bugs

1. **Never block in async.** `std::thread::sleep`, blocking file I/O or a
   synchronous HTTP client inside a task stalls the whole worker thread. Use
   `tokio::time::sleep`, `tokio::fs`, or `spawn_blocking`.
2. **Hold locks across `.await` only deliberately.** A `std::sync::Mutex` guard
   held across an await is not `Send`, so it will not compile — that is a feature.
3. **`tokio::main` is only for `main`.** In a library, expose a plain `async fn`
   and let the caller pick the runtime.
