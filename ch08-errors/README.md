# Chapter 8 — Errors and Explicit Failure Handling

```bash
cargo run -p ch08-errors
```

Two types, two jobs, and the chapter keeps them apart on purpose:

| Type | Answers | You handle it with |
|---|---|---|
| `Option<T>` | "is there a value?" | `unwrap_or`, `map`, `and_then`, `?` |
| `Result<T, E>` | "why did it fail?" | `?`, `map_err`, `match` |

Using `Result` where `Option` belongs produces meaningless errors; using
`Option` where `Result` belongs throws away the reason. A missing config file is
a `Result` (permission denied and not-found are different), while "no port
configured" is an `Option`.

## The three-listing progression

1. **`Box<dyn std::error::Error>`** — fastest to write. Any error type converts
   into it via `?`. Fine for a binary's `main`, useless for a caller that needs
   to distinguish "not found" from "bad syntax".
2. **Your own `enum`** — `ConfigError` gives every failure a name. Costs about
   fifteen lines of `Display` and `Error` boilerplate, and buys exhaustive
   matching at every call site.
3. **`Box<dyn Error>` plus context** — the pragmatic middle. `anyhow` packages
   this as `.context("read config")`, shown commented at the end of the
   chapter's third listing.

## `map_err` is not optional in practice

```
Error: No such file or directory (os error 2)
```

is what you get without it. With it:

```
Error: read config /etc/inventory/config.json: No such file or directory (os error 2)
```

One line of code, and the support ticket resolves itself. Every `?` on a
filesystem or network call should have one.
