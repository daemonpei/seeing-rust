# Chapter 3 — Your First Rust Program

```bash
cargo run -p ch03-first-program
```

```
Hello, systems engineers!
This is line 2 of 2

(created config.txt for this demo)
read 32 bytes
```

Three things worth noticing, all of them on the page in chapter 3:

1. **`println!` is a macro, not a function.** The `!` is not decoration. It
   expands to formatting code that type-checks your arguments at compile time —
   which is why a bad format string (`{}` with no argument) is a build error and
   not a crash.
2. **`{audience}` is inline capture.** Since Rust 2021 you can name a variable
   directly inside the braces; `println!("Hello, {}!", audience)` means the same
   thing.
3. **`fn main() -> Result<…>` is the polite way to fail.** With `?`, an error
   prints `Error: …` to stderr and exits non-zero. No `unwrap`, no panic.

## The `E0425` listing

The chapter prints the compiler error you get from misspelling `audience` as
`audiance`. It is kept as a comment at the bottom of `main` so you can
un-comment it and watch rustc suggest the fix — the suggestion machinery is
part of the language's usability, not an editor feature.
