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

## Query parameters in Warp

Warp provides a `query` method that can be used to extract query parameters from a request. The `query` method takes a
key as an argument and returns a `Filter` that can be used to extract the value of the query parameter with that key.

The route handler then must be a function that takes the query parameters structure (usually a `HashMap`) as an
argument.

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path!("hello")
        .and(warp::query::<HashMap<String, String>>())
        .map(|query_params: HashMap<String, String>| {
            format!("Hello, {}!", query_params.get("name").unwrap())
        });
}
```

## Modifying states: Concurrency challenges

We know that Rust only allows one routine to own a value at a time. This includes function callings, which means that
when we pass a value as an argument or we use a variable to assign another one, the previous holder of that value ends
up losing ownership of it and remains as `uninit`.

```rust
fn main() {
    let x = 1;
    let y = x;
    // x is now uninit (uninitialized)
}
```

A consequence of this policy is that **no value can be shared between threads**. We can think in measures to avoid this
obstacle, but they have important drawbacks:

- Copying the values. <- Pollutes the memory and struggles when mutations are needed.
- Waiting one task to finish before starting another one.  <- Kills the purpose of concurrency.

Fortunately, Rust provides tools to manage data structures under this difficult scenarios.

### For reading

`Rc` and `Arc`. `Arc` allocates the value on the heap and a pointer to it on the stack, being this pointer the actually
copied thing.

- `Rc` works only on **single-threaded** systems. It stands for **reference counted**.
- `Arc` works on **multi-threaded** systems. It stands for **atomically reference counted**.

```rust
use std::sync::Arc;

fn main() {
    let x = Arc::new(String::from("Hello"));
    let y = Arc::clone(&x);
    // The "Hello" string is encapsulated in a heap-allocated memory direction with a counter.
    // This counter counts the number of pointers to it (2 in this case).
    // When the counter reaches 0, `Arc` drops the value and the memory is freed.
}
```

### For writing

`Mutex` and `RwLock`.

- `Mutex` stands for **mutual exclusion**. It allows only one thread to access the data at a time.
- `RwLock` stands for **read-write lock**. It allows multiple threads to read the data at the same time, but only one
  thread to write it.

Notice that both of them come from the `std::sync` module, which means that they are meant to be used for synchronous
tasks. Fos asynchronous operations, we have to use the `tokio::sync` module.

```rust
use std::sync::RwLock;

#[tokio::main]
async fn main() {
    let x = RwLock::new(String::from("Hello"));
}
```

## Handling create, update and delete requests

We can use the `warp::post` method to create a route that only matches `POST` requests. We can use
the `warp::body::json` method to extract the JSON body from the request.

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path!("store")
        .and(warp::post())
        .and(warp::body::json())
        .map(|item: Item| {
            // ...
        });
}
```

Similar can be done for `PUT`, `PATCH` and `DELETE` requests. Following REST good practices, these methods require a
parameter in
the path to identify the resource that is being updated. We can use the `warp::path::param` method to extract this
parameter from the path.

In terms of the `HashMap` type, we can use the `insert` method to insert a new key-value pair into the map. If we just
want to update the value of an existing key, we obtain a mutable reference with the `get_mut` method and assign the new
value to it.

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path!("store")
        .and(warp::put())
        .and(warp::path::param::<u32>())
        .and(warp::body::json())
        .and(warp::any().map(move || store.clone()))
        .map(|id: u32, item: Item, store: HashMap<u32, Item>| {
            store.get_mut(&item.id).map(|i| *i = item);
        });
}
```

## Reading url-form-encoded data

Warp allows the reading of `url-form-encoded` data with the `warp::body::form` method.

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path!("store")
        .and(warp::post())
        .and(warp::body::form())
        .map(|item: Item| {
            // ...
        });
}
```
