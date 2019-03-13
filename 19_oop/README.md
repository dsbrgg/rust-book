# Rust and OOP

OOP is a way of modeling programs. Objects came from Simula in 1960. Objects influenced [Alan Kay](https://en.wikipedia.org/wiki/Alan_Kay) programming architecture in which objects pass messages to each other by which he coined the term *object-oriented programming* in 1967 to describe this architecture. Rust fall into some definitions of OOP and some not. We'll explore how certain characteristics that are commonly considered OOP and how those can be translated into idiomatic Rust. We'll also see the trade-offs of doing OOP in Rust and implementing a solution using Rust's strengths.

--------------------------------

## Characteristics of OOP

Rust is influenced by many programming paradigms, including OOP. Arguably, OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance, let's see what they mean and wether Rust supports it.

### Objects Contain Data and Behavior

In the book *Design Patterns: Elements of Reusable Object-Oriented Software*, OOP is defined as:

```
Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.
```

By this definition, Rust is OO, `structs` and `enums` have data, and `impl` block provide methods on structs and enums. Even though they are not called objects, they provide the same functionality.

### Encapsulation that Hides Implementation Details

*Encapsulation* means that the implementation details of an object aren't accessible to code using that object. The only way to interact with an object is through its public API. Code using that object shouldn't be able to change the object's data or behaviour directly. This enables the programmer to change and refactor an object's internals without needing to change the code that uses the object.

In the following example we'll see how this is applicable to Rust by encapsulating the implementation details of the struct `AveragedCollection`. We can easily change aspects, such as the data structure, in the future. As long as the signatures of `add`, `remove` and `avarage` stay thesame, code using it wouldn't need to change.

```rust
fn main() {
  pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
  }

  impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
      self.list.push(value);
      self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
      let result = self.list.pop();
      match result {
        Some(value) => {
          self.update_average();
          Some(value)
        },
        None => None,
      }
    }

    pub fn average(&self) -> f64 {
      self.average
    }

    fn update_average(&mut self) {
      let total: i32 = self.list.iter().sum();
      self.average = total as f64 / self.list.len() as f64;
    }
  }
}
```

### Inheritance as a Type System and as Code Sharing

*Inheritance* is a mechanism where an object can inherit from another object's definition, thus gathering the parent object's data and behaviour without you having to define them again.

You choose inheritance for two main reasons: enabling you to reuse that implementation for a different type and to enable a child type to be used in the same places as the parent type(*polymorphism*).

> Polymorphism usually is considerd synonymous to inheritance but actually it's a more general concept that refers to code that can work with data of multiple types. Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is called: bounded parametric polymorphism.

Inheritance has been proven to not be that godo as it's often at risk of sharing more code than necessary. Inherited classes doesn't always need all the code the parent class provides and it also leaves to possible errors in case the a method is applied where it shouldn't to a subclass.

