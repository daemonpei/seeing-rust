# Chapter 32 — Dependency Management and Third-Party Crates

```bash
cargo run -p ch32-dependencies
cargo run -p ch32-dependencies --no-default-features
cargo run -p ch32-dependencies --features pattern
cargo tree -p ch32-dependencies
cargo tree -p ch32-dependencies --all-features --edges features
```

The first run has **zero** dependencies. The third adds `regex` and everything
behind it. That is the feature system doing the only thing it is really for:
letting a downstream crate pay for what it uses.

## `optional = true` plus `dep:`

```toml
[features]
pattern = ["dep:regex"]

[dependencies]
regex = { version = "1", optional = true }
```

`dep:regex` says "this feature enables the *dependency* regex, without also
creating a feature called `regex`". Without the `dep:` prefix you get two names
for one thing, and a downstream crate can enable `regex` directly, bypassing
whatever you intended.

## `cfg!(feature = "...")` versus `#[cfg(feature = "...")]`

```rust
if cfg!(feature = "fast-path") { ... }        // both branches must compile
#[cfg(feature = "pattern")] fn f() { ... }    // the item may not exist
```

Use `cfg!` when both paths are always valid. Use `#[cfg]` when the code
*references* something that only exists with the feature — the regex import
above would be an unresolved-crate error without it.

## The four commands worth memorising

| Command | Answers |
|---|---|
| `cargo tree --depth 1` | what do I actually depend on? |
| `cargo tree -d` | why do I have two copies of `rand`? |
| `cargo tree -i rand` | who pulled this in? |
| `cargo bloat --crates` | which crate is 6 MB of my binary? |

Two copies of a crate is normal (different major versions) and expensive (both
are compiled, both are linked). `-d` names them, `-i` names the culprit, and the
fix is usually bumping one dependent.

## The rule that keeps builds honest

> A feature must only ever add capability.

If `--no-default-features` changes behaviour rather than removing it, two
dependents that disagree get a build nobody tested. This is why the ecosystem
treats "feature = behaviour change" as a semver violation.
