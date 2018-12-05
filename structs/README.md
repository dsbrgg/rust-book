# Using Structs to Structure Data

Here we will compare and contrast tuples with structs, demonstrate how to use structs
and discuss how to define methods and associated function to specify behaviour associated
with a struct's data. Structs and enums are the building blocks for creating new types
in your program's domain to take full advantage of Rust's compile type checking.

Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean.
As a result of these names, structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
```

Struct definition is like a general template for the type and instances fill in that template with particular data to create values of the type.

```rust
let use1 = User {
  email: String::from("some@email.com"),
  username: String::from("some_username"),
  active: true,
  sign_in_count: 1,
}
```

Modifying a struct's property with dot notation(note that the entire struct must be mutable):

```rust
let mut use1 = User {
  email: String::from("some@email.com"),
  username: String::from("some_username"),
  active: true,
  sign_in_count: 1,
}

user1.email = String::from("anotheremail@example.com");
```

