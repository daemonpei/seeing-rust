# Chapter 14 — Traits and Composition

```bash
cargo run -p ch14-traits
```

## One trait, three calling conventions

```rust
fn send_alert(n: &impl Notifier)      // sugar for the line below
fn send_alert2<N: Notifier>(n: &N)    // explicit generic
let ns: Vec<Box<dyn Notifier>> = ...  // dynamic dispatch
```

The first two are identical after type-checking: monomorphised, one copy of the
function per concrete type, no vtable. The third stores a vtable pointer and
decides at run time — necessary when the collection is heterogeneous, and the
only option when the type is chosen at run time (a plugin, a config value).

Cost of `dyn`: one pointer indirection per call, and no inlining across it.
Cost of generics: code size, and slower compiles. Reach for `dyn` at
architectural boundaries and generics inside.

## Default methods are the "abstract class" answer

```rust
fn name(&self) -> &'static str { "unnamed" }
```

`SlackNotifier` does not implement `name()` and gets `"unnamed"`. `EmailNotifier`
overrides it. No inheritance, no `super::name()` chain — just a function with a
body that implementors may replace.

## Composition, not inheritance

`Service` owns a `Box<dyn Notifier>`, a `Box<dyn Store>` and a `Box<dyn Clock>`.
It cannot call methods that belong to nothing it holds. Compare the usual
inheritance stack where `Service extends Base extends Component` and every
`Service` inherits a dozen capabilities it never uses — plus the diamond, plus
the fragile base class problem.

The test for this design: swap `MemoryStore` for a `PostgresStore` and nothing
in `Service::run` changes. That is what "depends on the abstraction" buys.

## Object safety

A trait can be used as `dyn` only if every method is callable without knowing
the concrete type — no generic methods, no `Self` return. `Notifier` qualifies;
add `fn clone_box(&self) -> Self` and it stops compiling as `dyn`. The error
message says "the trait cannot be made into an object", which is rustc's way of
telling you which method broke it.
