//! Chapter 34: Building, Releasing and Running Rust Programs

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SHA: &str = env!("GIT_SHA");

fn main() {
    // Log it at startup so a running process can always be identified.
    println!("version {VERSION}  commit {SHA}");

    println!();
    println!("profile: {}", if cfg!(debug_assertions) { "dev" } else { "release" });

    println!();
    println!("the same program, three ways to ship it:");
    println!("  1. cargo build --release");
    println!("  2. cargo build --release --target x86_64-unknown-linux-musl   (static)");
    println!("  3. docker build -t items-api .                               (see Dockerfile)");
    println!();
    println!("check the binary size difference yourself:");
    println!("  cargo build -p ch34-release");
    println!("  cargo build -p ch34-release --release");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_script_always_sets_a_sha() {
        assert!(!SHA.is_empty());
        // Either a real short sha (7+ hex chars) or the fallback.
        assert!(SHA == "unknown" || SHA.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn version_matches_the_manifest() {
        assert_eq!(VERSION, "0.1.0");
    }
}
