# Chapter 33 — Safety, Unsafe Rust and Security Basics

```bash
cargo run -p ch33-safety
API_TOKEN=abc cargo run -p ch33-safety
```

## Make the bad value unrepresentable

```rust
pub struct Port(u16);
impl Port { pub fn parse(raw: &str) -> Result<Self, ConfigError> { ... } }
```

`u16` already excludes `70000`. `Port` excludes `0` as well, and the only way to
build one is `parse`. Every function that takes `Port` is now free of a `if port
== 0` check — the check exists once, at the boundary, instead of in every
consumer.

This generalises: `Email`, `UserId`, `NonEmpty<T>`, `SanitisedPath`. The pattern
is called a *newtype*, and the compiler enforces it for free.

## Path traversal is a `components()` problem

```rust
if candidate.is_absolute()
    || candidate.components().any(|c| !matches!(c, Component::Normal(_)))
{
    return Err(...);
}
```

String checks (`contains("..")`) miss `..%2f` after decoding, `....//` on some
filesystems, and `..\` on Windows. `Path::components()` normalises all of that
into `ParentDir`, `RootDir`, `Prefix` and `Normal`, so the check is structural
rather than textual.

The second check (`full.starts_with(base)`) is belt and braces — cheap, and it
catches a symlink that resolves outside the base.

## Command injection

```rust
// WRONG: the filename becomes shell syntax.
Command::new("sh").arg("-c").arg(format!("cat {name}"));

// RIGHT: arguments are passed as separate values; no shell is involved.
Command::new("cat").arg(&name).status()?;
```

`Command::new("cat").arg(name)` passes `name` as one `argv` entry. There is no
shell, so `;`, `|`, `$(…)` and backticks are literal characters. The moment you
write `sh -c`, you have opted back into shell parsing, and every untrusted byte
becomes code.

The same shape applies to SQL: string interpolation is the injection, bind
parameters are the fix. The API differs, the principle does not.

## Bound what you allocate

```rust
router.layer(DefaultBodyLimit::max(64 * 1024));
if items.len() > MAX_ITEMS { return Err(ApiError::TooLarge); }
```

A `Content-Length: 999999999` header is an instruction to allocate. Reject before
parsing, and bound every collection whose size comes from the request.

## Writing `unsafe` you can defend

Every `unsafe fn` needs a `# Safety` doc section stating the caller's
obligations, and every `unsafe` block needs a `// SAFETY:` comment stating why
they are met *here*. `cargo clippy` warns on a missing safety doc; reviewers
should reject a missing safety comment.

The number of `unsafe` blocks in a project is a useful metric only when each one
has that paragraph. `unsafe { }` with no explanation is indistinguishable from a
bug.

## `Secret` beats discipline

```rust
impl Debug for Secret { fn fmt(...) { f.write_str("Secret([redacted])") } }
```

`#[derive(Debug)]` on a struct with a token means one `{:?}` anywhere — a log
line, a panic message, an `unwrap` — leaks it. A newtype that refuses to print
itself makes the leak a compile-time impossibility, and `expose()` marks every
deliberate read.
