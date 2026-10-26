//! Integration tests: an external crate that can only see the public API.
//!
//! Note what is missing compared with `src/lib.rs`: no `#[cfg(test)]` module,
//! no `use super::*`. This is the caller's view of your crate.

use ch26_testing::{classify, Config};
use std::io::Write;

#[test]
fn config_rejects_a_zero_port() {
    let cfg = Config {
        service: "api".into(),
        port: 0,
    };
    assert!(cfg.validate().is_err());
}

/// Cargo builds the binary and tells the test where it is, so the test can run
/// the *real* thing with the *real* exit codes.
#[test]
fn binary_prints_help() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_ch26"))
        .arg("--help")
        .output()
        .expect("binary should run");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("ch26 <config-file>"), "got: {stdout}");
}

#[test]
fn binary_exits_one_on_an_invalid_config() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("config.txt");
    std::fs::write(&p, "service=api\nport=0\n").unwrap();

    let out = std::process::Command::new(env!("CARGO_BIN_EXE_ch26"))
        .arg(&p)
        .output()
        .expect("binary should run");

    assert_eq!(out.status.code(), Some(1));
}

/// A temporary directory per test: no shared filename, so tests can run in
/// parallel without interfering.
#[test]
fn loads_a_config_file() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("config.txt");

    let mut f = std::fs::File::create(&p).unwrap();
    writeln!(f, "service=api").unwrap();
    writeln!(f, "port=80").unwrap();

    let cfg = Config::load(&p).unwrap();
    assert_eq!(cfg.port, 80);
    assert_eq!(cfg.service, "api");
    // `dir` is dropped here, which deletes the directory.
}

#[test]
fn classify_is_reachable_from_outside() {
    assert_eq!(classify(429), "rate limited");
}
