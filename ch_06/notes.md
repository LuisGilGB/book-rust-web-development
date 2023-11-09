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

In Rust, logging usually takes two libraries: a facade one and another one that implements the methods for the facade,
being the most popular ones:

- For acting as a facade: [`log`](https://crates.io/crates/log)
- For implementing the facade:
    - [`env_logger`](https://crates.io/crates/env_logger)
    - [`simple_logger`](https://crates.io/crates/simple_logger)
    - [`pretty_env_logger`](https://crates.io/crates/pretty_env_logger)
    - [`fern`](https://crates.io/crates/fern)
    - [`tracing`](https://crates.io/crates/tracing)
    - [`slog`](https://crates.io/crates/slog)
    - [`sloggers`](https://crates.io/crates/sloggers)
    - [`flexi_logger`](https://crates.io/crates/flexi_logger)
    - [`log4rs`](https://crates.io/crates/log4rs)

`log` provides macros for logging at different levels:

- `trace!`
- `debug!`
- `info!`
- `warn!`
- `error!`

```rust
use log::{debug, error, info, trace, warn};

fn main() {
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
}
```

## Logging route handlers with `warp`

`warp` provides a filter that can be used to print the same logs for all the routes it is applied. This filter can be
applied to the routes handler then with the `with` method.

```rust
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    log::info!("Starting server...");

    let log = warp::log::custom(|info| {
        log::info!("{} {} {} - {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        )
    });

    //...

    let routes = route_handler
        // ... whatever is added
        .with(log)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
```
