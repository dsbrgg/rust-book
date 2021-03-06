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

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

Code above returns the following error:

```
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
```

When we have a value of type like `i8` is Rust, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value.
Only when we have an `Option<T>` de we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

You have to convert `Option<T>` into a `T` before you can perform `T` operations. This helps
catching one common issue with null: assuming that something ins't null when it actually is.

In order to use an `Option<T>` value, you want to have code that will handle each variant.
You want some code that will run only when you have a `Some(T)` value, and this code is allowed to use the inner `T`. You want some other code to run if you have a `None` value, and that code doesn't have a `T` value available. The `match` expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

## `match` Control Flow Operator

`match` allows to compare a value against a series of patterns and execute code based on these.
It's like a coin-sorting machine: coins slide down a track with variously holes along it, and each coin falls through the first hole it encounters that fits into.

When a `match` pattern expression executes, it compares the resulting value agaisnt the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. The code within each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire `match` expression.

## Matching with `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## Matches are Exhaustive

With `match` we must exhaust every last possibility in order for the code to be valid. Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the `None` case, it protects us from assuming that we have a value when we might have null.

## The `_` Placeholder

The `_` will match any value. By putting it on a `match` it will cover all possible cases that aren't specified before it. The `()` is just the unit value, so nothing will happen in the `_`case.

Example: 

```rust
let some_u8_value = 0u8;

match some_u8_value {
  1 => println!("one"),
  3 => println!("three"),
  5 => println!("five"),
  7 => println!("seven"),
  _ => (),
}
```

## Concise control flow with `if let`

Example of `match` expression that is too verbose:

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
  Some(3) => println!("three"),
  _ => (),
}
```

The example above is just matching in case of a single variant, all others will execute the default which is nothing in this case. It can be more concise with the following expression:

```rust
if let Some(3) = some_u8_value {
  println!("three");
}
```

The `=` above is **not** an *assigment* but a comparison that works just like the `match` expression. However, you lose the exhaustive checking that `match` enforces. Consider if gaining conciseness is an appropriate trade-off for losing exhaustive checking.

## Summary

Creating custom types to use in your aPI ensures type safety: the compiler will make certain your functions get only values of the type each function expects.