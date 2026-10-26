//! Configuration belongs in one place, with one controlled way in.
//!
//! `port` is private. `port()` is the only reader, so the invariant "a port is
//! never 0" has exactly one place to be enforced.

use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    port: u16,
}

impl Config {
    pub fn new(host: impl Into<String>, port: u16) -> Result<Self, ConfigError> {
        if port == 0 {
            return Err(ConfigError::InvalidPort(port));
        }
        Ok(Self {
            host: host.into(),
            port,
        })
    }

    /// A controlled way in: the field itself is private.
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .ok()
            .map(|v| v.parse::<u16>())
            .transpose()
            .map_err(|_| ConfigError::InvalidPort(0))?
            .unwrap_or_else(Self::default_port);
        Self::new(host, port)
    }

    fn default_port() -> u16 {
        8080
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: Self::default_port(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidPort(u16),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPort(p) => write!(f, "port {p} is not usable"),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_zero_is_rejected_at_construction() {
        assert!(Config::new("h", 0).is_err());
    }

    #[test]
    fn defaults_come_from_one_place() {
        assert_eq!(Config::default().port(), 8080);
    }
}
