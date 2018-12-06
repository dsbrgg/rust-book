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

Using struct update syntax, we can achieve the same effect with less code. The syntax
`..` specifies the fields not explicitly set should have the same value as in the given
instance.

```rust
let user2 = User {
  email: String::from("blabla@bla.com"),
  username: String::from("blabla"),
  ..use1
};
```

## Tuple Structs

Tuple structs have the added meaning the struct name provides but don't have names associated
with fields; they just have the types of the fields. Notice that each struct is its own type.
Even if the types are composed by the same values, they are different entities.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Tuple structs behave like tuples: you can destructure them into individual pieces, you can use
`.` to point for its index to access individual values and so on.

## Unit-Like Structs Without Fields

You can define structs that don't have fields. They are called *unit-like structs* because
they behave similarly to `()`, the unit type. Useful to implement traits on some type you don't
have any data to store in the type itself.

## Structs and Ownership

In the examples above, we've used `String` type, instead of a `&str` reference, to make sure the structs instances would own their data and for it to be valid as long as the entire struct instance is also valid.

It's possible to store references to data owned by something else but to do so, it requires use of lifetimes.