# Chapter 5 — Operators, Expressions, and Basic Decisions

```bash
cargo run -p ch05-expressions
```

Four things the chapter argues for, all visible in the output:

**Blocks evaluate to their last expression.** The missing semicolon on
`subtotal + tax` is the feature, not a typo. A `;` turns the block into `()`,
which is why adding one there is such a common first Rust mistake.

**`&&` short-circuits, `&` does not.** In `port != 0 && table.contains(&port)`,
the lookup is skipped when `port` is 0. Writing `&` instead would be a type
error (`bool & bool` needs two `bool`s, not a `u16`), which is the compiler
saving you from a C-ism.

**`match` on ranges is exhaustive by default.** `100..=199` covers the range
inclusively; without the `_` arm rustc rejects the match because it cannot prove
every `u16` is covered. That `_` is the only place in this function where you
can forget a case — and the compiler tells you.

**`let-else` beats nesting.** Compare:

```rust
// before
let port = match config {
    Some(p) => p,
    None => return,
};
// after
let Some(port) = config else { return; };
```

The `else` branch must diverge (`return`, `break`, `panic!`), which is exactly
what keeps the happy path unindented.
