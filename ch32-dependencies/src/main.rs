//! Chapter 32: Dependency Management and Third-Party Crates
//!
//! Build it three ways to see the feature system do its job:
//!
//! ```bash
//! cargo run -p ch32-dependencies
//! cargo run -p ch32-dependencies --no-default-features
//! cargo run -p ch32-dependencies --features pattern
//! cargo tree -p ch32-dependencies
//! ```

use std::time::Instant;

/// The zero-dependency path: no regex, hand-rolled scanning.
fn count_without_regex(haystack: &str, needle: &str) -> usize {
    haystack.match_indices(needle).count()
}

#[cfg(feature = "pattern")]
fn count_with_pattern(haystack: &str) -> usize {
    use regex::Regex;
    use std::sync::OnceLock;
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"level=error").unwrap());
    re.find_iter(haystack).count()
}

fn main() {
    println!("compiled features:");
    println!("  default    {}", cfg!(feature = "fast-path"));
    println!("  strict     {}", cfg!(feature = "strict"));
    println!("  pattern    {}", cfg!(feature = "pattern"));
    println!();

    let log = "level=error service=api\nlevel=info service=api\nlevel=error service=web\n";

    if cfg!(feature = "fast-path") {
        // A feature can select an implementation, not just a dependency.
        let start = Instant::now();
        let n = count_without_regex(log, "level=error");
        println!("fast-path: {n} matches in {:?}", start.elapsed());
    } else {
        println!("fast-path is off: using the generic path");
        println!("matches: {}", count_without_regex(log, "level=error"));
    }

    #[cfg(feature = "pattern")]
    println!("with regex: {} matches", count_with_pattern(log));

    #[cfg(not(feature = "pattern"))]
    println!("\n(rebuild with --features pattern to pull in the regex crate)");

    if cfg!(feature = "strict") {
        println!("strict mode: extra validation is on");
    }

    println!();
    println!("`cargo tree -p ch32-dependencies --all-features` prints what got pulled in.");
}
