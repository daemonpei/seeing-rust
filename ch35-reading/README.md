# Chapter 35 — Reading Rust Code and Continuing Your Learning

```bash
cargo run -p ch35-reading                      # outlines its own source
cargo run -p ch35-reading -- --self
cargo run -p ch35-reading -- path/to/file.rs
cargo run -p ch35-reading -- --docs
```

## Read the outline before the body

Any unfamiliar crate, in this order:

1. **`Cargo.toml`** — what does it depend on, and with which features? A crate
   with `tokio` and no `serde` is doing I/O; the reverse is doing data.
2. **`lib.rs`** — the `pub mod` and `pub use` lines are the crate's actual API.
   Everything else is implementation, which is a promise the compiler enforces.
3. **The trait definitions** — they name the seams. A crate with one `Store`
   trait and three implementations has a very different shape from one with ten
   `impl` blocks on a concrete struct.
4. **`impl` blocks, in source order** — this is where behaviour lives.
5. **The tests** — they are executable documentation of intent, and unlike
   comments they cannot be wrong about what the code does.

`cargo run -- path/to/file.rs` does step 1 for a single file and prints the item
list with line numbers, which is the map you read before the territory.

## `impl Trait` and `where` clauses are the real signature

```rust
fn process<T>(items: Vec<T>) -> Result<usize, Error>
where
    T: Serialize + Send + 'static,
```

The bounds are not boilerplate. `Send + 'static` says "this crosses a thread
boundary" and `Serialize` says "whose output leaves the process". Reading the
bounds tells you the architecture faster than reading the body.

## The four habits that make Rust code readable

| Habit | What it buys |
|---|---|
| short functions, one `Result` each | the signature is the documentation |
| newtypes over `String`/`u16` | the reader stops tracking invariants manually |
| `enum` over `bool` flags | no `process(true, false)` call sites |
| `?` instead of `match` | error handling stops hiding the happy path |

None of them are about being clever. They are about leaving the reader less to
hold in their head — which is what chapter 1 promised and chapter 35 delivers on.

## Where to go next

- *The Rust Programming Language* — free, authoritative, and the chapters on
  lifetimes and closures are the ones worth rereading.
- *Rust for Rustaceans* — the book for after this one; it assumes you can already
  write Rust and explains why the idioms are what they are.
- The standard library docs, read as prose — `Option`, `Iterator` and `Result`
  have more methods than you think, and every one of them is a bug you do not
  have to write.
- `cargo doc --open` on your own crate, once it has `//!` comments. If the docs
  read badly, the API probably does too.
