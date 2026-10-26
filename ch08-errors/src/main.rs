//! Chapter 8: Errors and Explicit Failure Handling

// `configured` is deliberately a literal `None`: the point is what the Option
// combinators do with it, not where it came from.
#![allow(clippy::unnecessary_literal_unwrap)]

use std::fs;
use std::num::ParseIntError;

fn main() {
    // ---- Option: absence, handled without unwrapping --------------------
    let configured: Option<u16> = None;

    let port = configured.unwrap_or(8080); // default value
    println!("unwrap_or          -> {port}");

    let port = configured.unwrap_or_else(default_port); // default computed lazily
    println!("unwrap_or_else     -> {port}");

    let doubled: Option<u32> = configured.map(|p| u32::from(p) * 2);
    println!("map                -> {doubled:?}");

    let host = lookup_host().and_then(validate);
    println!("and_then           -> {host:?}");

    // ---- Result with Box<dyn Error>: the quick path ---------------------
    println!();
    match read_port("port.txt") {
        Ok(p) => println!("read_port          -> {p}"),
        Err(e) => println!("read_port failed   -> {e}"),
    }
    match read_port("no-such-file.txt") {
        Ok(p) => println!("read_port          -> {p}"),
        Err(e) => println!("read_port failed   -> {e}"),
    }

    // ---- a real error type: the path you keep ---------------------------
    println!();
    for raw in ["8080", "99999", "eighty"] {
        match ConfigError::port(raw) {
            Ok(p) => println!("ConfigError        -> ok {p}"),
            Err(e) => println!("ConfigError        -> {e}"),
        }
    }

    // ---- a required setting: the third variant --------------------------
    println!();
    match require("SERVICE", None) {
        Ok(v) => println!("require            -> {v}"),
        Err(e) => println!("require            -> {e}"),
    }

    // ---- adding context, which is the part people skip ------------------
    println!();
    match read_port_with_context("no-such-file.txt") {
        Ok(p) => println!("context            -> {p}"),
        Err(e) => println!("context            -> {e}"),
    }
}

fn default_port() -> u16 {
    8080
}

fn lookup_host() -> Option<String> {
    Some(String::from("api-1"))
}

fn validate(host: String) -> Option<String> {
    if host.trim().is_empty() {
        None
    } else {
        Some(host)
    }
}

/// `?` converts any error into `Box<dyn Error>`, which is why this signature can
/// absorb both an `io::Error` and a `ParseIntError` without a `map_err`.
fn read_port(path: &str) -> Result<u16, Box<dyn std::error::Error>> {
    if path == "port.txt" && !std::path::Path::new(path).exists() {
        fs::write(path, "9090\n")?;
    }
    let text = fs::read_to_string(path)?; // io::Error  -> Box<dyn Error>
    let port: u16 = text.trim().parse()?; // ParseIntError -> Box<dyn Error>
    Ok(port)
}

/// The same call, with the file name attached to the failure.
fn read_port_with_context(path: &str) -> Result<u16, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("read config {}: {e}", path))?;
    text.trim()
        .parse()
        .map_err(|e: ParseIntError| format!("parse port in {path}: {e}"))
}

#[derive(Debug)]
enum ConfigError {
    MissingField(&'static str),
    NotANumber(String),
    InvalidPort(u32),
}

impl ConfigError {
    fn port(raw: &str) -> Result<u16, Self> {
        let n: u32 = raw.parse().map_err(|_| Self::NotANumber(raw.to_string()))?;
        if !(1..=65535).contains(&n) {
            return Err(Self::InvalidPort(n));
        }
        Ok(n as u16)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(name) => write!(f, "{name} is required"),
            Self::NotANumber(raw) => write!(f, "\"{raw}\" is not a number"),
            Self::InvalidPort(p) => write!(f, "port {p} is out of range"),
        }
    }
}

impl std::error::Error for ConfigError {}

fn require(name: &'static str, value: Option<String>) -> Result<String, ConfigError> {
    value.ok_or(ConfigError::MissingField(name))
}
