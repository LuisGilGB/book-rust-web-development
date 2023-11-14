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

## Creating a connections pool to a PostgreSQL database

In order to handle multiple connections to the database, we need to create a
connection pool. The pool will handle the connections and will give us a
connection when we need it. We can also stablish a maximum number of connections.

```rust
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await?;

    // ...
}
```
