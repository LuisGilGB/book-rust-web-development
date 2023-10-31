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

To allow a client file of that module to consume it, the module must be referenced with the `mod` keyword:

```rust
// src/main.rs
mod my_module;
```
