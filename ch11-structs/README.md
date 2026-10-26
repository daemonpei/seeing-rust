# Chapter 11 — Structs, Enums and Domain Models

```bash
cargo run -p ch11-structs
```

## Structs are records. Enums are choices.

```rust
struct Server { name: String, port: u16, healthy: bool }   // all fields at once

enum Job {                                                 // exactly one variant
    Queued,
    Running { worker: u32, progress: f32 },
    Done(u64),
    Failed(String),
}
```

A `struct` with a `status: String` field and a comment saying "only 'queued' or
'running' are valid" is the design `Job` replaces. The enum makes the invalid
state unrepresentable, and `describe` cannot forget a variant because `match`
is exhaustive.

## Which variant is this? The compiler knows

```rust
match job {
    Job::Done(ms) => ...          // ms: u64, bound by the pattern
    Job::Running { progress, .. } => ...   // progress: &f32, `..` ignores worker
}
```

Reaching into a `Done(u64)` requires first proving it *is* a `Done`. There is no
cast, no `isinstance`, no null check on a field that only exists in one variant.

## Struct update syntax

```rust
let next = Server { port: 9090, ..base };
```

Fills the remaining fields from `base`. For `Copy` fields this copies; for
`String` it **moves**, leaving `base` partially moved and unusable as a whole.
If you need both, `base.clone()` first — and if the clone is expensive, that is
a signal the field should be an `&str` or a shared pointer.

## `#[derive]` is not magic

`Debug`, `Clone`, `PartialEq` are ordinary traits with compiler-generated
implementations. `#[derive(Debug)]` prints every field — which is why chapter 28
hand-writes `Debug` for anything holding a token.
