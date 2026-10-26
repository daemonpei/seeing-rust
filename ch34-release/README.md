# Chapter 34 — Building, Releasing and Running Rust Programs

```bash
cargo run -p ch34-release
cargo run -p ch34-release --release
cargo test -p ch34-release
```

```
version 0.1.0  commit 1a2b3c4

profile: dev
```

## `build.rs` injects build-time facts

```rust
println!("cargo:rustc-env=GIT_SHA={sha}");
```

becomes `env!("GIT_SHA")` in your code, at compile time — a `&'static str`, no
runtime cost. The same mechanism carries the target triple
(`TARGET`), the profile (`PROFILE`), and any generated file you need to embed.

`rerun-if-changed` is not optional: without it cargo only re-runs the script when
`build.rs` changes, so the SHA goes stale after every commit.

Always have a fallback. `git describe` fails in a source tarball, and a build
script that panics turns "extracted a zip" into "cannot compile".

## `env!` versus `option_env!`

```rust
const SHA: &str = env!("GIT_SHA");            // compile error if missing
const SHA: Option<&str> = option_env!("GIT_SHA");  // None if missing
```

Use `option_env!` for anything a downstream builder might not set. `env!` on a
variable only your build script sets is fine, precisely because the script
guarantees it.

## The Dockerfile, and why the manifests come first

```dockerfile
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked
```

Layer order is cache order. Copying `src/` after the manifests means a one-line
source change re-runs the final build but reuses the dependency layer — for a
crate with `tokio` and `axum` that is minutes versus seconds. (`cargo-chef` is
the tool that automates this properly.)

`--locked` makes the build fail if `Cargo.lock` disagrees with `Cargo.toml`,
which is what stops "works on my machine" dependency drift.

Non-root, fixed UID (`10001`) so a Kubernetes `securityContext` can assert on it.

## What actually differs in a release build

Beyond optimisation: `debug_assert!` is compiled out, integer overflow becomes
wrapping instead of panicking, and `cfg!(debug_assertions)` is false. That second
one is the important one — a bug that panics in dev can silently wrap in release,
which is why `overflow-checks = true` is worth keeping on for services that
handle money.
