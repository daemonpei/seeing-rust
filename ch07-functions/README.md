# Chapter 7 — Functions and Program Decomposition

```bash
cargo run -p ch07-functions
```

## Return type is an ownership decision

Chapter 7's shortest useful table:

| Signature | What the caller gets |
|---|---|
| `fn length_of(s: &str) -> usize` | borrows, returns a number — nothing is moved |
| `fn shout(s: &str) -> String` | borrows, returns **new owned** text |
| `fn consume(s: String) -> usize` | takes ownership; the caller can no longer use `s` |

Rust forces you to pick one. In C++ the equivalent choice is invisible: a
`const std::string&` parameter and a `std::string` parameter compile to code
that behaves differently on overflow, aliasing and lifetime, and neither
signature tells you.

## Tuples are not a god object

`split_host_port` returns `(&str, u16)` because those two values genuinely
travel together and mean nothing apart. Once you have three or more, name it:

```rust
struct Endpoint<'a> { host: &'a str, port: u16 }
```

The compiler will not stop you from returning a `(String, u16, bool, bool)`;
your reviewer will.

## Closures capture by need

`|n| n * multiplier` borrows `multiplier`. `|| calls += 1` needs `&mut`, so it
takes a mutable borrow. Add `move` and the closure takes ownership instead —
which is what `thread::spawn` requires (chapter 23).

## The decomposition listing

The chapter contrasts one ninety-line `process_everything()` with three named
functions that each return `Result`. The listing is a signature sketch, not
code; `summarise` here is the same shape made runnable.
