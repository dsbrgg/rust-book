# Generic Types, Traits and Lifetimes

*Generics* are abstract stand-ins for concrete types or other properties. We can express the behaviour of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code,

*Traits* define behaviour in a generic way. You can combine traits with generic types to constrain a generic type to only those types that have a particular behaviour, as opposed to just any type.

*Lifetimes* are a variety of generics that give the compiler information about how references relate to each other. Lifetimes allow us to borrow values in many situations while still enabling the compiler to check that the references are valid.

## Generic Types

Using generics makes our code more flexible and provides more functionality to callers of our function while preventing code duplication:

Function implementation:

```rust
fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  // next function call to "largest" will give a bug because
  // largest implementation, although using generic type
  // it's relying on integer/float operations
  // but it shows how the function will accept any type

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);
}
```

In `struct`definitions:

```rust
struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };
}
```
