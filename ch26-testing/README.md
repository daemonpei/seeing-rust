# Chapter 26 — Testing With Cargo

```bash
cargo test -p ch26-testing
cargo test -p ch26-testing -- --nocapture      # see println! output
cargo test -p ch26-testing classify            # filter by test name
cargo run  -p ch26-testing -- Cargo.toml
```

## Three kinds of test, three directories

| Where | Sees | Use for |
|---|---|---|
| `#[cfg(test)] mod tests` in `src/lib.rs` | private items | unit tests |
| `tests/*.rs` | public API only | integration, the caller's view |
| `#[doc]` examples | public API | documentation that is compiled |

Integration tests are separate crates. That is why `tests/integration.rs` needs
`use ch26_testing::…` and cannot call a private helper — and it is the reason
"the API is awkward" shows up as a compile error in your own test file rather
than as a bug report six months later.

`#[cfg(test)]` means the module is not compiled into the release binary at all.
The `tests/` directory is never compiled into your crate either.

## `#[should_panic]` should name the message

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn indexing_past_the_end_panics() {
    let _ = vec![1, 2, 3][9];
}
```

Without `expected`, a panic for the wrong reason — an `unwrap` on a `None`
somewhere upstream — passes the test. With it, the assertion is on the message.

## Table-driven tests

```rust
let cases = [("ok", 200u16, "success"), ...];
for (name, code, expected) in cases {
    assert_eq!(classify(code), expected, "case: {name} ({code})");
}
```

The third argument to `assert_eq!` is the message printed *only* on failure. One
test, seven cases, and a failure names the case instead of dumping two arrays.

## Testing the binary

```rust
std::process::Command::new(env!("CARGO_BIN_EXE_ch26"))
```

Cargo sets that variable for integration tests and builds the binary first. It
is the supported way to assert on exit codes and stdout — which, for a CLI, is
the actual contract. `--help` returning 0 and an invalid config returning 1 are
behaviour someone will script against.

## Test isolation

`tempfile::tempdir()` gives a unique directory that deletes itself on drop. Two
tests writing to `./config.txt` in the working directory will pass alone and fail
together, which is the most expensive kind of flaky.
