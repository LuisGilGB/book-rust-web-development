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
