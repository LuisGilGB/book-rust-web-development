# Why Rust?

- More performant than garbage collected languages like Java.
- More readable than older languages like C.
- The compiler grants memory safety.

## Building blocks and asynchronous programming

Rust does not provide a built-in HTTP implementation, it goes until the Transport layer with a TCP implementation.
Fortunately, the community has provided enough good implementations for HTTP. Examples of this are `reqwest` as a module
for HTTP requests and `warp` as a web framework.

Similar happens with asynchronous programming: Rust needs to add a runtime that enables asynchronous syntax and
operations. The choice in this book is `tokio`. In `tokio`, any function that is asynchronous is preceded with
the `async` keyword and uses a `Future` type that is resolved (and polled until completion) with the use of `.await?`.
When using `tokio`, the main function must use its exported traits.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // App stuff
}
```