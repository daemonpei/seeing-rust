//! Runs before the crate is compiled. Anything printed as `cargo:KEY=VALUE`
//! becomes available to the code through `env!`.

use std::process::Command;

fn main() {
    // A git-derived version string, with a fallback so a source tarball (no
    // .git directory) still builds.
    let sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into());

    println!("cargo:rustc-env=GIT_SHA={sha}");

    // Without this, cargo only re-runs build.rs when build.rs itself changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
}
