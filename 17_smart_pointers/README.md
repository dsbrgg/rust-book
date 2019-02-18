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

## `Deref` Trait

`Deref`trait allows you to customize the behaviour of the *dereference operator* (`*`). By using it, you can make a smart pointer be treated liek a regular reference, you can write code that operates on references and use that code with smart pointers too.

```rust
fn main() {
  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  // to make this assertion possible
  // since "y" is a reference (an address to the actual value) to x
  // we have to use "*y" to follow the reference to the value
  // it's pointing to (hence dereference)
  assert_eq!(5, *y);

  // you can also do this same example with a Box

  let w = 6;
  let z = Box::new(x);

  assert_eq!(6, w);
  // since box just allocates data in the heap
  // it works much like a reference
  // it's a smart pointer with the Deref trait
  assert_eq!(6, *z);
}
```

## Defining a Smart Pointer with `Deref` Trait

The `Box<T>` type is ultimately defined as a *tuple struct* with one element.

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
```

For now `MyBox<T>` cannot be dereferenced since it's just a simple tuple struct. In order for us to have the same behaviour as the `Box<T>`, we still have to implement `Deref`.

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
  // associated types are a slightly different way
  // to declare a generic parameter
  type Target = T;

  // with this, when trying to access the value from MyBox
  // it will actually return a reference to the value we
  // want to access with the * operator
  fn deref(&self) -> &T {
    &self.0
  }
}
```

Now, making the same assertion as before but with our `MyBox<T>` compiles!

```rust
fn main() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  // behind the scenes, Rust is calling: *(y.deref())
  assert_eq!(5, *y);
}
```

## Deref Coercion

*Deref coercion* makes it easy to pass arguments to function and methods and get it's actual value by automatically dereferencing(in case the type has `Deref` implemented).  It happens automatically when we pass a reference to a particular type's value as an argument to a function or method that doesn't match the parameter type in the function or method definition.

Example:

```rust
fn hello(name: &str) {
  println!("Hello, {}!", name);
}

fn main() {
  let m = MyBox::new(String::from("Rust"));
  // this will deref coerce until it gets to the MyBox deref method
  // which would return the actual value as a reference (&String)
  // which would eventually call the deref from String
  // since a String is a smart pointer as well
  hello(&m);
}
```

## Deref Coercion and Mutability

You can use `Deref` trait to override the `*` operator on **immutable references**, you can use `DerefMut` instead for **mutable references**.

Rust does deref coercion when it finds types and trait implementations in three cases:

- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile). Converting one mutable reference to one immutable reference will never break the borrowing rules. Converting an immutable reference to a mutable reference would require that there is only one immutable reference to that data, and the borrowing rules don’t guarantee that. Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.

## `Drop` Trait

The `Drop` trait let you specify what happens when a value is about to go out of scope. In some langauges, the programmer must call code to free memory or resources every time they finish using and instance of a smart pointer. If they forget, the system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically. As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources!

The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to `self`.

```rust
struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data: {}", self.data);
  }
}

fn main() {
  let c = CustomSmartPointer { data: String::from("my stuff") };
  let d = CustomSmartPointer { data: String::from("other stuff") };
  println!("CustomSmartPointers created");
}
```

## Dropping with `std::mem::drop`

Rust won't let you call the `Drop` trait's `drop` method manually; instead you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.

Rust doesn't let us call `drop` explicitly because Rust would still automatically call `drop` on the value at the end of `main`. This would be a *double free* error because Rust would be trying to clean up the same value twice.

```rust
fn main() {
  let c = CustomSmartPointer { data: String::from("some data") };

  println!("CustomSmartPointer created.");

  drop(c);

  println!("CustomSmartPointer dropped before the end of main.");
}
```

## `Rc<T>`, Reference Counted Smart Pointer

Ownership is clear: you know exactly which variable owns a given value. However, there are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the same node and the node is conceptually owned by all of the edges that point to it. A node shouldn't be cleaned up unless it doesn't have any edges pointing to it.

`Rc<T>` which is an abbreviation for *reference counting* enables multiple ownership. It keeps track of the number of references to a value which determines or not a value is still in use.

> Analogy: Imagine `Rc<T>` as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

Let's reimplement our *cons list* example with `Rc<T>`

```rust
enum List {
  Cons(i32, Rc<List>),
  Nil,
}

use List::{Cons,Nil};
use std::rc::Rc;

fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  let b = Cons(3, Rc::clone(&a));
  let c = Cons(4, Rc::clone(&a));
}
```

The implementation of `Rc::clone` doesn’t make a deep copy of all the data like most types’ implementations of clone do. The call to `Rc::clone` only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time. By using `Rc::clone` for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count. When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to `Rc::clone`.

## `RefCell<T>` and Interior Mutability Pattern

*Interior mutability* is a design pattern in Rust that allows you to mutate data even when tehre are immutable references to that data; normally this action is not allowed by the *borrowing rules*. The pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing. We can use types that use the interior mutability pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can't guarantee that. The `unsafe` code involved is then wrapped in a safe API, and the outer type is still immutable.

With references and `Box<T>`, the borrowing rules invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced *at runtime*. With references, if you break these rules, you'll get a compiler error. With `RefCell<T>`, if you break these rules, your program will panic and exit.

Checking borrowing rules at compile time will make errors be caught sooner in the development process, and there is no impact in runtime performance, this is Rust's default. 

Checking the borrowing rules at runtime can make certain memory-safe scenarios allowed whereas they wouldn't be because of the compile-time checks.

Static analisys, like the Rust compiler, is inherently conservative. Some properties of code are impossible to detect by analyzing the code.