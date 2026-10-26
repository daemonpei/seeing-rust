# Chapter 19 — Command-Line Programs

```bash
cargo run -p ch19-cli                      # processes a generated sample
cargo run -p ch19-cli -- --needle 503 -n   # line numbers, different needle
cargo run -p ch19-cli -- Cargo.toml        # works on any text file
cat app.log | cargo run -p ch19-cli -- -   # stdin
cargo run -p ch19-cli -- --help
```

Note the `--`: everything before it goes to `cargo`, everything after it goes to
your binary.

## `std::env::args` versus `clap`

The chapter shows both. `env::args()` is fine for a throwaway script:

```rust
let args: Vec<String> = env::args().skip(1).collect();
```

It stops being fine the moment you need `--help`, a default, a short flag, an
error message that names the missing argument, and `--version`. `clap` derives
all of that from the struct:

```rust
#[arg(short, long, default_value = "error")]
needle: String,
```

— and gives you `-n`, `--needle`, `--needle=500`, validation, and a usage line
that matches the docs comment above each field.

## Exit codes are part of the interface

| Code | Meaning here |
|---|---|
| `0` | ran, results printed |
| `2` | could not run (bad file, bad args) |

`clap` uses `2` for argument errors by convention, so keeping `2` for "could not
run" makes the tool behave predictably in a shell pipeline. Exit `1` is left free
for "ran fine, but the answer was bad" — which is what chapter 29's health check
uses to signal an unhealthy target.

## Composable means: read stdin

A tool that only takes a filename cannot participate in a pipeline. `-` as the
filename is the near-universal convention (see `gzip`, `jq`, `tar`), and the
implementation is three lines:

```rust
for line in io::stdin().lock().lines() { ... }
```

`lock()` matters: without it every `lines()` call takes and releases the mutex.
