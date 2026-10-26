# Seeing Rust — the code

**English** | [中文说明](README.zh-CN.md)

Every runnable program from **_Seeing Rust: A Visual Guide to Ownership, Borrowing,
and Real Systems Code_** (Horace Pei, 2026), split into one Cargo package per
chapter. Each package builds and runs on its own; all 35 share one workspace, so
a single `cargo build` covers the whole book.

Clone it, run chapter 1, and you are using it:

```bash
git clone https://github.com/daemonpei/seeing-rust
cd seeing-rust
cargo run -p ch01-hello
```

```
Rust is ready for systems work.
```

---

## Contents

- [Quick start](#quick-start)
- [Requirements](#requirements)
- [What is in here](#what-is-in-here)
- [Running a chapter](#running-a-chapter)
- [Chapter index](#chapter-index)
- [Chapters that need something extra](#chapters-that-need-something-extra)
- [Reading this alongside the book](#reading-this-alongside-the-book)
- [Troubleshooting](#troubleshooting)
- [Licence](#licence)

---

## Quick start

```bash
# 1. one package — compiles only that chapter and its dependencies
cargo run -p ch09-ownership

# 2. everything — 35 packages, a few minutes the first time
cargo build --workspace

# 3. every test in the book
cargo test --workspace
```

You do **not** need to build the whole workspace to follow the book. Start with
step 1 and cargo will compile only what that chapter needs. The full build is for
checking that nothing is broken, not for reading chapter 4.

---

## Requirements

| | |
|---|---|
| Rust | **1.82 or newer**. `rust-toolchain.toml` pins `stable` and pulls in `rustfmt` + `clippy`; `rustup` applies it automatically on first use. |
| Network | Only for the first build, to download crates. **Every program then runs offline** — chapters that need HTTP start a throwaway server on `127.0.0.1`. |
| OpenSSL | Not needed. All HTTP uses `rustls`, so there is nothing to install on Windows. |
| Disk | The shared `target/` directory reaches several GB once everything is built. Source is under 1 MB; `target/` is git-ignored and never committed. |
| OS | Linux, macOS, Windows. Path handling in chapter 16 is written to work on all three. |

---

## What is in here

```
seeing-rust/
├── Cargo.toml                 ← the workspace: lists all 35 members
├── Cargo.lock                 ← pinned versions, committed on purpose
├── rust-toolchain.toml        ← stable + rustfmt + clippy
├── README.md                  ← this file
├── README.zh-CN.md            ← 中文说明
├── ch01-hello/                ← one package per chapter
│   ├── Cargo.toml
│   ├── README.md              ← what this chapter demonstrates
│   └── src/
├── ch02-setup/
│   ├── NOTES.md               ← shell commands that are not Rust code
│   └── src/
├── …
└── ch35-reading/
```

Two conventions worth knowing before you start:

- **The root `Cargo.toml` is a workspace, not a program.** It has no `src/`. It
  exists so all 35 packages share one `target/` and one `Cargo.lock`, which is
  why `cargo build --workspace` works at all.
- **`Cargo.lock` is committed.** These are binaries, not libraries, so the exact
  resolved versions are part of the deal: you get the same `serde`, `tokio` and
  `axum` the book was written against. (Chapter 2 explains why libraries do the
  opposite.)

---

## Running a chapter

Every chapter is a binary, so every chapter has the same one-line command:

```bash
cargo run -p <package-name>
```

The `-p` flag names the package; the name is the directory name (`ch09-ownership`,
not `ch09`, not `ownership`).

### The `--` matters

Arguments **before** `--` go to cargo, arguments **after** `--` go to your program:

```bash
cargo run -p ch19-cli -- --needle 503 -n     # --needle goes to the CLI tool
cargo run -p ch19-cli --help                 # --help goes to cargo, not the tool
```

If a flag you passed to your program is rejected with "unexpected argument", you
forgot the `--`.

### Other useful forms

```bash
cargo run  -p ch27-performance --release -- --bench   # optimised build
cargo test -p ch26-testing                            # a chapter's tests
cargo test -p ch26-testing classify                   # one test by name
cargo test --workspace                                # all tests
cargo build --workspace --release                     # optimised, slowest
cargo clean                                           # delete target/ (several GB)
```

---

## Chapter index

The default command is `cargo run -p <package>`. The last column lists only what
is different from that.

| Ch | Package | Title | Extra |
|---|---|---|---|
| 1 | `ch01-hello` | What Rust Is and Why Systems Engineers Use It | |
| 2 | `ch02-setup` | Setting Up the Rust Environment | also `--release`; commands in `NOTES.md` |
| 3 | `ch03-first-program` | Your First Rust Program | |
| 4 | `ch04-values` | Values, Variables, and Types | |
| 5 | `ch05-expressions` | Operators, Expressions, and Basic Decisions | |
| 6 | `ch06-loops` | Loops and Repeated Work | |
| 7 | `ch07-functions` | Functions and Program Decomposition | |
| 8 | `ch08-errors` | Errors and Explicit Failure Handling | |
| 9 | `ch09-ownership` | Ownership: How Rust Manages Memory | |
| 10 | `ch10-borrowing` | Borrowing, References and Lifetimes | |
| 11 | `ch11-structs` | Structs, Enums and Domain Models | |
| 12 | `ch12-collections` | Vectors, Strings and Hash Maps | |
| 13 | `ch13-modules` | Crates, Modules and Project Organisation | |
| 14 | `ch14-traits` | Traits and Composition | |
| 15 | `ch15-generics` | Generics for Reusable Type-Safe Code | |
| 16 | `ch16-files` | File I/O and Paths | writes files next to where you run it |
| 17 | `ch17-text` | Text Processing and Regular Expressions | |
| 18 | `ch18-json` | JSON and Structured Data | |
| 19 | `ch19-cli` | Command-Line Programs | **takes arguments** (below) |
| 20 | `ch20-http-client` | HTTP Clients and API Consumption | starts a local server on `127.0.0.1` |
| 21 | `ch21-http-server` | HTTP Servers and REST-Style APIs | **long-running** (below) |
| 22 | `ch22-async` | Async Rust, Futures and Tokio | |
| 23 | `ch23-threads` | Threads and Concurrent Work | |
| 24 | `ch24-channels` | Channels and Message Passing | |
| 25 | `ch25-shared-state` | Shared State: Arc, Mutex and Atomics | see `NOTES.md` |
| 26 | `ch26-testing` | Testing With Cargo | **run with `cargo test`** |
| 27 | `ch27-performance` | Benchmarking, Profiling and Performance | **use `--release`** |
| 28 | `ch28-observability` | Logging, Configuration and Observability | `RUST_LOG=…` (below) |
| 29 | `ch29-healthcheck` | Project 1: A Health Check CLI Tool | **takes arguments** (below) |
| 30 | `ch30-config-validator` | Project 2: A JSON Configuration Validator | **takes a file** (below) |
| 31 | `ch31-rest-api` | Project 3: A Small REST API Server | **long-running** (below) |
| 32 | `ch32-dependencies` | Dependency Management and Third-Party Crates | features + `cargo tree` |
| 33 | `ch33-safety` | Safety, Unsafe Rust and Security Basics | |
| 34 | `ch34-release` | Building, Releasing and Running Rust Programs | also `--release`; see `NOTES.md` |
| 35 | `ch35-reading` | Reading Rust Code and Continuing Your Learning | **takes a file** (below) |

Grouped by part: 1–3 orientation · 4–8 language core · 9–10 ownership ·
11–15 data modelling · 16–22 real work · 23–25 concurrency · 26–28 production ·
29–31 projects · 32–35 shipping.

---

## Chapters that need something extra

### Servers you have to leave running (ch21, ch31)

These two do not exit — they listen until you stop them with `Ctrl-C`. Run them
in one terminal and talk to them from another:

```bash
# terminal 1
cargo run -p ch21-http-server

# terminal 2
curl localhost:8080/health
curl localhost:8080/items
curl -X POST localhost:8080/items -H 'content-type: application/json' -d '{"name":"widget"}'
curl localhost:8080/items/item-0001
```

Both chapters bind **port 8080**, so do not run them at the same time. If you
would rather not open a second terminal, each has tests that need no socket:

```bash
cargo test -p ch21-http-server
cargo test -p ch31-rest-api
```

On Windows PowerShell, `curl` is an alias for `Invoke-WebRequest` and takes
different flags — use `curl.exe` to get the real thing.

### Chapters that take arguments

```bash
cargo run -p ch19-cli                          # processes a generated sample
cargo run -p ch19-cli -- --needle 503 -n       # different needle, line numbers
cargo run -p ch19-cli -- Cargo.toml            # works on any text file
cargo run -p ch19-cli -- --help                # the tool's own help

cargo run -p ch29-healthcheck -- --selftest            # offline, 2 good + 2 bad
cargo run -p ch29-healthcheck -- --selftest --json
printf 'http://127.0.0.1:9/\n' | cargo run -p ch29-healthcheck -- - --workers 1

cargo run -p ch30-config-validator -- examples/good.json ; echo "exit=$?"
cargo run -p ch30-config-validator -- examples/bad.json  ; echo "exit=$?"
cargo run -p ch30-config-validator -- examples/bad.json --json

cargo run -p ch35-reading                      # outlines its own source
cargo run -p ch35-reading -- --self
cargo run -p ch35-reading -- path/to/file.rs
cargo run -p ch35-reading -- --docs
```

`--selftest` starts its own local server, so chapter 29 demonstrates a real HTTP
health check without touching the internet.

### Chapters where the build profile is the point (ch02, ch27, ch34)

Debug and release are genuinely different programs — integer overflow panics in
one and wraps in the other. Run these both ways and compare:

```bash
cargo run -p ch02-setup
cargo run -p ch02-setup --release

cargo run -p ch27-performance -- --bench
cargo run -p ch27-performance --release -- --bench    # expect 10–50x

cargo run -p ch34-release
cargo run -p ch34-release --release
```

Never trust a timing from a debug build.

### Logging (ch28, ch31)

```bash
RUST_LOG=debug cargo run -p ch28-observability
RUST_LOG=ch28_observability=trace cargo run -p ch28-observability
RUST_LOG=debug cargo run -p ch31-rest-api
```

On Windows, set it first: `$env:RUST_LOG="debug"` then `cargo run …`.

### Features and the dependency tree (ch32)

```bash
cargo run -p ch32-dependencies
cargo run -p ch32-dependencies --no-default-features
cargo run -p ch32-dependencies --features pattern
cargo tree -p ch32-dependencies
cargo tree -p ch32-dependencies --all-features --edges features
```

---

## Reading this alongside the book

The book prints fragments because a page has no room for the rest. Nothing is
trimmed here, so a few conventions are worth knowing:

- **Code the compiler is *supposed* to reject** (moved values, overlapping
  borrows) is kept as a commented block with the exact `error[E…]:` message
  underneath it. Un-comment it and `cargo build` reproduces the error; the
  runnable version right below is the fix the chapter argues for. **A chapter
  that fails to compile is usually you, not the repo** — check whether you
  un-commented a demonstration block.
- **Shell commands** (`rustup`, `cargo tree`, `perf`) live in a package's
  `README.md` or `NOTES.md`, not in `src/`.
- **`Cargo.toml` and config snippets** appear as real configuration where they
  matter and as quoted text where they were only illustrative.
- **Network chapters** start a throwaway server on `127.0.0.1` in a background
  thread, so they run offline and exit on their own. Nothing here talks to the
  public internet during `cargo test`.
- **Each package has its own `README.md`** explaining what it demonstrates. That
  is the commentary; `src/` is the code the book is quoting.

---

## Troubleshooting

| Symptom | Cause and fix |
|---|---|
| `error: no package named ch09` | The flag takes the full directory name: `cargo run -p ch09-ownership`. |
| `unexpected argument '--needle'` | Missing `--` between cargo's args and your program's. |
| `Address already in use (os error 48/98/10048)` | Port 8080 is taken — usually by chapter 21 or 31 still running. `Ctrl-C` it. |
| `curl` prints PowerShell help | Windows aliases `curl` to `Invoke-WebRequest`. Use `curl.exe`. |
| Compile error you did not expect | You un-commented a "this should fail" block. See the note above. |
| Build takes forever / disk is full | `cargo clean` frees the whole `target/` (several GB). The next build recompiles everything. |
| `cargo` not found after install | Restart the shell, or `source "$HOME/.cargo/env"`. |
| Offline machine, first build | Build once where there is a network, then set `CARGO_NET_OFFLINE=true`. |
| `rustup` wants to download a toolchain | Expected on first run; `rust-toolchain.toml` pins `stable`. |

---

## Licence

MIT. Use the code in your own projects; attribution is welcome but not required.

Book: *Seeing Rust: A Visual Guide to Ownership, Borrowing, and Real Systems Code*
— **Horace Pei**, 2026 — `1091186233@qq.com`
