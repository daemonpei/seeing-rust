# Chapter 12 — Vectors, Strings and Hash Maps

```bash
cargo run -p ch12-collections
```

## `[]` panics, `get()` returns `Option`

```rust
ports[0]        // panics on out-of-range
ports.get(0)    // Option<&u16>
```

Use indexing when the bound is structurally guaranteed (a loop over `0..len`,
an index you just checked). Use `get` for anything that came from outside the
program. The cost is the same; the difference is who goes down.

## `String` is a `Vec<u8>` that is always UTF-8

`String` owns; `&str` borrows. `s[0]` does not compile, because indexing a
UTF-8 string by byte could land in the middle of a character. You get:

```rust
s.chars()        // Unicode scalar values
s.bytes()        // raw bytes
s.chars().count() // characters, not bytes -- what "length" usually means
```

`"héllo".len()` is 6 (bytes) while `"héllo".chars().count()` is 5. That gap is
the source of a whole family of truncation bugs in other languages.

## The `entry` API

```rust
counts.insert("ok".to_string(), 1);                          // overwrites
counts.entry("ok".to_string()).and_modify(|c| *c += 1)
      .or_insert(1);                                          // read-modify-write
*c_counts.entry("ok".to_string()).or_insert(0) += 1;          // the one you want
```

`entry` computes the hash once and gives you a slot you can modify in place.
`contains_key` then `insert` hashes twice, and `if let Some(v) = get_mut(..)`
hashes twice as well.

## Sort before you print

`HashMap` iteration order is deliberately randomised per process (HashDoS
resistance). Any output built from a `HashMap` will differ between runs — sort
the keys, or use `BTreeMap` when order is part of the contract.
