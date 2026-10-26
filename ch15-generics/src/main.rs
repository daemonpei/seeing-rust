//! Chapter 15: Generics for Reusable Type-Safe Code

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

fn main() {
    // ---- without generics you write the same body twice ------------------
    let names = vec![String::from("a"), String::from("b")];
    let numbers = vec![3, 1, 4];
    println!("{:?} {:?}", first_string(&names), first_int(&numbers));

    // ---- with generics: once --------------------------------------------
    println!("{:?} {:?}", first(&names), first(&numbers));

    // ---- bounds are the compiler asking for what the body needs ----------
    println!("{}", contains(&numbers, &4));
    println!("{:?}", largest(&numbers));
    println!("{:?}", largest(&["pear", "apple", "zucchini"]));

    let mut scores = HashMap::new();
    scores.insert("api", 3);
    scores.insert("web", 7);
    println!("{}", render(&scores));

    // ---- generic structs, with impl blocks that add methods conditionally --
    let pair = Pair {
        left: 3,
        right: 9,
    };
    println!("max {} label {}", pair.max(), pair.label());

    let mixed = Mixed {
        a: "left",
        b: 2.5,
    };
    println!("{:?}", mixed);

    // ---- From gives you Into for free -----------------------------------
    let a: Celsius = 21.5.into(); // from the blanket impl
    let b = Celsius::from(21.5); // explicit form
    println!("{} == {} ? {}", a.0, b.0, a == b);

    // ---- impl Trait in return position ----------------------------------
    println!("{:?}", evens(&[1, 2, 3, 4, 5, 6]).collect::<Vec<_>>());

    // ---- a boxed closure when the branches differ -----------------------
    let f = make(true);
    let g = make(false);
    println!("{} {}", f(10), g(10));
}

fn first_string(items: &[String]) -> Option<&String> {
    items.first()
}

fn first_int(items: &[i32]) -> Option<&i32> {
    items.first()
}

fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

/// `T: PartialEq` is required because == needs it.
fn contains<T: PartialEq>(items: &[T], target: &T) -> bool {
    items.iter().any(|item| item == target)
}

/// `T: Ord` is required because sorting needs a total order.
fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    items.iter().copied().reduce(|a, b| if a > b { a } else { b })
}

/// A `where` clause keeps long bounds readable.
fn render<K, V>(map: &HashMap<K, V>) -> String
where
    K: Display + Hash + Eq,
    V: Display,
{
    let mut pairs: Vec<String> = map.iter().map(|(k, v)| format!("{k}={v}")).collect();
    pairs.sort();
    pairs.join(",")
}

#[derive(Debug, Clone, PartialEq)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T: PartialOrd> Pair<T> {
    fn max(&self) -> &T {
        if self.left > self.right {
            &self.left
        } else {
            &self.right
        }
    }
}

impl<T: Display> Pair<T> {
    fn label(&self) -> String {
        format!("{} | {}", self.left, self.right)
    }
}

/// Different types are allowed too.
#[derive(Debug)]
#[allow(dead_code)] // the fields are only ever read through Debug
struct Mixed<A, B> {
    a: A,
    b: B,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Celsius(f64);

/// Implementing From gives you Into, at no extra cost.
impl From<f64> for Celsius {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

/// Returns "some iterator" -- callers get the methods, not the type name.
fn evens(xs: &[i32]) -> impl Iterator<Item = i32> + '_ {
    xs.iter().copied().filter(|n| n % 2 == 0)
}

/// Returning a boxed trait object when the branches differ.
fn make(flag: bool) -> Box<dyn Fn(i32) -> i32> {
    if flag {
        Box::new(|n| n + 1)
    } else {
        Box::new(|n| n * 2)
    }
}
