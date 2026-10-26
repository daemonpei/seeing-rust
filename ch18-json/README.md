# Chapter 18 — JSON and Structured Data

```bash
cargo run -p ch18-json
```

Dependencies:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Two crates, two jobs

`serde` is the framework: it defines `Serialize`/`Deserialize` and generates the
code from `#[derive]`. `serde_json` is one format implementation. Swap it for
`toml` or `rmp-serde` and the struct does not change — that is the whole point
of the split.

## `rename` makes the wire format explicit

```rust
#[serde(rename = "timeoutSeconds", default = "default_timeout")]
timeout_seconds: u64,
```

Rust style is `snake_case`; most JSON APIs are `camelCase`. Renaming at the
boundary keeps both conventions honest instead of forcing one to be wrong.

`default = "default_timeout"` names a function, so the default can be computed —
`default` alone only works when the field's type implements `Default`.

## `Option` plus `default` is the partial-update idiom

```rust
#[serde(default)] pub name: Option<String>,
```

| JSON | `name` |
|---|---|
| absent | `None` |
| `"name": null` | `None` |
| `"name": "a"` | `Some("a")` |

Without `#[serde(default)]`, an absent field is an error. With it, absent means
`None` — which is what "the client did not mention this field" should mean.
(`Option<T>` already accepts explicit `null`; `default` is what accepts absence.)

## Internally tagged enums

```rust
#[serde(tag = "kind", rename_all = "snake_case")]
enum Event { Started { pid: u32 }, ... }
```

produces `{"kind":"started","pid":41}`. `serde` also supports `#[serde(tag =
"t", content = "c")]` (adjacently tagged) and the default externally tagged
form, which nests the variant name as a key. Pick one and keep it — clients end
up parsing it.

## Parsing is not validation

`serde_json` proves the document has the right *shape*: `port` is a number that
fits in `u16`. It says nothing about whether `port == 0` is meaningful or
whether `"prod"` is a legal environment. `validate` is a separate function
returning a list of problems, so one run reports all of them instead of dying on
the first.
