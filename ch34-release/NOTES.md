# Chapter 34 — cross-compiling and containers

## musl: the static binary

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

The musl target links no libc dynamically, so the result runs on any Linux
kernel — including `FROM scratch`. That is the smallest possible container:

```dockerfile
FROM scratch
COPY target/x86_64-unknown-linux-musl/release/app /app
ENTRYPOINT ["/app"]
```

Caveat: musl's allocator is slower under heavy multithreaded allocation than
glibc's. For an I/O-bound service it does not matter; for an allocation-heavy
one, measure before switching. `mimalloc` or `jemallocator` closes most of the gap.

## The profile, with the trade spelled out

```toml
[profile.release]
opt-level = 3
lto = "thin"            # "fat" is smaller/faster, but much slower to link
codegen-units = 1        # better optimisation, longer builds
panic = "abort"          # smaller binary; no unwinding
strip = "symbols"        # smaller binary; keep a separate debug build

[profile.dev]
opt-level = 0
debug = true             # full debug info while developing
```

| Setting | Effect | Cost |
|---|---|---|
| `lto = "fat"` | 5-15% faster | link time multiplies |
| `codegen-units = 1` | 5-10% faster | no parallel codegen |
| `panic = "abort"` | 10-20% smaller | `catch_unwind` stops working, tests too |
| `strip = "symbols"` | ~30% smaller | no useful backtraces |

`panic = "abort"` and `strip = "symbols"` together are why a "why did production
panic" investigation is so much harder than it needs to be. Ship a second
artifact with symbols, or set `debug = true` in release (symbols stay, size grows
by a few MB) and keep `strip` off until you have to.

## Cross-compiling to macOS or Windows

Rust has no built-in linker for foreign targets. Options, in order of pain:

1. **CI matrix.** `ubuntu-latest` / `windows-latest` / `macos-latest` natively
   build their own target. Slow, zero setup, always correct.
2. **`cross`.** `cargo install cross && cross build --target aarch64-unknown-linux-gnu`
   — runs the build inside a container with the right toolchain.
3. **`cargo-zigbuild`.** Uses zig as the C compiler and linker, handles glibc
   version targeting (`--target x86_64-unknown-linux-gnu.2.17`) better than
   anything else.

`cargo build --target x86_64-pc-windows-gnu` from Linux needs `mingw-w64`
installed, and any crate with a C dependency needs its C code cross-compiled too.
That last part is where most cross-compilation attempts die.
