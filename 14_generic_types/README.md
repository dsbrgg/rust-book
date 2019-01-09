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

Every reference has a *lifetime*, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate lifetimes when the lifetimes of references could be related in a few different ways.

The Rust compiler has a *borrow checker* that compares scopes to determine wether all borrows are valid.

Invalid borrow: 

```rust
{
  let r;                // ---------+-- 'a
                        //          |
  {                     //          |
      let x = 5;        // -+-- 'b  |
      r = &x;           //  |       |
  }                     // -+       |
                        //          |
  println!("r: {}", r); //          |
}                       // ---------+
```

Valid borrow:

```rust
{
  let x = 5;            // ----------+-- 'b
                        //           |
  let r = &x;           // --+-- 'a  |
                        //   |       |
  println!("r: {}", r); //   |       |
                        // --+       |
}      
```

## Generic Lifetimes in Functions

```rust
fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}
```

Note that the function take string slices, which are *references*, like that we can accept both slices of a `String` and string literals.

`longest` will throw an error though:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |                                 ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
signature does not say whether it is borrowed from `x` or `y`
```

We don't know the concrete lifetimes of the references that will be passed in, so we can't look at the scope as we did in the other example from before. We'll have to annotate it.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

On the `longest` function:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}
```

The constraint we're expressing here is that all the references in the parameters and the return value must have the same lifetime, naming it `'a`.

We're **not** changing the lifetimes of any values passed in or returned, rather we're specifying that the borrow checker should reject any values that don't adhere to these constraints.

When a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetiems of the parameters or return values on its own. The lifetimes might be different each time the function is called.

In the `longest` example, the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`. In other words, the generic lifetime `'a` will get the *concrete lifetime* that is equal to the smaller of the lifetimes of `x` and `y`.

Valid Example:

```rust
fn main() {
  let string1 = String::from("long string is long");

  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }
}
```

Invalid Example:

```rust
fn main() {
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {}", result);
}
```

The error: 

```
error[E0597]: `string2` does not live long enough
  --> src/main.rs:15:5
   |
14 |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
15 |     }
   |     ^ `string2` dropped here while still borrowed
16 |     println!("The longest string is {}", result);
17 | }
   | - borrowed value needs to live until here
```

As humans, we can look at this code and see that `string1`  is longer than `string2` and therefore `result` will contain the `string1` reference. However, the compile can't see that the reference is valid in this case. We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. 

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

## Lifetime Annotation in Structs

```rust
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.')
    .next()
    .expect("Could not find a '.'");
  let i = ImportantExcerpt { part: first_sentence };
}
```

## Lifetime Elision

The elision rules don’t provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be. In this case, instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations that specify how the references relate to each other.

Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.

These rules apply to fn definitions, as well as impl blocks.

The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.

The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.

The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

First rule and second rule applies here:

```rust
fn first_word(s: &str) -> &str {}
// ==
fn first_word<'a>(s: &'a str) -> &str {}
// == 
fn first_word<'a>(s: &'a str) -> &'a str {
```

First rule applies:

```rust
fn longest(x: &str, y: &str) -> &str {}
// above function has two params, so each param is given a specific lifetime by the compiler
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
```


## Lifetime Annotations in Method Definitions

```rust
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
    3
  }
}
```

Third elision rule applies:

```rust
impl<'a> ImportantExcerpt<'a> {
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}
```

## Static lifetime

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the binary of your program, which is always available. Therefore, the lifetime of all string literals is `'static`.

## Implementing generics, trais and lifetimes together

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
  where T: Display
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```