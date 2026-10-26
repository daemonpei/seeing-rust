//! Chapter 3: Your First Rust Program
//!
//! Three versions of the same `main`, in the order the chapter builds them up:
//! the println form, the empty skeleton, and the one that can fail.

use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- the chapter's opening listing ------------------------------------
    let audience = "systems engineers";
    println!("Hello, {audience}!");
    println!("This is line {} of {}", 2, 2);

    println!();

    // -- `main` that returns a Result -------------------------------------
    // The `?` operator is the whole reason this signature exists: an error
    // becomes a non-zero exit code and a message on stderr, for free.
    let path = Path::new("config.txt");
    if !path.exists() {
        fs::write(path, "service=inventory-api\nport=8080\n")?;
        println!("(created {} for this demo)", path.display());
    }
    let text = fs::read_to_string(path)?;
    println!("read {} bytes", text.len());

    // -- what happens when you get a name wrong ---------------------------
    // Un-commenting this reproduces the compiler error the chapter prints:
    //
    //     error[E0425]: cannot find value `audiance` in this scope
    //      --> src/main.rs:3:24
    //       |
    //     3 |     println!("Hello, {audiance}!");
    //       |                        ^^^^^^^^ help: a local variable with a
    //       |                                  similar name exists: `audience`
    //
    // println!("Hello, {audiance}!");

    Ok(())
}

/// Kept only so the `io` import is not unused — `?` on a `Result<_, io::Error>`
/// is what makes `main`'s return type work, and this spells that out.
#[allow(dead_code)]
fn read_typed(path: &str) -> io::Result<usize> {
    let text = fs::read_to_string(path)?;
    Ok(text.len())
}
