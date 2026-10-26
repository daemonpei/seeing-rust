//! The CLI's contract: exit codes and output shape. Both are what a script sees.

use ch29_healthcheck::{check_all, mock::MockServer, Config};
use std::time::Duration;

#[test]
fn selftest_exits_one_because_one_target_is_down() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_healthcheck"))
        .arg("--selftest")
        .output()
        .expect("binary should run");

    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("healthy"), "got: {stderr}");
}

#[test]
fn json_mode_is_valid_json() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_healthcheck"))
        .args(["--selftest", "--json"])
        .output()
        .expect("binary should run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
    let arr = v.as_array().expect("array");
    assert_eq!(arr.len(), 4);
    assert!(arr.iter().any(|r| r["status"] == 200));
    assert!(arr.iter().any(|r| r["status"] == 503));
}

#[test]
fn check_all_sorts_by_url_so_diffs_are_stable() {
    let server = MockServer::spawn();
    let targets = vec![
        server.url("/health"),
        server.url("/down"),
        server.url("/missing"),
    ];
    let cfg = Config {
        timeout: Duration::from_secs(2),
        workers: 3,
    };
    let results = check_all(&targets, &cfg);
    let urls: Vec<&str> = results.iter().map(|r| r.url.as_str()).collect();
    let mut sorted = urls.clone();
    sorted.sort();
    assert_eq!(urls, sorted);
    assert_eq!(server.requests(), 3);
}
