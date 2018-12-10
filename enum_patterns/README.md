# Enums and Pattern Matching

Here we'll look into:

- Enums
- `Option` enum
- `match` expression
- `if let` construct

Enums in Rust are most similar to *algebraic data types* in functional languages such as F#, OCaml and Haskell.

## Enum Values

*Variants* of the enum are namespaced under its identifier, and we use a double colon to 
separate the two. The reason this is useful is that now all value end up having the same type.
Now it's possible to define functions that takes that `enum` type.

It's possible to associate data types to the enum as well (following the example from `src/main.rs`):

```rust
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
```

Now the *variant* from the enum has an associated *value* with it.

There's also other advantage for using an enum rather than a struct: each variant can have different types and amounts of associated data. Following the example above, an IP address from version 4 has four numeric components that range from 0 to 255. So if we want to store `V4` addresses as for `u8` and still express `V6` as a `String`, enums can handle that still.

```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String), 
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

It's worth noting you can put any kind of data inside an enum. To illustrate:

```rust
enum Message {
  Quit,  // no data associated at all
  Move { x: i32, y: i32 }, // anonymous struct
  Write(String), // include single String
  ChangeColor(i32, i32, i32), // includes three i32 values
}
```

Using the example above shows how you can use an enum instead of defining different `struct` to structure data. In other words, the `enum` above would be declared as such:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
  x: i32,
  y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

`impl` can be used for defining methods both on `struct` and `enum`:

```rust
impl Message {
  fn call(&self) {
    // body...
  }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## `Option` Enum and its advantages over null values

The `Option` type is used a lot because it encodes the very common scenario in which a value could be something or it could be nothing.

Rust doesn't have the *Null* feature that many other languages have. *Null* is a value that means there is no value.

> Trivia: Tony Hoare, the inventor of null, said that implementing a null reference was a big mistake.

The problem with null values is that if you try to use a null value as a not-null value, you'll get an error of some kind.

However the concept that null is trying to express is still a useful one: null is a value that is currently invalid or absent for some reason.

Hence `Options`:

```rust
enum Option<T> {
  Some(T),
  None,
}
```

`Option<T>` is so useful you don't even have to include into scope. You can also use `Some` and `None` directly without the `Option::` prefix.

The `<T>` syntax is a feature of Rust called a genery type parameter. It will be covered later.

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

If `None` is used, we need to tell Rust what type of `Option<T>` we have. The compiler can't infer the type that the `Some` variant will hold by looking at a `None` value.

