# Create your first route handler

In this chapter we implement a simple server with its simple route handler with Warp.

When using Warp, both `warp` and `tokio` crates have to be added to the project.

## Filters

Rote handlers in Warp are implemented with Warp filters.

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .and(warp::path::end())
        .and_then(my_route_handler);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```

## Route handler

Route handler is a function that takes a request and returns a response. In Warp, route handlers are implemented as
async functions that return a `Result` of a `warp::Reply` (for the success) or a `warp::Rejection` (for the error
cases).

```rust
use warp::Filter;

async fn my_route_handler(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Hello, {}!", name))
}
```
