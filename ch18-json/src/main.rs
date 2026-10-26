//! Chapter 18: JSON and Structured Data

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    service: String,
    port: u16,
    #[serde(rename = "timeoutSeconds", default = "default_timeout")]
    timeout_seconds: u64,
    #[serde(default)]
    debug: bool,
}

fn default_timeout() -> u64 {
    5
}

/// Absent and `null` are different, and a partial update needs to tell them
/// apart. `Option<T>` with `#[serde(default)]` does exactly that.
#[derive(Debug, Deserialize)]
struct Patch {
    #[serde(default)]
    pub name: Option<String>, // absent  -> None
    #[serde(default)]
    pub enabled: Option<bool>, // absent -> None, false -> Some(false)
}

/// Internally tagged: the variant name lands in a `kind` field, so the JSON is
/// self-describing and a client can route on it.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Event {
    Started { pid: u32 },
    Finished { pid: u32, ok: bool },
    Failed { reason: String },
}

const SAMPLE: &str = r#"{
  "service": "inventory-api",
  "port": 8080,
  "timeoutSeconds": 3
}"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---- from a string ---------------------------------------------------
    let cfg: Config = serde_json::from_str(SAMPLE)?;
    println!("{cfg:#?}");

    // Defaults filled the field that was absent.
    assert_eq!(cfg.timeout_seconds, 3);
    assert!(!cfg.debug);

    // ---- from a reader (a file, a socket) --------------------------------
    let file = std::io::Cursor::new(SAMPLE.as_bytes());
    let cfg: Config = serde_json::from_reader(file)?;
    println!("from_reader: {} {}", cfg.service, cfg.port);

    // ---- encode: compact for the wire, pretty for humans -----------------
    let bytes = serde_json::to_vec(&cfg)?;
    println!("\ncompact ({} bytes): {}", bytes.len(), String::from_utf8_lossy(&bytes));
    println!("\npretty:\n{}", serde_json::to_string_pretty(&cfg)?);

    // ---- a bad document gives a useful error ----------------------------
    let bad = r#"{"service":"api","port":"8080"}"#;
    match serde_json::from_str::<Config>(bad) {
        Ok(_) => unreachable!(),
        Err(e) => println!("\nrejected: {e}"),
    }

    // ---- partial updates -------------------------------------------------
    println!();
    for json in [r#"{"name": "a"}"#, r#"{"enabled": false}"#, "{}"] {
        let p: Patch = serde_json::from_str(json)?;
        println!("{json:22} -> name={:?} enabled={:?}", p.name, p.enabled);
    }

    // ---- tagged enums ----------------------------------------------------
    println!();
    for ev in [
        Event::Started { pid: 41 },
        Event::Finished { pid: 41, ok: true },
        Event::Failed {
            reason: "timeout".into(),
        },
    ] {
        println!("{}", serde_json::to_string(&ev)?);
    }

    // ---- parsing proves the shape; validation proves the meaning ---------
    println!();
    let broken = Config {
        service: "  ".into(),
        port: 0,
        timeout_seconds: 9999,
        debug: true,
    };
    match validate(&broken) {
        Ok(()) => println!("valid"),
        Err(e) => println!("invalid: {e}"),
    }

    Ok(())
}

fn validate(cfg: &Config) -> Result<(), String> {
    let mut problems = Vec::new();
    if cfg.service.trim().is_empty() {
        problems.push("service must not be empty".to_string());
    }
    if cfg.port == 0 {
        problems.push("port must be greater than zero".to_string());
    }
    if cfg.timeout_seconds > 300 {
        problems.push("timeoutSeconds must be 300 or less".to_string());
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("; "))
    }
}
