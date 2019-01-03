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

Using more than one generic type to support different generics simultaneously:

```rust
struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let both_integer = Point { x: 5, y: 10 };
  let both_float = Point { x: 1.0, y: 4.0 };
  let integer_and_float = Point { x: 1.0, y: 4.0 };
}
```

In `enum` definitions:

```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

In method definitions, note that you have to declare `T` just after `impl` so we can use it to specify that we're implementing methods on the type `Point<T>`. By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type.

```rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

fn main() {
  let p = Point { x: 5, y: 10 };

  println!("p.x = {}", p.x());
}
```

It's possible to implement methods only on `Point<f32>`(or whatever other type) instances rather than on `Point<T>` isntances with any generic type.

```rust
impl Point<i32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```

Generic type parameters in a struct definition aren't always the same as those you use in that struct's method signature:

```rust
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```