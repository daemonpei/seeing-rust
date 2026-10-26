//! Chapter 30: Project 2 — A JSON Configuration Validator

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub service: String,
    pub port: u16,
    #[serde(rename = "timeoutSeconds", default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_env")]
    pub environment: String,
    #[serde(default)]
    pub replicas: u8,
}

fn default_timeout() -> u64 {
    5
}

fn default_env() -> String {
    "development".into()
}

/// One variant per rule, so a caller can match instead of parsing messages.
/// `PartialEq` makes the test assertions direct.
#[derive(Debug, PartialEq)]
pub enum Problem {
    MissingService,
    PortOutOfRange(u16),
    TimeoutOutOfRange(u64),
    UnknownEnvironment(String),
    TooManyReplicas(u8),
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingService => write!(f, "service must not be empty"),
            Self::PortOutOfRange(p) => write!(f, "port {p} is outside 1..=65535"),
            Self::TimeoutOutOfRange(t) => {
                write!(f, "timeoutSeconds {t} must be between 1 and 300")
            }
            Self::UnknownEnvironment(e) => write!(
                f,
                "environment \"{e}\" is not development|staging|production"
            ),
            Self::TooManyReplicas(n) => write!(f, "replicas {n} exceeds 10"),
        }
    }
}

/// Every rule runs, and every failure is reported. Stopping at the first one
/// means the operator fixes the config in as many rounds as there are mistakes.
pub fn validate(cfg: &Config) -> Vec<Problem> {
    let mut problems = Vec::new();

    if cfg.service.trim().is_empty() {
        problems.push(Problem::MissingService);
    }
    if cfg.port == 0 {
        problems.push(Problem::PortOutOfRange(cfg.port));
    }
    if cfg.timeout_seconds == 0 || cfg.timeout_seconds > 300 {
        problems.push(Problem::TimeoutOutOfRange(cfg.timeout_seconds));
    }
    if !["development", "staging", "production"].contains(&cfg.environment.as_str()) {
        problems.push(Problem::UnknownEnvironment(cfg.environment.clone()));
    }
    if cfg.replicas > 10 {
        problems.push(Problem::TooManyReplicas(cfg.replicas));
    }

    problems
}

pub fn run(path: &Path, as_json: bool) -> Result<usize, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;

    let cfg: Config = serde_json::from_str(&text)
        .map_err(|e| format!("{} is not a valid config: {e}", path.display()))?;

    let problems = validate(&cfg);

    if as_json {
        let out = serde_json::json!({
            "valid": problems.is_empty(),
            "problems": problems.iter().map(Problem::to_string).collect::<Vec<_>>(),
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&out).map_err(|e| e.to_string())?
        );
    } else {
        for p in &problems {
            eprintln!("  - {p}");
        }
        if problems.is_empty() {
            eprintln!("config is valid ({} {})", cfg.service, cfg.environment);
        }
    }
    Ok(problems.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_config() -> Config {
        Config {
            service: "api".into(),
            port: 8080,
            timeout_seconds: 5,
            environment: "production".into(),
            replicas: 2,
        }
    }

    #[test]
    fn a_good_config_has_no_problems() {
        assert!(validate(&valid_config()).is_empty());
    }

    #[test]
    fn every_rule_fires_independently() {
        let cases: Vec<(Config, Problem)> = vec![
            (
                Config {
                    service: "  ".into(),
                    ..valid_config()
                },
                Problem::MissingService,
            ),
            (
                Config {
                    port: 0,
                    ..valid_config()
                },
                Problem::PortOutOfRange(0),
            ),
            (
                Config {
                    timeout_seconds: 0,
                    ..valid_config()
                },
                Problem::TimeoutOutOfRange(0),
            ),
            (
                Config {
                    replicas: 99,
                    ..valid_config()
                },
                Problem::TooManyReplicas(99),
            ),
        ];
        for (cfg, expected) in cases {
            assert!(validate(&cfg).contains(&expected), "missing {expected}");
        }
    }

    #[test]
    fn all_problems_are_reported_not_just_the_first() {
        let bad = Config {
            service: "".into(),
            port: 0,
            timeout_seconds: 9999,
            environment: "prod".into(),
            replicas: 0,
        };
        assert_eq!(validate(&bad).len(), 4);
    }

    #[test]
    fn defaults_apply_when_fields_are_absent() {
        let cfg: Config = serde_json::from_str(r#"{"service":"a","port":80}"#).unwrap();
        assert_eq!(cfg.timeout_seconds, 5);
        assert_eq!(cfg.environment, "development");
        assert_eq!(cfg.replicas, 0);
    }

    #[test]
    fn a_string_port_is_a_parse_error_not_a_silent_zero() {
        let e = serde_json::from_str::<Config>(r#"{"service":"a","port":"80"}"#).unwrap_err();
        assert!(e.to_string().contains("invalid type"), "got: {e}");
    }
}
