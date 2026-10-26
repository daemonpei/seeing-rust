# Chapter 16 — File I/O and Paths

```bash
cargo run -p ch16-files
```

Everything is written under `std::env::temp_dir()`, so running it twice is fine
and it never touches the repository.

## Four reads, four situations

| Need | Call | Cost |
|---|---|---|
| whole small file | `fs::read_to_string(p)` | one allocation, whole file in memory |
| large file, line by line | `BufReader::new(File::open(p)?).lines()` | one line alive at a time |
| bytes, not text | `fs::read(p)` | `Vec<u8>` |
| stream into a parser | `File::open` + `serde_json::from_reader` | no intermediate `String` |

`read_to_string` on a 2 GB log file is how you get OOM-killed. `lines()` on a
small config file is fine but slower than reading it whole — it allocates a
`String` per line.

## Append needs `OpenOptions`

`fs::write` always truncates. There is no `append: true` shorthand, because
truncation is the safe default:

```rust
let mut f = fs::OpenOptions::new().create(true).append(true).open(&p)?;
writeln!(f, "port=8080")?;
```

## Atomic replace

```rust
fs::write(&tmp, contents)?;
fs::rename(&tmp, &path)?;     // atomic on the same filesystem
```

A reader either sees the old file or the new one, never a half-written file.
This fails with `EXDEV` across filesystems (temp dir on a different mount than
the target), which is why the temp file must be a sibling of the target —
`path.with_extension("tmp")`, not `std::env::temp_dir()`.

## `&Path` in signatures

```rust
fn load(p: &Path) -> std::io::Result<String>
```

Accepts `&PathBuf`, `&Path`, `&str` (via `Path::new`) and `String::as_ref()`.
Taking `&str` would reject a `PathBuf` the caller already has; taking
`&PathBuf` would reject a `&str` literal. Same argument as `&str` over
`&String` in chapter 12.

## Match the error, do not pre-check

```rust
Err(e) if e.kind() == ErrorKind::NotFound => { /* defaults */ }
```

`if !p.exists() { ... }` then `fs::read_to_string(p)` has a gap: the file can
disappear between the two calls (TOCTOU). Matching `ErrorKind` has no gap,
because the failure is the check.
