//! `use super::Report` walks up one level; `use crate::config::Config` walks to
//! the crate root. Both are absolute -- there are no `../` paths in Rust.

use super::Report;
use crate::config::Config;

pub fn build(cfg: &Config) -> Report {
    Report {
        target: format!("{}:{}", cfg.host, cfg.port()),
        // A port of 0 is impossible here: Config refused to be constructed.
        // The check is structural, not defensive.
        healthy: !cfg.host.trim().is_empty(),
    }
}

pub fn format(r: &Report) -> String {
    if r.healthy {
        format!("OK    {}", r.target)
    } else {
        format!("FAIL  {}", r.target)
    }
}
