# Integrate third-party APIs

An app is likely to need services provided by others that are consumed via a third party. The usual way to call those
services is making HTTP requests, something your app probably needs an appropriate HTTP client for. The most popular
HTTP clients for Rust are:

- [reqwest](https://crates.io/crates/reqwest) (valid for Tokio)
- [awc](https://crates.io/crates/awc) (valid for Actix Web)

## Concerns about errors using third-party libraries

When using third-party libraries, some compilation errors may arise with descriptions like "the trait bound `...` is not
satisfied" due to an error trait we're using is not implemented in the third-party library. As we cannot modify what
this library implements, we have to deal with this scenarios with other strategies like wrapping the third party
structure in a first-party one that satisfies our app's requirements.

## Reqwest

Reqwest is a simple and easy-to-use HTTP client. An example of a POST request with `reqwest` is:

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .header("Content-Type", "application/json")
        .body("the exact body that is sent")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);
    Ok(())
}
```
