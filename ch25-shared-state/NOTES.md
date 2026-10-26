# Chapter 25 — the concurrency toolchain

Timing bugs do not appear in a debug build. These are the commands the chapter
puts in a footnote, collected:

```bash
cargo test --release            # run tests in release: different timing
cargo test --release -- --test-threads=1   # serialise, to tell a race from a
                                           # test-ordering bug

RUSTFLAGS="-Z sanitizer=thread" cargo +nightly test --target x86_64-unknown-linux-gnu
cargo miri test                 # checks unsafe code for UB
```

| Tool | Finds | Cost |
|---|---|---|
| `cargo test --release` | races that need optimisation to surface | free |
| ThreadSanitizer | data races in `unsafe` and FFI code | nightly, Linux/macOS |
| Miri | undefined behaviour, not races between threads | very slow |
| `cargo loom` | whether an interleaving is *possible* | needs a rewrite around loom |

Loom is worth knowing about: it is a model checker that explores every
interleaving of a small piece of code, so a lock-free queue can be proven
correct rather than sampled.

## Choosing a primitive

```
read-only after setup        Arc<T>            no locking at all
counters, flags              Atomic*           lock-free, one word
map/counter, short critical  Mutex<T>          one waiter at a time
config, many readers         RwLock<T>         readers in parallel
structure changes cheaply    im / dashmap      persistent or sharded
```

## The two deadlocks that actually happen

1. **Two locks, opposite order.** Thread A takes `metrics` then `config`, thread
   B takes `config` then `metrics`. Fix: one documented lock order, or one lock.
2. **Holding a guard across a channel `send` or a network call.** The guard lives
   for the whole statement. Fix: copy out, drop, then call.
