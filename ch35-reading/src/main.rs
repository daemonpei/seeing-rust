//! Chapter 35: Reading Rust Code and Continuing Your Learning
//!
//! The chapter is about *reading* Rust. This is a small tool for doing it: point
//! it at a .rs file and it prints the item outline, which is the skeleton you
//! want before reading any unfamiliar crate.

use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("outline <file.rs> | --self | --docs");
        return Ok(());
    }
    if args.iter().any(|a| a == "--docs") {
        print_docs();
        return Ok(());
    }
    if args.iter().any(|a| a == "--self") {
        // Reading this file with this file's own parser: the honest demo.
        return outline_file(Path::new(file!()));
    }

    match args.first() {
        Some(p) => outline_file(Path::new(p)),
        None => {
            println!("no file given; outlining myself\n");
            outline_file(Path::new(file!()))
        }
    }
}

fn outline_file(path: &Path) -> std::io::Result<()> {
    let text = std::fs::read_to_string(path)?;
    println!("{} ({} lines)\n", path.display(), text.lines().count());

    let mut depth = 0i32;
    for (n, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();

        // Track module nesting so the outline shows structure, not a flat list.
        if trimmed.starts_with("mod ") && trimmed.ends_with('{') {
            println!("{}{}", "  ".repeat(depth as usize), trimmed);
            depth += 1;
            continue;
        }
        if trimmed == "}" && depth > 0 {
            depth -= 1;
            continue;
        }

        if let Some(item) = classify(trimmed) {
            println!("{:>4}  {}{}", n + 1, "  ".repeat(depth as usize), item);
        }
    }
    Ok(())
}

/// The visibility + keyword pairs that make up a Rust item signature.
fn classify(line: &str) -> Option<String> {
    let keywords = [
        "pub fn ", "pub async fn ", "pub const fn ", "fn ", "async fn ", "pub trait ",
        "trait ", "pub struct ", "struct ", "pub enum ", "enum ", "pub mod ", "mod ",
        "impl ", "pub type ", "type ", "pub const ", "const ", "unsafe fn ",
    ];

    // Skip attributes, comments and anything inside a fn body.
    if line.starts_with("//") || line.starts_with("#[") || line.starts_with("///") {
        return None;
    }

    keywords.iter().find_map(|kw| {
        line.strip_prefix(kw).map(|rest| {
            // Keep the signature up to the opening brace or the line end.
            let sig = rest.split('{').next().unwrap_or(rest).trim_end();
            format!("{kw}{sig}")
        })
    })
}

fn print_docs() {
    println!("Read the docs that are already on your disk:");
    println!();
    println!("  cargo doc --open           # your dependencies, locally");
    println!("  rustup doc std             # the standard library, offline");
    println!("  rustup doc --book          # The Rust Programming Language");
    println!("  cargo doc --no-deps --open # only your crate");
    println!();
    println!("Read the source of a function you use -- it is on your disk:");
    if let Ok(out) = std::process::Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
    {
        let root = String::from_utf8_lossy(&out.stdout).trim().to_string();
        println!("  {root}/lib/rustlib/src/rust/library/core/src/");
    } else {
        println!("  $(rustc --print sysroot)/lib/rustlib/src/rust/library/core/src/");
    }
    println!("  (install it once: rustup component add rust-src)");
}
