# Integrate third-party APIs

An app is likely to need services provided by others that are consumed via a third party. The usual way to call those
services is making HTTP requests, something your app probably needs an appropriate HTTP client for. The most popular
HTTP clients for Rust are:

- [reqwest](https://crates.io/crates/reqwest) (valid for Tokio)
- [awc](https://crates.io/crates/awc) (valid for Actix Web)
