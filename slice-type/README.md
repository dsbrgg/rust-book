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