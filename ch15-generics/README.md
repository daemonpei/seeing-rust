# Chapter 15 — Generics for Reusable Type-Safe Code

```bash
cargo run -p ch15-generics
```

## Bounds are a description of the body

```rust
fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T>
```

The compiler will not let you write `a > b` unless `T: PartialOrd`, and it will
not let you move `*a` out of the slice unless `T: Copy`. You are not declaring
permissions — you are documenting what the function already does, and rustc
checks that you were telling the truth.

The error you get for forgetting a bound is unusually good:

```
error[E0369]: binary operation `>` cannot be applied to type `T`
note: consider further restricting this bound: `T: PartialOrd`
```

## `impl Trait` vs `<T: Trait>`

In **argument** position they are the same thing; `&impl Notifier` is sugar for
a hidden generic. In **return** position they are not:

```rust
fn evens(xs: &[i32]) -> impl Iterator<Item = i32> + '_
```

The caller learns the behaviour, not the type. That is why you can change the
chain inside without breaking anyone — and why you cannot name the return type
in a `let` binding.

`+ '_` is the lifetime: the iterator borrows `xs`, so the result cannot outlive
it. Drop the `+ '_` and rustc asks for it by name.

## Conditional methods

```rust
impl<T: PartialOrd> Pair<T> { fn max(&self) -> &T { ... } }
impl<T: Display>    Pair<T> { fn label(&self) -> String { ... } }
```

`Pair<Vec<u8>>` gets neither method. `Pair<i32>` gets both. `Pair<MyType>` gets
whichever traits `MyType` implements — no `where` clause at the call site, no
runtime check.

## The monomorphisation trade

Generics are compiled per concrete type. `Vec<i32>` and `Vec<String>` are
different code. That is why generics cost nothing at run time and why a heavily
generic crate can produce a larger binary and a slower build. `dyn` trades the
opposite way (chapter 14).
