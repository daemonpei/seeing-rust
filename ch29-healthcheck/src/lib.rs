//! Chapter 29: Project 1 — A Health Check CLI Tool
//!
//! ```text
//! healthcheck --file urls.txt --timeout 3 --workers 8
//! cat urls.txt | healthcheck - --json
//! ```

pub mod checker;
pub mod mock;

pub use checker::{check_all, load_targets, CheckResult, Config};
