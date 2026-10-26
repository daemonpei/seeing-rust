//! Chapter 21: HTTP Servers and REST-Style APIs

use ch21_http_server::{router, AppState};

#[tokio::main]
async fn main() {
    let app = router(AppState::new());

    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("cannot bind {addr}: {e}"));

    println!("listening on http://{addr}");
    println!("  GET  /health");
    println!("  GET  /items");
    println!("  POST /items      {{\"name\":\"widget\"}}");
    println!("  GET  /items/:id");
    println!("Ctrl-C to stop.");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
            println!("\nshutting down");
        })
        .await
        .expect("server error");
}
