//! Chapter 13: Crates, Modules and Project Organisation
//!
//! The crate root. `pub mod` makes a module reachable from outside; without
//! `pub` the module is private to the crate.
//!
//! ```text
//! healthcheck/
//!   src/
//!     main.rs            argument parsing, exit codes, wiring
//!     lib.rs             pub mod config; pub mod checker;
//!     config.rs          load + validate configuration
//!     checker/
//!       mod.rs           the checker's public surface
//!       report.rs        formatting results
//!   tests/
//!     cli.rs             integration tests against the binary
//! ```

pub mod checker;
pub mod config;

pub use checker::{run, Report};
pub use config::Config;
