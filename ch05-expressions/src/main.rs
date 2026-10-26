//! Chapter 5: Operators, Expressions, and Basic Decisions

fn main() {
    // ---- a block is an expression ---------------------------------------
    let total = {
        let subtotal = 120;
        let tax = 20;
        subtotal + tax // no semicolon: the block evaluates to this
    };
    println!("{total}"); // 140

    // ---- short-circuiting is short-circuiting, not a bitwise and --------
    let table = [80u16, 443, 8080];
    println!("8080 in table: {}", valid(8080, &table));
    println!("0 in table   : {}", valid(0, &table));

    // ---- match on ranges ------------------------------------------------
    for code in [100u16, 204, 301, 404, 500, 700] {
        println!("{code} -> {}", classify(code));
    }
    let code = 204u16;
    println!("204 is one of 200|201|204: {}", matches!(code, 200 | 201 | 204));

    // ---- if let / let-else ----------------------------------------------
    let configured: Option<u16> = Some(9090);
    if let Some(port) = configured {
        println!("port {port}");
    } else {
        println!("no port configured");
    }

    early_exit(Some(8080));
    early_exit(None);
}

/// Cheap check first: the expensive one never runs when port is 0.
fn valid(port: u16, table: &[u16]) -> bool {
    port != 0 && table.contains(&port)
}

fn classify(code: u16) -> &'static str {
    match code {
        100..=199 => "informational",
        200..=299 => "success",
        300..=399 => "redirect",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown",
    }
}

/// `let-else`: early exit when the shape is wrong (Rust 1.65+).
fn early_exit(config: Option<u16>) {
    let Some(port) = config else {
        println!("let-else: nothing to do");
        return;
    };
    println!("let-else: got {port}");
}
