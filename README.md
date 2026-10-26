# Seeing Rust — the code

All the runnable code from **Seeing Rust: A Visual Guide to Ownership, Borrowing,
and Real Systems Code** (Horace Pei, 2026), split into one Cargo package per
chapter. Every package builds and runs on its own; the whole set shares a
single workspace so you can `cargo build` once and get everything.

```
git clone https://github.com/horacepei/seeing-rust-code
cd seeing-rust-code
cargo build --workspace
cargo run -p ch09-ownership
```

## What is in here

35 packages, one per chapter. Each has its own `README.md` explaining which
ideas it demonstrates.

| Part | Chapters | Packages |
|---|---|---|
| 1 Orientation | 1–3 | `ch01-hello` … `ch03-first-program` |
| 2 Language core | 4–8 | `ch04-values` … `ch08-errors` |
| 3 Ownership | 9–10 | `ch09-ownership`, `ch10-borrowing` |
| 4 Data modelling | 11–15 | `ch11-structs` … `ch15-generics` |
| 5 Real work | 16–22 | `ch16-files` … `ch22-async` |
| 6 Concurrency | 23–25 | `ch23-threads` … `ch25-shared-state` |
| 7 Production | 26–28 | `ch26-testing` … `ch28-observability` |
| 8 Projects | 29–31 | `ch29-healthcheck` … `ch31-rest-api` |
| 9 Shipping | 32–35 | `ch32-dependencies` … `ch35-reading` |

## How to read this alongside the book

The book shows fragments because a printed page has no room for the rest. Here
nothing was trimmed, so a few conventions are worth knowing:

- **Code that the compiler is *supposed* to reject** (moved values, overlapping
  borrows) is kept as a commented block with the exact `error[E….]:` message
  underneath it. Un-comment it and `cargo build` will reproduce the error. The
  runnable version right below is the fix the chapter is arguing for.
- **Shell commands** (`rustup`, `cargo tree`, `perf`) are in each package's
  `README.md` or a `NOTES.md`, not in `src/`.
- **`Cargo.toml` and config snippets** appear as real configuration where they
  matter and as quoted text where they were only illustrative.
- Packages that need the network (chapters 20, 29) start a throwaway HTTP server
  on `127.0.0.1` in a background thread, so they run offline and finish on their
  own. Nothing here talks to the public internet during `cargo test`.

## Requirements

Rust 1.82 or newer (`rust-toolchain.toml` pins `stable`). No system OpenSSL
needed — HTTP uses `rustls`.

## Licence

MIT. Use the code in your own projects; attribution is welcome but not required.

Book: *Seeing Rust* — Horace Pei — `1091186233@qq.com`
