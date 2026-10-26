# Chapter 10 — Borrowing, References and Lifetimes

```bash
cargo run -p ch10-borrowing
```

## The rule that produces every "fight the borrow checker" story

> At any moment you may have **either** any number of shared references (`&T`)
> **or** exactly one exclusive reference (`&mut T`), never both.

Read that as a reader/writer lock that the compiler checks statically. It is the
same invariant `RwLock` enforces at run time (chapter 25) — Rust just makes the
violation a build error.

## E0502, and the two honest fixes

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // immutable borrow starts
v.push(4);           // mutable borrow — conflict
println!("{first}"); // immutable borrow used here, so it is still alive
```

The third line is what surprises people: `first` is *used after* the `push`, so
the borrow is still live and the `push` has nothing to do with it. Delete the
`println!` and the code compiles, because the borrow ended at its last use (non-
lexical lifetimes).

Two fixes, and they are not interchangeable:

- **Copy out** — `let first = v[0];` — right when the element is small and you
  only need the value.
- **Scope the borrow** — `{ let first = &v[0]; ... }` — right when the element
  is expensive and you need it as a reference.

`v.get_mut(0)` then dropping the guard is the third option; it exists, but if
the borrow checker is pointing at you, the first two are usually what you meant.

## `&[i32]` accepts more than `&Vec<i32>`

```rust
fn total(xs: &[i32]) -> i32 { xs.iter().sum() }

total(&numbers);  // &[i32; 6]  -> coerces
total(middle);    // &[i32]     -> already a slice
```

Taking `&Vec<T>` would have rejected both callers. This is the same argument as
`&str` over `&String`, and it is why the standard library's signatures look the
way they do.

## Lifetimes are not a runtime concept

`first_char` and `first_char_explicit` are the same function. The elision rules
say: one input reference, so the output borrows from it. `longest` has two
inputs, so the compiler cannot guess — you must say `'a` on both, which is the
compiler asking "does the result borrow from `a`, `b`, or both?"

Nothing is checked at run time. A lifetime is a name for "this reference is
valid for at least this long", and all of it is erased before the binary exists.
