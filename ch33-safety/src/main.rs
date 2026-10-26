//! Chapter 33: Safety, Unsafe Rust and Security Basics

use std::path::{Component, Path, PathBuf};
use std::process::Command;

// ------------------------------------------------------- make bad states unrepresentable
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Port(u16);

impl Port {
    pub fn parse(raw: &str) -> Result<Self, ConfigError> {
        let n: u32 = raw
            .trim()
            .parse()
            .map_err(|_| ConfigError::NotANumber(raw.to_string()))?;
        if !(1..=65535).contains(&n) {
            return Err(ConfigError::PortOutOfRange(n));
        }
        Ok(Port(n as u16))
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}

#[derive(Debug)]
pub enum ConfigError {
    NotANumber(String),
    PortOutOfRange(u32),
    UnsafePath(String),
    MissingSecret(&'static str),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotANumber(s) => write!(f, "\"{s}\" is not a number"),
            Self::PortOutOfRange(n) => write!(f, "port {n} is out of range"),
            Self::UnsafePath(s) => write!(f, "unsafe path: {s}"),
            Self::MissingSecret(k) => write!(f, "{k} is not set"),
        }
    }
}

impl std::error::Error for ConfigError {}

// ------------------------------------------------------------------ path traversal
fn safe_join(base: &Path, untrusted: &str) -> Result<PathBuf, ConfigError> {
    let candidate = Path::new(untrusted);

    // Reject absolute paths and any parent-directory component outright.
    if candidate.is_absolute()
        || candidate
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(ConfigError::UnsafePath(untrusted.to_string()));
    }

    let full = base.join(candidate);
    // Belt and braces: confirm the result is still under the base.
    if !full.starts_with(base) {
        return Err(ConfigError::UnsafePath(untrusted.to_string()));
    }
    Ok(full)
}

// ----------------------------------------------------------------------- secrets
#[derive(Clone)]
pub struct Secret(String);

impl Secret {
    pub fn from_env(key: &'static str) -> Result<Self, ConfigError> {
        std::env::var(key)
            .map(Secret)
            .map_err(|_| ConfigError::MissingSecret(key))
    }

    /// The only way to read it is to name it, so the call sites are greppable.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secret([redacted])")
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[redacted]")
    }
}

// ------------------------------------------------------------------------- unsafe
/// # Safety
///
/// `ptr` must be valid for reads of `len` bytes and aligned for `u8`, and the
/// memory must not be mutated for the duration of the call.
unsafe fn read_bytes<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    // SAFETY: the caller has promised the preconditions above.
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

const MAX_ITEMS: usize = 1_000;
const MAX_BODY: usize = 64 * 1024;

fn check_len(len: usize) -> Result<(), String> {
    if len > MAX_BODY {
        Err(format!("body is {len} bytes; limit is {MAX_BODY}"))
    } else {
        Ok(())
    }
}

fn main() {
    // ---- a newtype makes a bad port unrepresentable ---------------------
    println!("-- Port");
    for raw in ["8080", "0", "99999", "eighty"] {
        match Port::parse(raw) {
            Ok(p) => println!("  {raw:>8} -> ok, port {}", p.get()),
            Err(e) => println!("  {raw:>8} -> {e}"),
        }
    }
    println!("  everything downstream takes a Port, so nothing downstream can");
    println!("  be handed a bad one.");

    // ---- path traversal --------------------------------------------------
    println!("\n-- safe_join");
    let base = Path::new("/srv/static");
    for name in ["logo.png", "css/app.css", "../etc/passwd", "/etc/passwd", "a/../../b"] {
        match safe_join(base, name) {
            Ok(p) => println!("  {name:>14} -> {}", p.display()),
            Err(e) => println!("  {name:>14} -> rejected ({e})"),
        }
    }

    // ---- command injection ----------------------------------------------
    println!("\n-- Command");
    // WRONG: the filename becomes shell syntax.
    //   Command::new("sh").arg("-c").arg(format!("cat {name}"));
    //
    // RIGHT: arguments are passed as separate values; no shell is involved.
    let name = "file; rm -rf /";
    let status = Command::new("rustc")
        .arg("--version")
        .arg(format!("--comment={name}")) // harmless: it stays one argv entry
        .output();
    match status {
        Ok(out) => println!(
            "  passed {name:?} as a single argv entry; rustc exited {}",
            out.status.code().unwrap_or(-1)
        ),
        Err(e) => println!("  could not run rustc: {e}"),
    }

    // SQL is the same argument, one layer down:
    //   WRONG: format!("SELECT * FROM users WHERE name = '{name}'")
    //   RIGHT: a bind parameter, escaped by the driver
    //     query_as("SELECT * FROM users WHERE name = $1").bind(&name)

    // ---- bound what you allocate ----------------------------------------
    println!("\n-- limits");
    println!("  check_len(1024)     -> {:?}", check_len(1024));
    println!("  check_len(MAX+1)    -> {:?}", check_len(MAX_BODY + 1));
    println!("  MAX_ITEMS = {MAX_ITEMS} (enforced before allocating)");
    // axum: router.layer(DefaultBodyLimit::max(MAX_BODY))

    // ---- unsafe, with the contract written down -------------------------
    println!("\n-- unsafe");
    let data = b"raw bytes";
    // SAFETY: `data` is a live local array, so the pointer is valid for
    // `data.len()` bytes for as long as `view` is alive -- which is this scope,
    // well inside `data`'s lifetime.
    let view = unsafe { read_bytes(data.as_ptr(), data.len()) };
    println!("  {}", String::from_utf8_lossy(view));

    // ---- secrets ---------------------------------------------------------
    println!("\n-- Secret");
    match std::env::var("API_TOKEN") {
        Ok(_) => println!("  API_TOKEN is set (value never printed)"),
        Err(_) => println!("  API_TOKEN is not set"),
    }
    let fake = Secret("s3cr3t".into());
    println!("  Debug:   {fake:?}");
    println!("  Display: {fake}");
    println!("  expose(): {} chars", fake.expose().len());
}
