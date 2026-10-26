# Chapter 17 — Text Processing and Regular Expressions

```bash
cargo run -p ch17-text
```

## `&str` methods that allocate nothing

`trim`, `contains`, `starts_with`, `ends_with`, `split`, `split_once`,
`split_whitespace`, `strip_prefix`, `strip_suffix`, `find`, `chars`, `lines` —
all return `&str` slices or indices into the original. `replace`, `to_uppercase`
and `format!` allocate.

Rule of thumb: if the answer can be expressed as "a range of the input", it does
not allocate. `split_once('=')` on `"port=8080"` gives two `&str` pointing into
the original buffer.

## `Option`-returning parsers compose

```rust
let (key, value) = line.split_once('=')?;
```

`parse_setting` returns `Option<(&str, &str)>`, and `?` on `split_once` makes
"no equals sign" exit with `None` for free. No sentinel values, no `-1`, no
empty-string convention.

The signature also encodes that the returned slices borrow from the input — the
elided lifetime, again (chapter 10).

## Regex: compile once, reuse

```rust
fn level_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"level=(?P<lvl>\w+)").unwrap())
}
```

`Regex::new` in a hot loop is the single most common Rust performance mistake in
text code — a regex compile is tens of microseconds, and it shows up immediately
in a profile. `OnceLock` is in std since 1.70; `lazy_static` and `once_cell` are
no longer needed.

Named groups (`?P<lvl>`) beat positional indices: `&caps["lvl"]` still works
after you add a group in front.

## When not to use a regex

`parse_setting` beats `Regex::new(r"^\s*(\w+)\s*=\s*(.*)$")` on every axis:
faster, no dependency, no compile step, and it cannot panic on a bad pattern.
Split into at most two parts on a known delimiter first; reach for `regex` when
the shape genuinely varies (timestamps, log formats, version strings).
