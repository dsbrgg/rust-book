# Slice Type

Slices does not have ownership, it lets you reference a contiguous sequence of elements
in a collection rather than a whole collection.

## String Slices

A *string slice* is a reference to part of a `String`. We use the `start..end` syntax, which
is the range that begins at `start` and continues up to, but not including `end`.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

If you want to include `end` on the `start..end` syntax:

```rust
let s = String::from("hello world");

let hello = &s[0..=4];
let world = &s[6..=10];
```

If you want to start at the beginning or the end of the string implicitly:

```rust
let s = String::from("hello");

let slice = &s[..2];
let slice = &s[2..];
```

If you want to slice the entire string:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[..];
let slice = &s[0..len];
```

> Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. For the purposes of introducing string slices, we are assuming ASCII only in this section.

## String Literals are Slices

```rust
let s = "Hello, world!";
```

The type `s` here is a `&str`: it's a slice pointing to a specific point of the binary. This is
why string literals are immutable as well; `&str` is an immutable reference.

## String Slices as Parameters

Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:

```rust
fn first_word(s: &str) -> &str {
```

Implementation:

```rust
fn main() {
  let my_string = String::from("hello world");
  // frist_word works on slices of `Strings`
  let word = first_word(&my_string[..]);

  let my_string_literal = "hello world";
  // first_word works on slices of string literals
  let word = first_word(&my_string_literal[..]);

  // because string lietrals *are* string slices already,
  // this works too, without the slice syntax!
  let word = first_word(my_string_literal);
}
```

## Other Slices

Arrays:

```rust
let a = [ 1, 2, 3, 4, 5 ];
let slice = &a[1..3];
```

The above slice has type `&[i32]`

## Sumary

Concepts of ownership, borrowing and slices ensure memory safety in Rust programs at compile time.
Rust gives you control over memory usage the same way as other systems languages, but having the
owner of the data automatically clean up that data when the owner does out of scope, means you don't
have to write and debug extra code to get this control.