# Add a database to your application

Crates for ORM:

- [diesel](https://crates.io/crates/diesel)

Crates for writing SQL queries:

- [sqlx](https://crates.io/crates/sqlx)

## SQLx characteristics

- It is asynchronous
- Supports MySQL, PostgreSQL, SQLite, and Microsoft SQL Server
- The PostgreSQL and SQLite backends are implemented in Rust
- It works with different runtimes (`tokio`, `async-std`, `actix`...)
- Widely used in the community

Drawbacks:

- It is not an ORM
- It can't verify the correctness of the SQL queries at compile time
