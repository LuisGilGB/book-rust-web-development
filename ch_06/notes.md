# Logging, tracing, and debugging

Standard outputs in Rust:

- `println!` macro for `stdout`
- `eprintln!` macro for `stderr`

```rust
fn main() {
    println!("Hello, world!");
    eprintln!("Hello, world!");
}
```

Logging most times uses `stderr`, although logging to another channel, appending lines to a file or calling a remote
service are available options (like any other or a combination of many, it's up to the developer to decide and
implement).
