# Chapter 4 — Values, Variables, and Types

```bash
cargo run -p ch04-values
```

The chapter's five listings, in order, all run from this one `main`:

| Listing | Idea |
|---|---|
| `let` vs `let mut` | immutability is the default; `mut` is a request you have to make |
| inference vs annotation | `8080` is an `i32` until a `u16` context says otherwise |
| `as` vs `From` vs `TryFrom` | widening is free, narrowing returns a `Result` |
| shadowing | rebinding with a new type, instead of a mutable variable |
| `const` / `static` | inlined at every use site vs one fixed address |

## Why `try_from` and not `as`

`300 as u8` is `44`. It compiles, it is silent, and it is the kind of bug that
survives code review. `u8::try_from(300)` returns `Err` and makes you decide
what to do. Reach for `as` only when the ranges genuinely line up — a `u32`
count that you know is under 2³², converted to `u64`.

## Shadowing

```rust
let input = "  42  ";
let input = input.trim();
let input: i32 = input.parse().unwrap();
```

Three bindings called `input`, three different types (`&str`, `&str`, `i32`),
one logical value. The alternative — `let mut input` — cannot change the type,
so you would end up with `input_str`, `input_trimmed`, `input_parsed`. Shadowing
is the idiomatic answer.
