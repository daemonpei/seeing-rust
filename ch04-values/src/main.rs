//! Chapter 4: Values, Variables, and Types

const MAX_RETRIES: u32 = 5; // inlined at every use site
const DEFAULT_PORT: u16 = 8080;
static VERSION: &str = "1.0.0"; // one fixed location in memory

fn main() {
    // ---- bindings are immutable unless you say otherwise ----------------
    let service = "inventory-api"; // cannot be reassigned
    let mut retries = 3; // may be reassigned
    retries += 1;
    println!("{service} after {retries} retries");

    // ---- inference vs annotation ----------------------------------------
    let port = 8080; // inferred i32 (the default integer)
    let typed_port: u16 = 8080; // annotated: must fit in 65535
    let name = String::from("api");
    println!("{port} i32 / {typed_port} u16 / {name}");

    // turbofish: when the argument cannot tell the compiler what you mean
    let n = "42".parse::<i32>().unwrap();
    println!("parsed {n}");

    // ---- conversions ----------------------------------------------------
    let count: u32 = 7;
    let as_float = count as f64 / 2.0; // 3.5
    let wide: u64 = u64::from(count); // widening: infallible, lossless
    let small: u8 = u8::try_from(300).unwrap_or(0); // narrowing: can fail
    println!("{as_float} / {wide} / try_from(300) -> {small}");

    // ---- shadowing beats mutation for transforming a value ---------------
    let input = "  42  ";
    let input = input.trim(); // &str -> &str, new binding
    let input: i32 = input.parse().unwrap(); // now it is an i32
    println!("{input}");

    // ---- consts and statics ---------------------------------------------
    println!("{MAX_RETRIES} {DEFAULT_PORT} {VERSION}");

    // The integer default is i32, so this is the one line that ties the
    // chapter together: an untyped literal takes the type its *use* demands.
    let mut total = 0u64;
    for _ in 0..MAX_RETRIES {
        total += DEFAULT_PORT as u64;
    }
    println!("{total} across {MAX_RETRIES} retries");
}
