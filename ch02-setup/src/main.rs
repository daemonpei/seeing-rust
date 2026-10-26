//! Chapter 2: Setting Up the Rust Environment
//!
//! rustup and cargo are already installed if you are reading this, so the only
//! useful thing to run is a program that reports what your project is made of.
//! The shell commands from the chapter are in NOTES.md.

use std::env;

/// The four entries the chapter draws as a folder tree.
const LAYOUT: [(&str, &str); 4] = [
    ("Cargo.toml", "project metadata and dependencies"),
    ("Cargo.lock", "exact resolved versions (generated)"),
    ("src/main.rs", "your program starts here"),
    (".gitignore", "target/ is not committed"),
];

fn main() {
    println!("this package: {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("manifest    : {}", env!("CARGO_MANIFEST_DIR"));
    println!();

    println!("a Cargo project is these four things:");
    for (name, why) in LAYOUT {
        println!("  {:<12} # {why}", name);
    }
    println!();

    // Cargo sets these at compile time; they are constants, not runtime lookups.
    let profile = if cfg!(debug_assertions) { "dev" } else { "release" };
    println!("built in the {profile} profile");
    println!("target dir  : {}", env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target/".into()));
    println!();
    println!("try: cargo run -p ch02-setup --release   (note the different profile)");
}
