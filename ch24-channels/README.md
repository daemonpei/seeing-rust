# Chapter 24 — Channels and Message Passing

```bash
cargo run -p ch24-channels
```

## "Do not communicate by sharing memory"

A channel moves ownership of the message to the receiver. Because only one
thread can own it at a time, there is no data race to reason about — the same
invariant as `&mut`, across threads.

`mpsc` is *multi-producer, single-consumer*: many `Sender` clones, exactly one
`Receiver`. Cloning the `Sender` is cheap and explicit; the `Receiver` cannot be
cloned, which is why a worker pool wraps it in `Arc<Mutex<_>>`.

## Closing is the end of iteration

```rust
for received in rx { ... }   // ends when every Sender is dropped
```

Dropping the last `Sender` closes the channel and `recv()` returns `Err`. That is
why `drop(tx)` appears in every fan-in — leaving the original `tx` alive means the
loop never ends. It is the most common mpsc deadlock, and it hangs quietly.

## Bounded is the responsible default

```rust
mpsc::sync_channel::<Job>(8)   // blocks the sender when full
mpsc::channel::<Job>()         // unbounded: grows until OOM
```

An unbounded queue is a memory leak with extra steps: it only fails when the
producer is faster than the consumer for long enough. The bound is your back
pressure, and it turns a slow consumer into a slow producer instead of a dead
process.

`sync_channel(0)` is a rendezvous channel — every send waits for a receive.

## `Arc<Mutex<Receiver>>` and its one subtlety

```rust
let job = rx.lock().unwrap().recv();
```

The guard is released at the end of the statement, so the workers take turns
receiving and then work in parallel. Hold the guard across the work and the whole
pool serialises:

```rust
let job = rx.lock().unwrap().recv();
match job { ... thread::sleep(10ms) ... }   // guard already dropped: parallel
```

If you need many consumers without the lock, `crossbeam-channel` provides a
multi-consumer `Receiver` directly. `std` deliberately does not — that is the
point at which you should look at a crate.

## Channels versus shared state

| Use a channel when | Use `Arc<Mutex<_>>` when |
|---|---|
| work flows one way | many readers of one config |
| the unit is a message | the unit is a counter or a map |
| you want back pressure | you want random access |

Chapter 25 builds the second column.
