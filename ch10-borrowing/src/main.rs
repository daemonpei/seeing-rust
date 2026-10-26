//! Chapter 10: Borrowing, References and Lifetimes

// `first_char_explicit` exists only to be compared with the elided form, so the
// "these lifetimes could be elided" lint is exactly what it is demonstrating.
#![allow(clippy::needless_lifetimes)]

fn main() {
    // ---- shared and exclusive borrows ------------------------------------
    let mut greeting = String::from("hello");
    let loud = shout(&greeting); // shared borrow
    push_exclaim(&mut greeting); // exclusive borrow, then it ends
    println!("{loud} / {greeting}"); // hello! -- the shared borrow did not
                                     // stop the &mut, because it was over

    // ---- the classic E0502 ----------------------------------------------
    // Un-commenting these three lines reproduces the chapter's error:
    //
    //     error[E0502]: cannot borrow `v` as mutable because it is also
    //                   borrowed as immutable
    //      --> src/main.rs:...
    //       |
    //     2 |     let first = &v[0];
    //       |                  - immutable borrow occurs here
    //     3 |     v.push(4);
    //       |     ^^^^^^^^^ mutable borrow occurs here
    //     4 |     println!("{first}");
    //       |               ------- immutable borrow later used here
    //
    // let mut v = vec![1, 2, 3];
    // let first = &v[0];
    // v.push(4);
    // println!("{first}");

    // ---- the fix: copy the value out, or scope the borrow ----------------
    let mut v = vec![1, 2, 3];
    let first = v[0]; // copy the i32 out; no borrow is held
    v.push(4);
    println!("{first} {:?}", v); // 1 [1, 2, 3, 4]

    let mut v = vec![1, 2, 3];
    {
        let first = &v[0];
        println!("{first}");
    } // borrow ends here
    v.push(4);
    println!("{v:?}");

    // ---- slices ----------------------------------------------------------
    let numbers = [10, 20, 30, 40, 50, 60];
    let middle = &numbers[1..4]; // shared slice of three elements
    println!("{middle:?}"); // [20, 30, 40]
    println!("{} vs {}", total(&numbers), total(middle));

    // ---- lifetimes -------------------------------------------------------
    println!("{} / {}", first_char("hello"), first_char_explicit("hello"));
    println!("{}", longest("abcd", "ab"));
}

fn shout(s: &str) -> String {
    s.to_uppercase() // reads through the reference
}

fn push_exclaim(s: &mut String) {
    s.push('!'); // writes through the reference
}

fn total(xs: &[i32]) -> i32 {
    xs.iter().sum() // accepts an array AND a Vec
}

/// These two signatures mean exactly the same thing. The elided form is what
/// you write; the explicit form is what the compiler reads.
fn first_char(s: &str) -> &str {
    &s[..1]
}

fn first_char_explicit<'a>(s: &'a str) -> &'a str {
    &s[..1]
}

/// The output could come from either input -- the compiler must ask.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}
