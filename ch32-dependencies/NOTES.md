# Chapter 32 — reading a dependency tree

The chapter's four inspection commands, with what each answers:

```bash
cargo tree --depth 1              # your direct dependencies
cargo tree -d                     # duplicate versions of the same crate
cargo tree --duplicates -i rand   # who pulls in the extra copy
cargo bloat --release --crates    # which crates dominate the binary size
cargo tree --edges features       # which feature turned which dependency on
```

## Version requirements, decoded

```toml
serde = "1"                  # ^1        -> >=1.0.0, <2.0.0
tokio = "1.38"               # ^1.38     -> >=1.38.0, <2.0.0
rand = "=0.8.5"              # exact pin, rare and usually wrong
local_utils = { path = "../utils" }
my_fork = { git = "https://github.com/me/fork", rev = "a1b2c3d" }
```

`^` (the default) means "compatible with", and for `0.x` versions that means
`^0.2.3` allows `0.2.9` but not `0.3.0` — pre-1.0 semver is treated as
"every minor version may break". That is why so many crates sit at `0.x` for
years: it buys the freedom to break.

`Cargo.lock` is what actually pins your build. Commit it for binaries.

## Turn defaults off

```toml
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
```

`default` features are convenience, and convenience is a dependency you did not
choose. `reqwest`'s default pulls in native-tls (and therefore OpenSSL on Linux);
`rustls-tls` is pure Rust and cross-compiles without a C toolchain. This is the
difference between `cargo build` working in a `scratch` container and not.

## Feature hygiene

- **Additive only.** A feature must never *remove* capability, or two dependents
  requesting different features get a combination nobody tested.
- **`dep:`** (as used here) keeps an optional dependency out of the implicit
  feature namespace, so `pattern` is the only name you expose.
- **Check the floor once in a while:**

```bash
cargo build -Z minimal-versions      # nightly: does the *lowest* allowed
                                     # version actually compile?
```

Semver-compatible does not mean "the version I tested". `cargo update -p foo
--precise 1.0.0` is the honest test.
