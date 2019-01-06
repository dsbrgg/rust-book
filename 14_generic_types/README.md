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

## Performance using Generics

Rust implements generics in such a way that your code doesn't run any slower using generic types than it would with concrete types.

Rust uses *monomorphization*: a process of turning generic code into specific code by filling in the concrete types that are used when compiled. There are no runtime cost for using generics.

For example:

```rust
let integer = Some(5);
let float = Some(5.0);

// The monomorphized code would like like this
enum Option_i32 {
  Some(i32),
  None,
}

enum Option_f64 {
  Some(f64),
  None,
}

fn main() {
  let integer = Option_i32::Some(5);
  let float = Option_f64::Some(5.0);
}
```

## Traits: Defining Shared Behaviour

Traits are similar to *interfaces*, with some differences. They tell the Rust compiler about a functionality a particular type has an can share with other types.

Defining traits:

```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```

Implementing a trait:

```rust
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location);
  }
}

// or ---
pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.password);
  }
}

let tweet = Tweet {
  username: String::from("horse_ebooks"),
  content: String::from("of course, as you probably already know, people"),
  reply: false,
  retweet: false,
}

println!("1 new tweet: {}", tweet.summarize());
```

Notice that `Summary` is on the same scope as the structs. If this is supposed to be on a crate and someone else wants to use the functionality, they would have to bring the trait into their scope by specifying `use <name_of_the_crate>::Summary;`. The trait aalse needs to be public for another crate to implement it.

Another restriction to traits is that we can only implement a trai on a type if either the trait or the type is local to our crate. This restriction is parrt of a property of programs called *coherence* and more specifically the *orphan rule*, named because the parent type is not present. The rule ensures that other people's code can't break your code and vice versa. Without the rul, two crates could implement the same trait for the same type and Rust wouldn't know which implementation to use.

## Default Implementations

Following the example above, we could have a default implementation for our `Summary` trait:

```rust
pub trait Summary {
  fn summarize_author(&self) -> String;

  // we can also use methods from the trait within itself
  fn summarize(&self) -> String {
    String::from("(Read more from {}...)", self.summarize_author())
  }
}

// then, we can use it for NewsArticle
impl Summary for NewsArticle {}

// overriding the trait method
impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

let article = NewsArticle {
  headline: String::from("Penguins are ruling the world"),
  location: String::from("Santos, SP, Brazil"),
  author: String::from("Chorão"),
  content: String::from("Penguins are very skate"),
};

println!("New article available! {}", article.summarize());

let tweet = Tweet {
  username: String::from("horse_ebooks"),
  content: String::from("of course, as you probably already know, people"),
  reply: false,
  retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

## Traits as arguments

```rust
pub fn notify(item: impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```

## Trait Bounds

`impl Trait` is syntax sugar for a longer form. This is called a *trait bound*:

```rust
pub fn notify<T: Summary>(item: T) {
  println!("Breaking news! {}", item.summarize());
}
```

Because of the trait bound on `T`, code that calls the function with any other type, like a `String` or an `i32`, won't compile, because those types don't implement `Summary`.

Multiple traits with `+`:

```rust
pub fn notify(item: impl Summary + Display) {}
pub fn notify<T: Summary + Display>(item: T) {}
```

## `where` clauses

Instead of write this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
```

We can use `where` clauses:

```rust
fn some_function<T, U>(t: T, u: U) -> i32 
  where T: Display + Debug,
        U: Clone + Debug
{
  // fn body !!
}
```

Returning Traits:

```rust
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}
```

## Trait Bounds to conditionally implement methods

```rust
use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {
      x, y
    }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

// implement a trait for any type that implements another trait
impl<T: Display> ToString for T {
  // --snip--
}
```

In dynamically typed languages, we would get an error at runtime if we called a method on a type that the type didn’t implement. But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.

## Lifetimes
