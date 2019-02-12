# Smart Pointers

Smart pointers, are data structures that not only act like a pointer but also have additional metadata and capabilities. The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist in other languages as well. In Rust, the different smart pointers defined in the standard library provide functionality beyond that provided by references. One example that we’ll explore in this chapter is the *reference counting* smart pointer type. This pointer enables you to have multiple owners of data by keeping track of the number of owners and, when no owners remain, cleaning up the data.

In Rust, which uses the concept of ownership and borrowing, an additional difference between references and smart pointers is that references are pointers that only borrow data; in contrast, in many cases, smart pointers **own** the data they point to.

`String` and `Vec<T>` both count as smart pointers because they own some memory and allow you to manipulate it. They also have metadata (such as their capacity) and extra capabilities or guarantees (such as with String ensuring its data will always be valid UTF-8).

Smart pointers are usually implemented using structs. The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers. The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.

## `Box<T>` to Point to data on the Heap

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size

- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so

- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

## Recursive Types with Boxes

At compile time, Rust needs to know how much space a type takes up. One type whose size can't be known at compile time is a *recursive type*, where a value can have as part of itself another value of the same type.

Boxes have a known size (pointer) so by inserting a box in a recursive type definition, you can have recursive types. A pointer's size doesn't change based on the amount of data it's pointing to.

*Cons List* example:

> A cons list is a data structure that comes from the Lisp programming language and its dialects. In Lisp, the cons function (short for “construct function”) constructs a new pair from its two arguments, which usually are a single value and another pair. These pairs containing pairs form a list.

```rust
// this would not compile, because
// Rust will never know how much space
// a List would take and adding it 
// as a property for one of the variants
// could theoretically go infinitely
enum List {
  Cons(i32, List),
  Nil,
}

// usage but won't compile
use List::{Cons, Nil};

fn main() {
  let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

Rust goes through each of the variants to see which variants needs more space. Because only one of the variants will be used, the most space a data structure will need is the space it would take to store the longest variant.

Help message from above code when trying to compile:

```
 = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

In this suggestion, “indirection” means that instead of storing a value directly, we’ll change the data structure to store the value indirectly by storing a pointer to the value instead.

In the next example, conceptually, we still have a list, created with lists “holding” other lists, but this implementation is now more like placing the items next to one another rather than inside one another.

```rust
enum List {
  Cons(i32, Box<List>),
  Nil
}

use List::{Cons,Nil};

fn main() {
  let list = Cons(1,
    Box::new(Cons(2,
      Box::new(Cons(3,
        Box::new(Nil)
      ))
    ))
  );
}
```

Boxes provides only the indirection and heap allocation, no other special capabilities. They also don't have any performance overhead that these special capabilities incur.

`Box<T>` is a smart pointer that implements the `Deref` trait which allows its values to be treated like references. When it goes out of scpe, the heap data that the box is pointing to is cleand up as well because of the `Drop` trait implementation.