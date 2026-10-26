//! Chapter 26: Testing With Cargo
//!
//! The library holds the logic. `src/main.rs` is a thin binary, and
//! `tests/integration.rs` is the integration test that can only see this file's
//! public API.

// The out-of-bounds test indexes a `Vec` on purpose.
#![allow(clippy::useless_vec)]

use std::fs;
use std::path::Path;

pub fn is_success(code: u16) -> bool {
    (200..300).contains(&code)
}

pub fn classify(code: u16) -> &'static str {
    match code {
        200..=299 => "success",
        300..=399 => "redirect",
        400..=499 if code == 429 => "rate limited",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown",
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub service: String,
    pub port: u16,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    EmptyService,
    PortZero,
    Io(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyService => write!(f, "service must not be empty"),
            Self::PortZero => write!(f, "port must be greater than zero"),
            Self::Io(detail) => write!(f, "io: {detail}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.service.trim().is_empty() {
            return Err(ConfigError::EmptyService);
        }
        if self.port == 0 {
            return Err(ConfigError::PortZero);
        }
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let text = fs::read_to_string(path).map_err(|e| ConfigError::Io(e.to_string()))?;
        let mut service = String::new();
        let mut port = 0u16;
        for line in text.lines() {
            if let Some((k, v)) = line.split_once('=') {
                match k.trim() {
                    "service" => service = v.trim().to_string(),
                    "port" => port = v.trim().parse().map_err(|_| ConfigError::PortZero)?,
                    _ => {}
                }
            }
        }
        let cfg = Self { service, port };
        cfg.validate()?;
        Ok(cfg)
    }
}

// ---------------------------------------------------------------------------
// Unit tests: inside the crate, so they can reach private items.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_codes_are_ok() {
        assert!(is_success(200));
        assert!(is_success(204));
        assert!(is_success(299));
    }

    #[test]
    fn boundaries_are_exclusive_at_300() {
        assert!(!is_success(300));
        assert!(!is_success(199));
    }

    /// A table-driven test: one case per line, and the failure message says
    /// which line.
    #[test]
    fn classify_covers_every_range() {
        let cases = [
            ("ok", 200u16, "success"),
            ("no content", 204, "success"),
            ("redirect", 301, "redirect"),
            ("not found", 404, "client error"),
            ("rate limited", 429, "rate limited"),
            ("server error", 500, "server error"),
            ("bogus", 0, "unknown"),
        ];

        for (name, code, expected) in cases {
            assert_eq!(classify(code), expected, "case: {name} ({code})");
        }
    }

    #[test]
    fn invalid_port_is_rejected() {
        let cfg = Config {
            service: "api".into(),
            port: 0,
        };
        let err = cfg.validate().unwrap_err();
        assert!(err.to_string().contains("port"), "got: {err}");
    }

    /// `#[should_panic]` asserts the message too, so a panic for the *wrong*
    /// reason still fails the test.
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn indexing_past_the_end_panics() {
        let v = vec![1, 2, 3];
        let _ = v[9];
    }

    #[test]
    fn empty_service_is_rejected() {
        let cfg = Config {
            service: "  ".into(),
            port: 80,
        };
        assert_eq!(cfg.validate(), Err(ConfigError::EmptyService));
    }
}
