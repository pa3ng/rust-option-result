# Option, Result, and more

## On `null`...

Rust does not have a `null`. This is great, and something Kotlin aspired to (but, being no Java, could never fully do). Instead, it has [`Option<T>`](https://doc.rust-lang.org/std/option/index.html). An `Option` is an `enum` and can be either `None` or `Some(T)`. Here's an example from the documentation:

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

let result = divide(2.0, 3.0);

match result {
    Some(x) => println!("Result: {}", x),
    None => println!("Cannot divide by 0"),
}
```

## On exceptions...

Rust, likewise, does not do exceptions. Instead, it has the [Result<T, E>](https://doc.rust-lang.org/std/result/) enum, which can be either `Ok(T)` or `Err(E)`, like so:

```rust
#[derive(Debug)]
enum Version { Version1, Version2 }

fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
    match header.get(0) {
        None => Err("invalid header length"),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("invalid version"),
    }
}

let version = parse_version(&[1, 2, 3, 4]);
match version {
    Ok(v) => println!("working with version: {:?}", v),
    Err(e) => println!("error parsing header: {:?}", e),
}
```

## On the `?`

Rust requires that a `Result` be used: you can not ignore results. [`unwrap`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap) is the easiest way to use a result and will panic if the result was not `Ok(T):

```rust
std::fs::File::create("valuable_data.txt").unwrap();
```

`unwrap` can be cumbersome. As a convenience, you can use [`?`](https://doc.rust-lang.org/std/result/#the-question-mark-operator-) instead if the function returns a `Result`.

## On `Default + PartialEq`

Consider the following:

```rust
fn is_none_or_default<T: Default + PartialEq>(obj: &Option<T>) -> bool {
    match obj {
        Some(o) => *o == T::default(),
        None => true,
    }
}
```

The `<T: Default + PartialEq>` says that the type `T` must be both [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) and [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html). Both are [traits](https://doc.rust-lang.org/book/ch10-02-traits.html), which are recognizable as interfaces.

For `Default`:

```rust
pub trait Default {
    fn default() -> Self;
}

impl Default for String {
    /// Creates an empty `String`.
    fn default() -> String {
        String::new()
    }
}
```

For `PartialEq`, notice that this trait allows the right hand side to be a different type; you must implement `eq` for every type you wish to compare to:

```rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    /// This method tests for `self` and `other` values to be equal, and is used
    /// by `==`.
    fn eq(&self, other: &Rhs) -> bool;

    /// This method tests for `!=`.
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}
```