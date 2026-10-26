# Chapter 1 — What Rust Is and Why Systems Engineers Use It

```bash
cargo run -p ch01-hello
```

```
Rust is ready for systems work.
```

That is the whole code listing in the chapter. The point of the chapter is the
panel on the page, not the program: before `main` runs, rustc has already
checked types, ownership and borrows, and refused to build anything that would
be wrong at run time.

The three checks, in the order the compiler applies them:

| Check | What it catches | Error family |
|---|---|---|
| Types | `let n: i32 = "x";` | `E0308` mismatch |
| Ownership | using a value after it was moved | `E0382` moved value |
| Borrows | `&a` and `&mut a` alive at the same time | `E0502` borrow conflict |

Chapters 4, 9 and 10 take each of these apart in turn.
