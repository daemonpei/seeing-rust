# Chapter 25 — Shared State: Arc, Mutex and Atomics

```bash
cargo run -p ch25-shared-state
```

## `Arc` and `Mutex` answer different questions

| Type | Question | Cost |
|---|---|---|
| `Arc<T>` | who owns this? | one atomic refcount per clone |
| `Mutex<T>` | who may touch it now? | a futex when contended |
| `Arc<Mutex<T>>` | both | both |

`Arc` alone gives you shared read access. `Mutex` alone gives you exclusive
access to a value on one thread's stack. The combination is the only way to
mutate one value from several threads.

`Rc<RefCell<T>>` is the single-threaded version, and the compiler refuses to send
it across a thread boundary — which is exactly the protection you want.

## `Ordering::Relaxed` is right more often than you think

For a counter, relaxed is correct. The guarantee you need is "these additions do
not lose updates", and that is all `fetch_add` promises under `Relaxed`.

You need `Acquire`/`Release` when the atomic *publishes other data*:

```rust
// writer
config.store(new_cfg, Ordering::Release);
ready.store(true, Ordering::Release);
// reader
if ready.load(Ordering::Acquire) { use_config(config.load(Ordering::Relaxed)); }
```

`Acquire`/`Release` is what makes the `config` write visible before the `ready`
flag. `Relaxed` would let the reader see `ready == true` and a stale `config`,
which is the classic publication bug.

## Prefer a channel to a shared map

The `Arc<Mutex<HashMap>>` in `mutex_counter` is correct and it is also the
slowest thing in this chapter: every thread serialises on one lock for the whole
`entry().or_insert()` expression. Chapter 24's worker pool with a channel to a
single owner-thread is usually faster and always simpler to reason about.

## Guards drop at the end of the statement

```rust
let mut map = counts.lock().unwrap();
*map.entry(k).or_insert(0) += 1;
// guard lives until here
```

If a function panics while holding a `Mutex`, the lock is poisoned: every later
`lock()` returns `Err`. `.unwrap()` turns that into a cascade of panics.
`unwrap_or_else(|e| e.into_inner())` is the deliberate "I know it panicked, the
data is still consistent" escape hatch.
