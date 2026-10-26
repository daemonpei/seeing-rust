# Chapter 2 — Setting Up the Rust Environment

```bash
cargo run -p ch02-setup
cargo run -p ch02-setup --release
```

The install commands live in `NOTES.md` — there is nothing to compile in them.
This package prints a Cargo project back at you, so you can see what the four
entries on disk actually are.

## A Cargo project is four things

```
hello-rust/
  Cargo.toml      # metadata and dependencies; you edit this
  Cargo.lock      # exact resolved versions; cargo edits this
  src/main.rs     # the binary crate root
  .gitignore      # /target
```

`cargo new` writes all four. Everything else cargo creates later — `target/`,
`Cargo.lock` — is generated, and `target/` is the only one big enough to matter
(several GB once you have built a few crates with dependencies).

## Why `Cargo.lock` is committed for binaries

`Cargo.toml` says `serde = "1"`, which means "any 1.x". `Cargo.lock` says
`serde 1.0.210` with a checksum. Commit it for anything you ship: a rebuild in
two years resolves to the same versions, which is the difference between a
reproducible bug report and an unreproducible one. Libraries leave it out,
because their consumers' lockfiles decide.

## `env!` versus `std::env::var`

```rust
env!("CARGO_PKG_NAME")                    // compile time, becomes a literal
std::env::var("CARGO_TARGET_DIR")         // run time, returns Result
```

`env!` is resolved by the compiler and baked into the binary — it has no failure
case at runtime. That is why this program can print its own version without
opening `Cargo.toml`. `env::var` reads the process environment and can fail,
which is the shape you want when the value is allowed to be absent.

## Dev and release are different programs

`cfg!(debug_assertions)` is the flag the standard library uses to decide whether
to check integer overflow. In `dev` it is on: `u8::MAX + 1` panics. In `release`
it wraps. Run this package with and without `--release` to see the profile line
change — that one line is the reason the same source behaves differently.

`target/debug/` and `target/release/` are separate directories, so switching
profiles does not invalidate the other build.
