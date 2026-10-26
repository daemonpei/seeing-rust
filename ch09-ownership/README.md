# Chapter 9 — Ownership: How Rust Manages Memory

```bash
cargo run -p ch09-ownership
```

## The rule, in one sentence

Every value has exactly one owner; when the owner goes out of scope the value is
freed. That is the whole of Rust's memory management — no collector, no manual
`free`, no reference counting unless you ask for it.

## Three things you can do with a value

```
let b = a;        MOVE    — a String's buffer changes hands; a is unusable
let c = b.clone(); CLONE  — a second buffer, allocated and copied
let y = x;        COPY    — i32 is Copy; both names stay valid
```

`Copy` is a trait. `i32`, `bool`, `char`, `f64`, `&T` and tuples of `Copy`
types implement it. `String`, `Vec<T>`, `Box<T>` and anything that owns heap
memory do not — moving them is a pointer transfer, so a silent duplication would
be a leak or a double free.

## Why `println!("{a}")` is an error and not undefined behaviour

```
error[E0382]: borrow of moved value: `a`
```

In C++ `std::move(a)` leaves `a` in a valid-but-unspecified state; reading it
compiles. In Rust the move is tracked by the type system, so the compiler knows
`a` is dead and refuses. There is no "valid but unspecified" state to reason
about, and no use-after-free to debug.

## Drop order

```rust
{
    let inner = String::from("inner");
}                     // inner freed here
```

Variables drop in reverse declaration order, at the end of their scope. Unlike
C, this happens on every exit path — early `return`, `?`, and `panic!` unwinding
all run the same destructors.

## `take` and `give`

A function that takes `String` by value consumes the caller's binding. That is
why `take(text)` makes `text` unusable afterwards. Chapter 10 shows the fix when
you want to keep using the value: pass `&text` instead.
