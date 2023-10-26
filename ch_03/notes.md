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

## Handling errors in Warp

By default, when a request is rejected, Warp returns a 404 response. This can be changed by using the `recover` filter,
where we can specify a custom error handler that works with the rejections returned by the previous filters.

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .and(warp::path::end())
        .and_then(my_route_handler)
        .recover(handle_rejection);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```

But to return a custom error, we need:

1. To create a custom error type.
2. To implement the `warp::reject::Reject` trait for the custom error type.
3. Return the custom error type from the route handler.

```rust
use warp::{Filter, reject::Reject, Reply, Rejection};

#[derive(Debug)]
struct CustomError;

impl Reject for CustomError {}

async fn my_route_handler(name: String) -> Result<impl Reply, Rejection> {
    Err(warp::reject::custom(CustomError))
}
```

To check what type of error was returned, we can use the `find` method on the `Rejection` type.

```rust
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    match r.find() {
        Some(InvalidId) => {
            Ok(warp::reply::with_status(
                "No valid id provided",
                StatusCode::UNPROCESSABLE_ENTITY,
            ))
        }
        _ => {
            Ok(warp::reply::with_status(
                "Route not found",
                StatusCode::NOT_FOUND,
            ))
        }
    }
}
```
