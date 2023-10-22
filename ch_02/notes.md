# Laying the foundation

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