//! Chapter 17: Text Processing and Regular Expressions

use regex::Regex;
use std::sync::OnceLock;

fn main() {
    // ---- the &str toolkit, with what each one allocates -------------------
    let line = "   level=error service=api  ";

    let clean = line.trim(); // &str, no allocation
    println!("trim          {clean:?}");
    println!("contains      {}", clean.contains("error"));
    println!("starts_with   {}", clean.starts_with("level="));
    println!("ends_with     {}", clean.ends_with("api"));
    println!("words         {}", clean.split_whitespace().count());
    println!("split_once    {:?}", clean.split_once('='));
    println!("replace       {:?}", clean.replace("api", "worker")); // new String
    println!("strip_prefix  {:?}", clean.strip_prefix("level="));
    println!("to_uppercase  {:?}", clean.to_uppercase()); // new String

    // ---- building a String -------------------------------------------------
    let mut s = String::with_capacity(64);
    s.push_str("host=api-1 ");
    s.push('|');
    s.push_str("port=8080");
    println!("\nbuilt          {s}");

    let kept: String = s.chars().filter(|c| *c != '|').collect();
    println!("filtered       {kept}");

    // Joining is the inverse of splitting.
    let parts = ["api-1", "api-2", "api-3"];
    println!("joined         {}", parts.join(", "));

    // ---- a parser with no regex at all ------------------------------------
    println!();
    let cases = [" port = 8080 ", "# comment", "", "no-equals-here", "k="];
    for c in cases {
        println!("parse_setting({c:?}) -> {:?}", parse_setting(c));
    }
    assert_eq!(parse_setting(" port = 8080 "), Some(("port", "8080")));
    assert_eq!(parse_setting("# comment"), None);

    // ---- the same job with a regex ---------------------------------------
    println!();
    let logs = [
        "2026-01-14T09:12:33Z level=error service=api code=500",
        "2026-01-14T09:12:41Z level=warn service=web code=429",
        "2026-01-14T09:12:42Z level=error service=api code=503",
    ];
    for l in logs {
        if let Some(lvl) = level_of(l) {
            println!("level={lvl}");
        }
    }

    println!();
    for l in errors_in(&logs.join("\n"), "code=5") {
        println!("error line: {l}");
    }
}

/// Returns `None` for blank lines and comments, so the caller cannot forget to
/// skip them.
fn parse_setting(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None; // blank or comment
    }
    let (key, value) = line.split_once('=')?;
    let (key, value) = (key.trim(), value.trim());
    if key.is_empty() {
        return None;
    }
    Some((key, value))
}

/// Compiling a regex is expensive, so do it once. `OnceLock` is the
/// thread-safe "lazy static" that needs no extra crate.
fn level_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"level=(?P<lvl>\w+)").unwrap())
}

/// `caps.name("lvl")` returns a `Match<'h>` tied to the haystack, so the result
/// borrows `line` and not the temporary `Captures`. Indexing (`&caps["lvl"]`)
/// would tie it to `caps`, which dies at the end of this function.
fn level_of(line: &str) -> Option<&str> {
    level_re()
        .captures(line)
        .and_then(|caps| caps.name("lvl").map(|m| m.as_str()))
}

/// Two input references, so the elision rules cannot pick one: the return type
/// borrows what the *body* borrows from, and you have to say so.
fn errors_in<'a>(text: &'a str, needle: &str) -> Vec<&'a str> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .filter(|l| l.contains(needle))
        .collect()
}
