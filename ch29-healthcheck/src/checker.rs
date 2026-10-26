use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Config {
    pub timeout: Duration,
    pub workers: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(3),
            workers: 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub url: String,
    pub status: Option<u16>,
    pub elapsed_ms: u128,
    pub error: Option<String>,
}

impl CheckResult {
    pub fn healthy(&self) -> bool {
        matches!(self.status, Some(s) if (200..300).contains(&s))
    }
}

/// One target file line per URL, blanks and `#` comments skipped.
pub fn load_targets(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect())
}

fn build_client(cfg: &Config) -> Client {
    Client::builder()
        .timeout(cfg.timeout)
        .connect_timeout(Duration::from_secs(2))
        .user_agent("healthcheck/0.1")
        .build()
        .expect("client construction cannot fail with these options")
}

pub fn check_one(client: &Client, url: &str) -> CheckResult {
    let start = Instant::now();
    let mut result = CheckResult {
        url: url.to_string(),
        status: None,
        elapsed_ms: 0,
        error: None,
    };

    match client.get(url).send() {
        Ok(resp) => {
            let code = resp.status().as_u16();
            result.status = Some(code);
            if !resp.status().is_success() {
                result.error = Some(format!("HTTP {code}"));
            }
        }
        Err(e) => result.error = Some(classify(&e)),
    }
    result.elapsed_ms = start.elapsed().as_millis();
    result
}

fn classify(e: &reqwest::Error) -> String {
    if e.is_timeout() {
        "timeout".into()
    } else if e.is_connect() {
        "connection failed".into()
    } else if e.is_decode() {
        "invalid response".into()
    } else {
        e.to_string()
    }
}

/// A worker pool over a shared queue. Threads, not async: `reqwest::blocking`
/// is what the book builds, and eight threads is the right shape for eight
/// targets. Chapter 22 is the async version of the same fan-out.
pub fn check_all(targets: &[String], cfg: &Config) -> Vec<CheckResult> {
    let client = build_client(cfg);

    let (tx, rx) = mpsc::channel::<CheckResult>();
    // `to_vec()` is not redundant here: the iterator is moved into threads that
    // must be `'static`, so it cannot borrow `targets`. Clippy only sees the
    // local `next()` call and suggests the borrowing version.
    #[allow(clippy::unnecessary_to_owned)]
    let queue = Arc::new(Mutex::new(targets.to_vec().into_iter()));

    let mut handles = Vec::new();
    for _ in 0..cfg.workers.max(1) {
        let tx = tx.clone();
        let queue = Arc::clone(&queue);
        let client = client.clone(); // cheap: shares the connection pool
        handles.push(thread::spawn(move || loop {
            let next = queue.lock().unwrap().next();
            match next {
                Some(url) => {
                    let _ = tx.send(check_one(&client, &url));
                }
                None => break,
            }
        }));
    }
    drop(tx);

    let mut results: Vec<CheckResult> = rx.into_iter().collect();
    for h in handles {
        let _ = h.join();
    }
    results.sort_by(|a, b| a.url.cmp(&b.url)); // stable output for diffing
    results
}

pub fn print_human(results: &[CheckResult]) {
    for r in results {
        if r.healthy() {
            println!(
                "OK    {:>5}ms  {:>3}  {}",
                r.elapsed_ms,
                r.status.unwrap_or(0),
                r.url
            );
        } else {
            eprintln!(
                "FAIL  {:>5}ms  {:>3}  {}  ({})",
                r.elapsed_ms,
                r.status.unwrap_or(0),
                r.url,
                r.error.as_deref().unwrap_or("unhealthy")
            );
        }
    }
    let failed = results.iter().filter(|r| !r.healthy()).count();
    eprintln!("{}/{} healthy", results.len() - failed, results.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(s: u16) -> CheckResult {
        CheckResult {
            url: "u".into(),
            status: Some(s),
            elapsed_ms: 1,
            error: None,
        }
    }

    #[test]
    fn healthy_means_2xx_only() {
        assert!(ok(200).healthy());
        assert!(ok(299).healthy());
        assert!(!ok(300).healthy());
        assert!(!ok(404).healthy());
    }

    #[test]
    fn load_targets_skips_blanks_and_comments() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("urls.txt");
        std::fs::write(&p, "http://a\n\n# comment\nhttp://b\n").unwrap();
        let t = load_targets(&p).unwrap();
        assert_eq!(t, vec!["http://a", "http://b"]);
    }
}
