# Example Program using Structs

- Run `cargo run`

## Method syntax

Methods are different from functions in that they're defined withinthe context of a struct
(or a enum or a trait object), and their first parameter is always `self` , which represents 
the instance of the struct the method is being called on.

To define a method we start the struct context the method will be in. We start with a 
`impl` keyword following with the struct's name, then we continue with the `fn` keyword
following with the method's name.

Note that the `self` parameter is represented as a reference (`&self`). Methods can take
ownership of `self`, borrow `self` immutably as we've done on the program or borrow `self` mutably just as any other parameter. For reading the data only of the struct, `&self` is the best option, if we were to change something on the struct's instance then `&mut self` would be used. Using `self` only on the parameter is rare and is a technique usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.

## Where's the `->` Operator

C and C++ have two operators for calling methods: `.` if you're calling a method on the object directly and `->` if you're calling the method on a pointer to the object and need to dereference the pointer first. In other words, it `object` is a pointer: `object->something()` is similar to `(*object).something()`.

Rust doesn't have and equivalent of `->` because it has a feature called *automatic referencing and derefencing*.

When calling `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of the method. The following are the same:

```rust
p1.distance(&p2); == (&p1).distance(&p2);
```

The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of `self`. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.


## Associated Functions

Functions that **don't** take `self` as a parameter are called *associated functions* because
they're associated with the struct; they're still functions, not methods, because they don't 
have an instance of the struct to work with. `String::from` is an associated function for example.

They are often used for constructors that will return a new instance of the struct.

```rust
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}
```

Associated functions use the `::` syntax with the struct name. Following the example above:
`let sq = Rectangle::square(3);`. This syntax is used both for associated functions and namespaces created by modules.

## Multiple `impl` Blocks

It's possible to define methods and associated functions within multiple `impl` blocks. But for now(this chapter of the book), they are not very useful but still valid syntax.