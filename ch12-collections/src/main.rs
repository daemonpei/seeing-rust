//! Chapter 12: Vectors, Strings and Hash Maps

// `vec![..]` versus `Vec::new()` + push is one of the chapter's listings.
#![allow(clippy::useless_vec)]

use std::collections::HashMap;

fn main() {
    // ---- Vec -------------------------------------------------------------
    let mut ports: Vec<u16> = Vec::new();
    ports.push(80);
    ports.push(443);
    ports.extend([8080, 9090]);
    println!("{} items, capacity {}", ports.len(), ports.capacity());

    let ports = vec![80, 443, 8080, 9090]; // the shorthand
    println!("{}", ports[0]); // panics if out of range
    println!("{:?}", ports.get(99)); // None -- safe
    println!("{:?}", ports.first()); // None too, but reads better

    // ---- String vs &str --------------------------------------------------
    let owned: String = String::from("hello");
    let borrowed: &str = "hello"; // a slice into static data
    println!("{} {} {}", owned, borrowed, owned == borrowed);

    let mut s = String::with_capacity(16);
    s.push_str("hello");
    s.push('!');
    println!("{s}"); // hello!

    // Building a large string in a loop: reserve, then push.
    let mut report = String::new();
    for host in ["api-1", "api-2"] {
        report.push_str(host);
        report.push('\n');
    }
    print!("{report}");

    // ---- HashMap ---------------------------------------------------------
    let mut counts: HashMap<String, u32> = HashMap::new();

    // Three ways to increment a counter
    counts.insert("ok".to_string(), 1);
    counts
        .entry("ok".to_string())
        .and_modify(|c| *c += 1)
        .or_insert(1);
    let c = counts.entry("ok".to_string()).or_insert(0);
    *c += 1;

    for word in ["ok", "ok", "error", "ok"] {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }

    match counts.get("ok") {
        Some(n) => println!("ok seen {n} times"),
        None => println!("never seen"),
    }

    // ---- deterministic output -------------------------------------------
    // HashMap iteration order is randomised per process. Sort before printing
    // or your diff-based tests will flake.
    let mut keys: Vec<&String> = counts.keys().collect();
    keys.sort();
    for k in keys {
        println!("{k} = {}", counts[k]);
    }
}
