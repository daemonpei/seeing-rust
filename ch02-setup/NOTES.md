# Chapter 2 — the shell commands

Installing the toolchain:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Confirming it worked — three commands, three different questions:

```bash
rustc --version    # the compiler
cargo --version    # the build tool and package manager
rustup show        # which toolchains are installed, and which is active
```

Creating a project and running it:

```bash
cargo new hello-rust
cd hello-rust
cargo run
```

`cargo new` writes `Cargo.toml`, `src/main.rs` and `.gitignore`, and
initialises a git repository unless you are already inside one.

On Windows, rustup also installs the MSVC linker prerequisites check; if
`cargo build` complains about `link.exe`, install the Visual Studio Build Tools
with the "Desktop development with C++" workload. The `stable-x86_64-pc-windows-gnu`
toolchain avoids that but is a second-best option.

## The layout

```
hello-rust/
  Cargo.toml      # project metadata and dependencies
  Cargo.lock      # exact resolved versions (generated)
  src/
    main.rs       # your program starts here
  .gitignore
```

`Cargo.lock` is generated, but commit it for binaries — it is what makes a
rebuild six months later resolve to the same dependency versions. For libraries
the convention is to leave it out.
