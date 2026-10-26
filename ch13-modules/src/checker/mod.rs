//! A module with children. `mod.rs` is the module's own file *and* its
//! directory's index -- the pre-2018 style, still the clearer one once a
//! module has more than one child.

pub mod report;

use crate::config::Config;

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub target: String,
    pub healthy: bool,
}

/// The crate's one public entry point. Everything below this line is an
/// implementation detail and can be reorganised without breaking callers.
pub fn run(cfg: &Config) -> Report {
    report::build(cfg)
}
