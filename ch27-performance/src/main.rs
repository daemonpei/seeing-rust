//! Chapter 27: Benchmarking, Profiling and Performance
//!
//! The book uses `criterion` (see README.md for that listing). This package
//! does the same measurement with `std::time::Instant` and `std::hint::black_box`
//! so that `cargo run` works with no extra dependency — the methodology is
//! identical, only the statistics are simpler.

use std::hint::black_box;
use std::time::{Duration, Instant};

const INPUT: &str = "level=error service=api host=10.0.0.5 code=503";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--bench") {
        bench();
        return;
    }

    println!("timing both parsers, 200_000 iterations each...\n");
    let split = time_it(|| parse_with_split(INPUT), 200_000);
    let scan = time_it(|| parse_with_scan(INPUT), 200_000);
    let regexish = time_it(|| parse_with_regex_island(INPUT), 200_000);

    println!("  split_once   {split:>10?}  -> {:?}", parse_with_split(INPUT));
    println!("  manual scan  {scan:>10?}  -> {:?}", parse_with_scan(INPUT));
    println!("  validate     {regexish:>10?}  -> {:?}", parse_with_regex_island(INPUT));

    println!();
    println!("The ordering is what matters here, not the exact numbers.");
    println!("Run with --bench for a more careful loop; see README.md for perf/flamegraph.");
}

/// `black_box` stops the optimiser from hoisting the whole call out of the loop,
/// which is how naive benchmarks end up measuring nothing.
fn time_it<F, T>(mut f: F, iterations: u32) -> Duration
where
    F: FnMut() -> T,
{
    // Warm up: first-call costs include page faults and branch prediction.
    for _ in 0..1_000 {
        black_box(f());
    }
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(f());
    }
    start.elapsed() / iterations
}

fn bench() {
    println!("--bench: 1_000_000 iterations, best of 5\n");
    for (name, f) in [
        ("split_once", parse_with_split as fn(&str) -> Option<(&str, &str)>),
        ("manual scan", parse_with_scan as fn(&str) -> Option<(&str, &str)>),
    ] {
        let mut best = Duration::from_secs(9);
        for _ in 0..5 {
            let start = Instant::now();
            for _ in 0..1_000_000 {
                black_box(f(black_box(INPUT)));
            }
            best = best.min(start.elapsed());
        }
        println!("  {name:<12} {best:>10?} / 1M");
    }
    println!();
    println!("best-of-N, not the mean: the minimum is the run least disturbed by");
    println!("the scheduler, so it is the closest thing to the true cost.");
}

/// What the standard library gives you.
fn parse_with_split(s: &str) -> Option<(&str, &str)> {
    s.split_once(' ')
}

/// The same job by hand -- usually not faster, and always more code. Included so
/// the benchmark has something to compare against.
fn parse_with_scan(s: &str) -> Option<(&str, &str)> {
    match s.as_bytes().iter().position(|b| *b == b' ') {
        Some(i) => Some((&s[..i], &s[i + 1..])),
        None => None,
    }
}

/// The cheapest useful "validation" step: no allocation, no regex.
fn parse_with_regex_island(s: &str) -> Option<&str> {
    s.strip_prefix("level=")
        .and_then(|rest| rest.split_whitespace().next())
}
