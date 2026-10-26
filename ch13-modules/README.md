# Chapter 13 — Crates, Modules and Project Organisation

```bash
cargo run -p ch13-modules
```

## The three path prefixes

| Prefix | Meaning |
|---|---|
| `crate::` | this crate's root |
| `super::` | the parent module |
| `self::` | this module |

There is no `../`. Modules form a tree and paths walk it, which is why moving a
file only changes its own `use` lines and the parent's `mod` line.

## `pub` is not transitive

```rust
pub struct Config {
    pub host: String,   // reachable from outside
    port: u16,          // private: only this module may touch it
}
```

`Config` being `pub` makes the *type* visible. Each field needs its own `pub`.
That is what lets `Config` guarantee "port is never 0": no outside code can
assign it, only `Config::new` can, and `new` checks.

## Private by default, and that is a feature

`fn default_port()` in `config.rs` has no `pub`, so no other module can call it.
Deleting or changing it cannot break anything outside the file — the compiler
will tell you if it does. In a language where everything is public by default,
a rename is a search across the whole repo plus every downstream crate.

## File layout

```
src/lib.rs              pub mod config; pub mod checker;
src/config.rs           the module itself
src/checker/mod.rs      a module with children
src/checker/report.rs   a child
src/main.rs             the binary, which consumes the library
```

`main.rs` and `lib.rs` can coexist. `main.rs` becomes thin wiring; everything
worth testing lives in the library, because integration tests in `tests/` can
only reach the library's public API (chapter 26).
