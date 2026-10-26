# Chapter 27 — Benchmarking, Profiling and Performance

```bash
cargo run -p ch27-performance
cargo run -p ch27-performance -- --bench
cargo run -p ch27-performance --release -- --bench   # compare: 10-50x faster
```

## Never benchmark a debug build

`cargo run` builds with `opt-level = 0`. Numbers from it are not wrong so much
as measuring a different program. Always:

```bash
cargo run --release -- ...
```

## `black_box` is not optional

```rust
for _ in 0..1_000_000 { black_box(f(black_box(INPUT))); }
```

Without it the optimiser sees a pure function with a constant argument, computes
it once, and deletes the loop. You then "benchmark" an empty loop and conclude
your code got faster after a change that did nothing — this is the single most
common benchmarking mistake in Rust.

## The release profile the chapter recommends

```toml
[profile.release]
opt-level = 3
lto = "thin"            # "fat" is smaller/faster, but much slower to link
codegen-units = 1        # better optimisation, longer builds
panic = "abort"          # smaller binary; no unwinding
strip = "symbols"        # smaller binary; keep a separate debug build

[profile.dev]
opt-level = 0
debug = true             # full debug info while developing
```

The `panic = "abort"` caveat: tests cannot catch panics, and `catch_unwind` stops
working. For a CLI or a server that dies anyway, it is a free 10-20% off the
binary size.

## With criterion, if you want statistics

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "parse"
harness = false
```

```rust
// benches/parse.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_parse(c: &mut Criterion) {
    let input = "level=error service=api host=10.0.0.5";
    c.bench_function("split_once", |b| b.iter(|| parse_with_split(black_box(input))));
    c.bench_function("scan", |b| b.iter(|| parse_with_scan(black_box(input))));
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
```

`cargo bench` then gives you a confidence interval and a regression check against
the previous run. That is worth the dependency once a benchmark is load-bearing;
`Instant` is enough to answer "is this 3x or 300x".

## Profiling, in order of usefulness

```bash
# 1. Flame graph: where is the time, in one picture
cargo install flamegraph
cargo flamegraph --bin healthcheck -- urls.txt

# 2. Sampling profiler, no recompile
perf record -g ./target/release/healthcheck urls.txt && perf report   # Linux
cargo install --force cargo-instruments && cargo instruments ...      # macOS

# 3. Allocation: who is calling malloc
valgrind --tool=massif ./target/release/healthcheck                   # Linux
heaptrack ./target/release/healthcheck                                # Linux

# 4. Binary size: which crate is 6 MB of it
cargo install cargo-bloat && cargo bloat --release --crates
```

`debug = true` in the release profile is what makes symbols available to the
profiler. Without it the flame graph is bars labelled `<unknown>`.

## The order to optimise in

1. Measure. If you have not profiled, you do not know.
2. Fix the algorithm. `O(n²)` over `O(n log n)` beats every micro-optimisation.
3. Remove allocations from the hot loop (`with_capacity`, `&str` instead of
   `String`, `entry` instead of two lookups).
4. Then, and only then, tune the profile and inline hints.
