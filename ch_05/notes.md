# Clean up your codebase

## Modules

Rust and Cargo allow you to organize your code in modules, making also a clear distinction between applications and
libraries. The Cargo commands to create any of both are the following ones:

```bash
cargo new my_app --bin # Creates a binary application
cargo new my_lib --lib # Creates a library
```

Only applications are meant to be executed, while libraries are meant to be used by other applications or libraries. Due
to this, only applications can create binary (executable) files.

### Modules inside a client file

Modules can also be defined inside a file with the `mod` keyword. Use this reasonably, when having a bigger file is good
enough compared to creating a new module outside (because of the module being too small and simple to worth the effort
of maintaining a library, for instance).

Keep in mind that these modules have no access to the parent module's scope. Any stuff needed from outside has to be
imported within the module block.

To use stuff from this module, two things are required:

- Making the element to be used public with the `pub` keyword.
- Consuming it with the `use` keyword or the module name and the `::` operator.

```rust
mod my_module {
    pub fn my_function() {
        // ...
    }
}

use my_module::my_function;
```

### Modules from another file

To expose a module from a file, we can wrap the code inside a `pub mod` block. `mod` contains the scope of the
module and `pub` makes it public.

```rust
// src/my_module.rs
pub mod my_module {
    pub fn my_function() {
        // ...
    }
}
```

This wrapping is not required if the module is defined in a file with the same name as the module. In that case, the
module is automatically exposed. Otherwise, consumers of the module code would have to call it with the module name
prepended:

```rust
// src/main.rs
use my_module::my_module::my_function;
```

To allow a client file of that module to consume it, the module must be referenced with the `mod` keyword. The `use`
keyword can be then specified to import specific elements from the module.

```rust
// src/main.rs
mod my_module;
```

When organizing files in folders, a `mod.rs` file is a way to define an entrypoint for all the modules in that folder.
The use of `pub mod` is required in this case.

```rust
// src/my_module/mod.rs
pub mod my_module;
```

Between modules that are not the main file or the entrypoint of the crate, elements can be imported with the `use`
keyword and the `crate` keyword following the module location and name:

```rust
// src/another_module/another_module.rs
use crate::my_module::my_function;
```

This only works for modules available in the main file of the crate.

## Library crates in the workspace

Cargo allows you to define a workspace with multiple crates. This is useful when you have a library that is used by
multiple applications, for instance. To create a workspace, create a `Cargo.toml` file in the root folder of the
workspace and add the following lines:

```toml
[workspace]
members = [
    "my_lib",
    "my_app"
]
```

To create a library crate, use the `cargo new` command with the `--lib` flag. To create an application crate, use the
`--bin` flag:

```bash
cargo new my_lib --lib
cargo new my_app --bin
```

To use a library crate in an application crate, add the following line to the `Cargo.toml` file of the application:

```toml
[dependencies]
my_lib = { path = "../my_lib" }
```

## Creating documentation

Rust provides tools to create documentation from your code. A template version of the documentation is generated from
the code itself, but more information can be added with comments. The documentation is generated with the `cargo doc`
command.

The comments that are included in the documentation use the following syntax:

```rust
/// This is a line comment that is included in the documentation
/** This is a block comment that is included in the documentation */
//! This is a line comment that is included in the documentation applied to the previous block
/*! This is a block comment that is included in the documentation applied to the previous block */
```

Comments that are not included in the documentation use the following syntax:

```rust
// This is a line comment that is not included in the documentation
/* This is a block comment that is not included in the documentation */
```

### Code in documentation

Code can be included in the documentation as code snippets for Markdown. To do so, use the following syntax:

```rust
/// ```rust
/// let x = 5;
/// ```
```

Test assertions can be included in the documentation as well. This grants your documentation to be properly up to date:

```rust
/// ```rust
/// let x = 5;
/// let y = 6;
/// assert_eq!(x + y, 11);
/// ```
```

These snippets will be displayed in the documentation as code blocks with the Rust syntax highlighting.

## Linting and formatting

Rust provides tools to lint and format your code. The linter is called `clippy` and the formatter is called `rustfmt`.

To install them, run the following commands:

```bash
rustup component add clippy
rustup component add rustfmt
```

To run them, run the following commands:

```bash
cargo clippy
cargo fmt
```

To use custom rules for `clippy`, create a `.clippy.toml` file in the root folder of your crate. Rules can also be added
to the top of the `main.rs` (or `lib.rs`) file with the `#![warn(clippy::rule_name)]` syntax.

```rust
#![warn(clippy::all)]
```
