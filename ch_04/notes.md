# Implement a RESTful API

## Some notes on HashMap keys

In Rust, we can declare hash maps with the `HashMap` type from the standard library.

In principle, we can use any type as the key, but in practice, we need to meet some requirements for the key type. The
most important of these requirements is that the key must implement the `Eq` and `Hash` traits. The `Eq` trait indicates
that two values are equal to each other, and the `Hash` trait allows these values to be hashed.

The `Hash` trait is important because it allows the key to be used in a hash map. The `Hash` trait is also implemented
for Rust's primitive types, such as `u32` and `String`. The `Eq` trait is implemented for all types that implement
the `PartialEq` trait. The `PartialEq` trait is used to compare two values for equality, and it is implemented for all
primitive types by default.

## Deserializing JSON files

The `serde` crate provides a `Deserialize` trait that can be used to deserialize JSON files into Rust structs. The
`serde_json` crate provides a `from_str` function that can be used to deserialize a JSON string into a Rust struct.

A JSON file can be read as a string at compile time using the `include_str!` macro.

Both tools can be combined the following way to parse a struct from a JSON file:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct MyStruct {
    // ...
}

fn main() {
    let json = include_str!("my_file.json");
    let my_struct: MyStruct = serde_json::from_str(json).expect("Parsing error");
}
```

Notice how the `Deserialize` trait is derived for the struct.

## Closures and the move keyword

Closures are anonymous functions that can capture their environment. This means that they can use variables that are
defined in the same scope that they are defined in. Closures are useful for defining callbacks and other functions that
are used as arguments to other functions.

Closures can use the `move` keyword to take ownership of used variables that come from the scope the function is defined
in.

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let store = warp::any().map(|| {
        let mut store = HashMap::new();
        store.insert(1, "one");
        store.insert(2, "two");
        store.insert(3, "three");
        store
    });

    let store_filter = warp::any().map(move || store.clone());
}
```
