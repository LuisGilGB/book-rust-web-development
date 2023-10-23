# Laying the foundation

## Tools

Docs can be opened online with the docs component:

```bash
rustup component add rust-docs
rustup doc --std
```

Docs can also be generated for a project locally using Cargo:

```bash
cargo doc --open
```

Ideas can be tried quickly in Rust in its [playground](https://play.rust-lang.org/).

## Creation of new types

New types can be defined using the  `struct` keyword. The syntax is as follows:

```rust
// A type that works as an alias of String:
struct QuestionId(String);

// A type that works as an alias of a tuple:
struct AnswerId(u32, u32);

// A type expressing an object with a determinate structure:
struct Question {
    id: QuestionId,
    title: String,
    body: String,
    tags: Vec<String>,
    answers: Vec<Answer>,
}
```

## Implementation of methods

Methods are functions that are defined inside the context of a type. They are defined using the `impl` keyword. The
syntax is as follows:

```rust
impl Question {
    // A method that returns the title of a question:
    fn title(&self) -> &str {
        &self.title
    }
}
```

We can also define methods that create new instances, as a constructor:

```rust
impl Question {
    // A method that creates a new instance of Question:
    fn new(id: QuestionId, title: String, body: String, tags: Vec<String>) -> Self {
        Self {
            id,
            title,
            body,
            tags,
            answers: Vec::new(),
        }
    }
}
```

Rust doesn't have a default name for the constructor, so the name `new` is usually chosen to implement one.

## Understanding options

When a value might or might not exist, we use the `Option` type. Given Rust is strictly typed, we cannot define optional
properties. Instead, we define those properties as `Option` types, that might be resolved as `Some` if a value exists
or `None` if it doesn't.

```rust
struct Book {
    title: String,
    isbn: Option<String>
}

impl Book {
    fn new(title: String, isbn: Option<String>) -> Self {
        Self {
            title,
            isbn
        }
    }
}

fn main() {
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        isbn: Some("978-1-59327-828-1".to_string())
    };
}
```

Options can be used with `match` statements to handle the different cases:

```rust
fn main() {
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        isbn: Some("978-1-59327-828-1".to_string())
    };

    match book.isbn {
        Some(isbn) => println!("ISBN: {}", isbn),
        None => println!("ISBN not available")
    }
}
```

## Functions onto types

Rust has two ways of implementing functions that are attached to types. They are:

- **Associated functions** (comparable to other languages' static methods, they are called with `::`).
- **Methods** (they are called with `.` and take the instance (&self) as the first parameter).

## String and &str

Rust has two types to represent strings:

- `String`: A heap-allocated string. Owned and resizable.
- `&str`: A string slice, a reference to a string. Immutable.

A rule of thumb is to use `&str` for read-only parameters and `String` for return values or parameters that we want to
own or modify.

## Ownership and borrowing

Rust has a strict *ownership* model. This means that every value has an owner, and there can only be one owner at a
time. When the owner goes out of scope, the value will be dropped.

If a variable is created from another variable, the ownership is transferred to the new variable, while the old one is
no longer valid.

When we pass a value to a function, we are *moving* ownership of that value to the function. This means that the value
will be dropped when the function returns.

If we want to pass a value to a function without moving ownership, we can use *borrowing*. This is done by passing a
reference to the value instead of the value itself. References are created using the `&` operator.

## Traits

Traits are the way Rust enable implementation of shared behaviours between types. They can be compared to interfaces,
abstract classes or mixins in other languages. They are, so, the way to implement polymorphism in Rust.