# Implement a RESTful API

## Some notes on HashMap keys

In Rust, we can declare hash maps with the `HashMap` type from the standard library.

In principle, we can use any type as the key, but in practice, we need to meet some requirements for the key type. The
most important of these requirements is that the key must implement the `Eq` and `Hash` traits. The `Eq` trait indicates
that two values are equal to each other, and the `Hash` trait allows these values to be hashed.

The `Hash` trait is important because it allows the key to be used in a hash map. The `Hash` trait is also implemented
for Rust's primitive types, such as `u32` and `String`. The `Eq` trait is implemented for all types that implement
the `PartialEq` trait. The `PartialEq` trait is used to compare two values for equality, and it is implemented for all
primitive types by default.
