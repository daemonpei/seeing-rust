//! Chapter 7: Functions and Program Decomposition

fn main() {
    // a predicate that reads like the question it answers
    println!("204 success: {}", is_success(204));
    println!("404 success: {}", is_success(404));

    // a tuple, when the pieces are genuinely a pair
    let (host, port) = split_host_port("localhost:8080");
    println!("{host} / {port}");
    let (host, port) = split_host_port("localhost");
    println!("{host} / {port}");

    // three signatures, three ownership decisions
    println!("{}", length_of("abc")); // borrows, returns a number
    println!("{}", shout("hello")); // borrows, returns owned text
    println!("{}", consume(String::from("owned"))); // takes ownership

    // closures capture what they use
    let multiplier = 3;
    let triple = |n: i32| n * multiplier; // captures multiplier by reference
    println!("{}", triple(5)); // 15

    let mut calls = 0;
    {
        let mut count = || calls += 1; // captures by mutable reference
        count();
        count();
    }
    println!("{calls}"); // 2

    // The chapter's "one big function vs five small ones" listing is a
    // signature sketch rather than code:
    //
    //   fn load_targets(path: &Path) -> Result<Vec<String>, Error>;
    //   fn check_target(client: &Client, url: &str) -> Result<u16, Error>;
    //   fn print_summary(total: usize, failed: usize);
    //
    // `summarise` below is that shape reduced to something runnable.
    println!("{}", summarise(&[Some(200), Some(503), None]));
}

fn is_success(code: u16) -> bool {
    (200..300).contains(&code)
}

fn split_host_port(addr: &str) -> (&str, u16) {
    match addr.split_once(':') {
        Some((h, p)) => (h, p.parse().unwrap_or(0)),
        None => (addr, 0),
    }
}

fn length_of(s: &str) -> usize {
    s.len()
}

fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn consume(s: String) -> usize {
    s.len()
}

/// `None` stands for "the check itself failed", which is different from a
/// non-2xx status. Keeping that distinction in the type is the chapter's point.
fn summarise(results: &[Option<u16>]) -> String {
    let total = results.len();
    let failed = results.iter().filter(|r| !matches!(r, Some(c) if (200..300).contains(c))).count();
    format!("{}/{total} healthy", total - failed)
}
