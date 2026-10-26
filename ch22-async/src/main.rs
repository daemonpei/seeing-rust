//! Chapter 22: Async Rust, Futures and Tokio

use std::time::{Duration, Instant};
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};

async fn fetch() -> String {
    sleep(Duration::from_millis(20)).await;
    "data".to_string()
}

/// Stands in for a network check. `bad` in the url makes it fail.
async fn check(url: &str) -> Result<u16, String> {
    let delay = if url.ends_with('3') { 90 } else { 30 };
    sleep(Duration::from_millis(delay)).await;
    if url.contains("bad") {
        Err(format!("{url}: connection refused"))
    } else {
        Ok(200)
    }
}

async fn slow() -> &'static str {
    sleep(Duration::from_millis(500)).await;
    "too late"
}

#[tokio::main]
async fn main() {
    // ---- an async fn does not run until you await it ---------------------
    let start = Instant::now();
    let future = fetch(); // nothing has happened yet
    println!("created the future in {:?}", start.elapsed());
    let out = future.await; // <- the .await is what makes it run
    println!("awaited it in {:?}: {out}\n", start.elapsed());

    // ---- fan out, then collect in completion order ----------------------
    let urls = [
        "http://api-1/health",
        "http://api-2/health",
        "http://api-3/health",
        "http://bad-4/health",
    ];
    let mut set = JoinSet::new();
    for url in urls {
        set.spawn(async move { check(url).await });
    }
    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok(status)) => println!("ok:      {status}"),
            Ok(Err(e)) => println!("failed:  {e}"),
            Err(join_err) => println!("panicked: {join_err}"),
        }
    }

    // ---- a timeout is just a future that races another ------------------
    println!();
    match timeout(Duration::from_millis(100), slow()).await {
        Ok(v) => println!("finished: {v}"),
        Err(_) => println!("timed out after 100ms -- the inner future was dropped,"),
    }
    println!("which is the whole cancellation mechanism.");

    // ---- select: first one wins, the loser is dropped -------------------
    println!();
    let winner = tokio::select! {
        v = check("http://api-a/health") => format!("{v:?}"),
        v = check("http://api-b/health") => format!("{v:?}"),
    };
    println!("select picked: {winner}");

    // The chapter's server example races ctrl_c against the server future:
    //
    //   select! {
    //       _ = signal::ctrl_c() => println!("shutting down"),
    //       res = server.serve() => res?,
    //   }
    //
    // It is commented out here because a demo that waits for Ctrl-C would hang
    // `cargo run` in CI. The shape is the same one used above.
}
