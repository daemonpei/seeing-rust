//! Chapter 20: HTTP Clients and API Consumption
//!
//! No public internet is used. The binary starts a throwaway HTTP server on
//! 127.0.0.1 in a background thread, then talks to it with a real `reqwest`
//! client — so timeouts, status handling and retries all run for real.

use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Health {
    status: String,
    #[serde(default)]
    uptime_seconds: u64,
}

fn main() {
    let server = TestServer::spawn();
    println!("local test server on {}\n", server.addr);

    // ---- build a client once, reuse it ----------------------------------
    let client = build_client();
    println!("client built: timeouts 5s / 2s, UA healthcheck/0.1\n");

    // ---- decode straight into a type ------------------------------------
    let url = server.url("/health");
    match client.get(&url).send().and_then(|r| r.error_for_status()) {
        Ok(resp) => {
            let health: Health = resp.json().expect("valid JSON");
            println!("/health      -> {} up {}s", health.status, health.uptime_seconds);
        }
        Err(e) => println!("/health failed: {e}"),
    }

    // ---- a 404 is not an error until you say so -------------------------
    let missing = server.url("/nope");
    let raw = client.get(&missing).send().expect("transport ok");
    let status = raw.status();
    let body = raw.text().unwrap_or_default();
    println!("/nope        -> {status} (body {body})");
    println!("error_for_status turns that into: {:?}", client.get(&missing).send().and_then(|r| r.error_for_status()).map(|_| ()));

    // ---- retry with exponential backoff ---------------------------------
    println!();
    let flaky = server.url("/flaky");
    match get_with_retry(&client, &flaky, 4) {
        Ok(body) => println!("/flaky       -> recovered: {body}"),
        Err(e) => println!("/flaky       -> gave up: {e}"),
    }

    println!("\nserver saw {} requests", server.requests());
}

fn build_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(5)) // whole request
        .connect_timeout(Duration::from_secs(2)) // just the handshake
        .user_agent("healthcheck/0.1")
        .build()
        .expect("client construction cannot fail with these options")
}

/// Retries only what is worth retrying: 5xx, 429, timeouts and connect errors.
/// A 4xx or a decode error is a real answer and returns immediately.
fn get_with_retry(client: &Client, url: &str, attempts: u32) -> reqwest::Result<String> {
    let mut delay = Duration::from_millis(50);
    let mut last: Option<reqwest::Error> = None;

    for attempt in 1..=attempts {
        match client.get(url).send() {
            Ok(r) if r.status().is_server_error() || r.status().as_u16() == 429 => {
                println!("  attempt {attempt}: {} -- backing off {:?}", r.status(), delay);
                last = r.error_for_status().err();
            }
            Ok(r) => {
                println!("  attempt {attempt}: {}", r.status());
                return r.text();
            }
            Err(e) if e.is_timeout() || e.is_connect() => {
                println!("  attempt {attempt}: {e} -- backing off {:?}", delay);
                last = Some(e);
            }
            Err(e) => return Err(e), // a real error: do not retry
        }
        thread::sleep(delay);
        delay *= 2; // exponential backoff
    }

    match last {
        Some(e) => Err(e),
        None => Ok(String::new()),
    }
}

// ---------------------------------------------------------------------------
// A minimal HTTP server, just enough for reqwest to talk to. Not production
// code: one request per connection, no keep-alive, no chunking.
// ---------------------------------------------------------------------------

struct TestServer {
    addr: String,
    requests: Arc<std::sync::atomic::AtomicUsize>,
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl TestServer {
    fn spawn() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        listener.set_nonblocking(true).expect("nonblocking");
        let addr = listener.local_addr().expect("addr").to_string();

        let stop = Arc::new(AtomicBool::new(false));
        let requests = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let (stop2, req2) = (Arc::clone(&stop), Arc::clone(&requests));

        let handle = thread::spawn(move || {
            let mut flaky_hits = 0u8;
            while !stop2.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        req2.fetch_add(1, Ordering::Relaxed);
                        respond(stream, &mut flaky_hits);
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(_) => break,
                }
            }
        });

        Self {
            addr,
            requests,
            stop,
            handle: Some(handle),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn requests(&self) -> usize {
        self.requests.load(Ordering::Relaxed)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

fn respond(mut stream: TcpStream, flaky_hits: &mut u8) {
    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    });

    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    // Drain the headers so the client is not left writing into a closed socket.
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap_or(0) == 0 || line == "\r\n" {
            break;
        }
    }

    let path = request_line.split_whitespace().nth(1).unwrap_or("/");
    let (code, reason, body) = match path {
        "/health" => (200, "OK", r#"{"status":"ok","uptime_seconds":42}"#.to_string()),
        "/flaky" => {
            *flaky_hits += 1;
            if *flaky_hits <= 2 {
                (503, "Service Unavailable", r#"{"status":"starting"}"#.to_string())
            } else {
                (200, "OK", r#"{"status":"ok","uptime_seconds":1}"#.to_string())
            }
        }
        _ => (404, "Not Found", r#"{"error":"not found"}"#.to_string()),
    };

    let response = format!(
        "HTTP/1.1 {code} {reason}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}
