# Chapter 6 — Loops and Repeated Work

```bash
cargo run -p ch06-loops
```

## The three iteration forms

```rust
for host in hosts               // by value: consumes the array (Copy here)
for i in 0..hosts.len()         // by index: half-open range
for (i, host) in hosts.iter().enumerate()   // index + reference together
```

`0..n` excludes `n`. That is not a quirk — it is the reason `0..v.len()` is
always a valid index range, while `0..=v.len()` panics on the last step.

## `loop` as an expression

`first_available` returns `Option<u16>` without a `return` statement. `break
Some(port)` makes the loop itself evaluate to `Some(port)`; the `?` on
`ports.get(i)` returns `None` from the function when the slice runs out. Two
exit paths, no mutable flag.

## Labelled loops

Rust has no `goto`, but it does have labels. `break 'search` leaves both loops;
`continue 'search` restarts the outer one. Without the label you would need a
`found` boolean checked in the outer condition — which is exactly the pattern
labels exist to delete.

## Loop or iterator?

Both versions at the bottom of `main` produce the same `Vec`. The loop is
clearer when the body does several things; the pipeline is clearer when it does
one. The one real difference is type: the loop produces `Vec<u16>`, the
pipeline produces `Vec<&u16>` because it never copies. `.copied()` closes that
gap when you want owned values back.
