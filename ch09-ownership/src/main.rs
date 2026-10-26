//! Chapter 9: Ownership: How Rust Manages Memory
//!
//! The chapter's first listing is *supposed* to fail to compile. It is kept
//! below as a comment block, with the error rustc actually prints, and the
//! runnable version follows it.

fn main() {
    // ---- move, clone, copy ----------------------------------------------
    let a = String::from("hello");
    let b = a; // MOVE: ownership transfers to b
    // println!("{a}"); // error[E0382]: borrow of moved value: `a`
    //                          |
    //                          = note: move occurs because `a` has type
    //                            `String`, which does not implement the `Copy` trait

    let c = b.clone(); // CLONE: deep copy, two independent buffers
    println!("{b} {c}"); // both are fine now

    let x: i32 = 5;
    let y = x; // COPY: still usable, x was never moved
    println!("{x} {y}"); // 5 5

    // ---- scopes decide when memory is freed ------------------------------
    scope_demo();

    // ---- ownership can leave a function and come back --------------------
    let text = give(); // ownership moves out to the caller
    let n = take(text); // ...and in again, where it is dropped
    // println!("{text}");  // error: text was moved into take()
    println!("{n}");
}

fn scope_demo() {
    let outer = String::from("outer");
    {
        let inner = String::from("inner");
        println!("{inner}");
    } // <- inner is dropped here
    println!("{outer}");
} // <- outer is dropped here

fn take(s: String) -> usize {
    s.len()
} // s is dropped here -- the caller no longer owns it

fn give() -> String {
    String::from("fresh") // ownership moves out to the caller
}
