//! Chapter 25: Shared State: Arc, Mutex and Atomics

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(Debug, Clone)]
struct Config {
    port: u16,
    retries: u8,
}

fn main() {
    mutex_counter();
    rwlock_config();
    atomics();
    drop_order();

    println!("\n(cargo test --release, cargo miri test and the thread sanitizer");
    println!(" are in NOTES.md -- timing bugs only show up at -O.)");
}

/// A guard announces when it is dropped. This is exactly what happens to a
/// `MutexGuard` at the end of the statement that created it -- which is why
/// "lock, take, release" is one line and not two.
struct DropNote(&'static str);

impl Drop for DropNote {
    fn drop(&mut self) {
        println!("  dropped: {}", self.0);
    }
}

fn drop_order() {
    println!("\n-- drop order");
    {
        let _outer = DropNote("outer");
        {
            let _inner = DropNote("inner");
        } // inner dropped here
        println!("  back in the outer scope");
    } // outer dropped here
}

/// Arc shares ownership; Mutex makes the contents reachable by one thread at a
/// time. Neither alone is enough.
fn mutex_counter() {
    println!("-- Arc<Mutex<HashMap>>");
    let counts = Arc::new(Mutex::new(HashMap::<String, u32>::new()));

    let mut handles = Vec::new();
    for word in ["a", "b", "a", "c", "a"] {
        let counts = Arc::clone(&counts); // clone the POINTER, not the map
        handles.push(thread::spawn(move || {
            let mut map = counts.lock().unwrap();
            *map.entry(word.to_string()).or_insert(0) += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    let map = counts.lock().unwrap();
    let mut rows: Vec<_> = map.iter().collect();
    rows.sort();
    for (k, v) in rows {
        println!("  {k} = {v}");
    }
}

fn load_config() -> Config {
    Config {
        port: 8080,
        retries: 3,
    }
}

/// RwLock when reads vastly outnumber writes: many readers or one writer.
fn rwlock_config() {
    println!("\n-- Arc<RwLock<Config>>");
    let config = Arc::new(RwLock::new(load_config()));

    // Many threads may hold this at once:
    {
        let cfg = config.read().unwrap();
        println!("  read  port {} retries {}", cfg.port, cfg.retries);
    } // read lock released here

    // Exactly one may hold this, and only when no reader is active:
    {
        let mut cfg = config.write().unwrap();
        cfg.port = 9090;
    }

    let cfg = config.read().unwrap();
    println!("  write port {}", cfg.port);

    // The trap: holding a guard across a long operation turns an RwLock into a
    // global bottleneck, and std's RwLock is not fair, so writers can starve.
    // Copy what you need out of the guard, then drop it.
}

/// Atomics: no lock at all, because the CPU can add without one.
fn atomics() {
    println!("\n-- atomics");
    #[derive(Default)]
    struct Metrics {
        requests: AtomicU64,
        errors: AtomicU64,
        healthy: AtomicBool,
    }

    impl Metrics {
        fn record_ok(&self) {
            self.requests.fetch_add(1, Ordering::Relaxed);
            self.healthy.store(true, Ordering::Relaxed);
        }
        fn record_err(&self) {
            self.requests.fetch_add(1, Ordering::Relaxed);
            self.errors.fetch_add(1, Ordering::Relaxed);
            self.healthy.store(false, Ordering::Relaxed);
        }
    }

    let metrics = Arc::new(Metrics::default());
    let mut handles = Vec::new();
    for i in 0..8 {
        let m = Arc::clone(&metrics);
        handles.push(thread::spawn(move || {
            for n in 0..1000 {
                if (i + n) % 7 == 0 {
                    m.record_err();
                } else {
                    m.record_ok();
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    println!(
        "  requests={} errors={} healthy={}",
        metrics.requests.load(Ordering::Relaxed),
        metrics.errors.load(Ordering::Relaxed),
        metrics.healthy.load(Ordering::Relaxed)
    );

    // Ordering::Relaxed is correct here: we only need each counter to be
    // atomic, not to order anything else. Reach for Acquire/Release when a
    // counter is a flag that publishes other data.
    println!("  relaxed is fine for counts; Acquire/Release is for publishing data");
}

