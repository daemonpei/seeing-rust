# Chapter 23 — Threads and Concurrent Work

```bash
cargo run -p ch23-threads
```

## `move` is not optional

```rust
let handle = thread::spawn(move || println!("{:?}", data));
```

A spawned thread may outlive the function that created it, so it cannot borrow
from that function's stack. `move` transfers ownership of everything the closure
captures. Without it you get `E0597: does not live long enough` — a compile
error, not a use-after-free.

## `thread::scope` removes the `'static` requirement

```rust
thread::scope(|s| {
    let a = s.spawn(|| numbers.iter().sum::<i32>());
    let b = s.spawn(|| numbers.len());
    (a.join().unwrap(), b.join().unwrap())
});
```

Scoped threads are guaranteed to be joined before `scope` returns, so they may
borrow locals. Before `std::thread::scope` (Rust 1.63) the only options were
`move` plus `Arc`, or a crate like `rayon`.

That changes what parallel code looks like: no `Arc::clone` boilerplate for a
read-only slice, and no `'static` bound leaking into your API.

## Join returns a `Result`

`join()` gives `Result<T, Box<dyn Any>>` — the `Err` case means the thread
panicked, and you can `downcast` to recover the payload. A panicking worker does
not take down the process, so a pool can report "3 of 8 workers failed" instead
of vanishing.

## How many threads?

```rust
std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)
```

Respects cgroup CPU quotas and affinity masks, which `num_cpus::get()` used to be
needed for. On a 2-core container on a 64-core host, this returns 2 — and 64
threads of contention would have been a 30x slowdown.

## When not to use threads

Each OS thread costs a stack (8 MB virtual by default) and a kernel scheduling
slot. Ten thousand of them is a design error. That is the boundary where async
(chapter 22) or a bounded worker pool (chapter 24) takes over: threads for CPU
work, async for I/O wait.
