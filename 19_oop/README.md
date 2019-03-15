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

Rust does not that have in a sense where, for example, structs and enums inherit from one another, so it wouldn't be considered OOP.

## Using Trait Objects that Allow for Values of Different Types

In Rust, a struct or enum, the data in the struct fields and the behavior in `impl` are separated, whereas in other languages, the data and behavior combined into one concept is often labeled an object. In this sense, *trait objects* are like objects in other languages in that they combine data and behavior but they also differ from traditional objects in that we can't add data to a trait object; their purpose is to allow abstraction across common behavior.

This works different than defining a struct that uses a generic type parameter with trait bounds. A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.

If you'll only ever have homogeneous collections, using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.

With the method using trait objects, one struct/enum instance can hold a `Vec`(for example) that contains a `Box<T>` with the type that has that trait.

This concept - of being concerned only with the messages a value responds to rather than the value's concrete type - is similar to the concept of *duck typing* in dynamically typed languages: if it walks like a duck and quacks like a duck, then it must be a duck. In the example below, we don't need to check if the components are instances of anything, it just calls the `draw` method. By specifying `Box<dyn Draw>`, we've defined `Screen` to need values that we can call the `draw` method on.

The advantage of using trait objects and Rust's type system to write code similar to using duck typing is that we never have to check wether a value implements a particular method at runtime or worrying about getting errors if a value doesn't implement a method but we call it anyway.

The code that results from monomorphization is doing *static dispatch*, which is when the compiler knows what method you're calling at compile time. This is opposed to *dynamic dispatch*, which is when the compiler can't tell at compile time which method you're calling. In dynamic dispatches, the compiler emits code that at runtime will figure out which method to call.

When using trait objects, Rust must use dynamic dispatch. The compiler doesn't know all the types that might be used with the code that is using trait objects, so it doesn't know which method implemented on which type to call. At runtime, Rust uses the pointers inside the trait object to know which method to call. There's a runtime cost when this lookup happens that doesn't occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turns prevents some optimizations. However, we did get extra flexibility in the code so it's a trade-off to consider.

```rust
pub trait Draw {
  fn draw(&self);
}

// using trait objects
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

// implementing the trait
pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String
}

impl Draw for Button {
  fn draw(&self) {
    // code here
  }
}

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // code to actually draw a select box
  }
}

fn main() {
  let screen = Screen {
    components: vec![
      Box::new(SelecBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ]
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK")
      })
    ]
  };

  screen.run();
}

// ---------------------------------------------

// using generics and trait bounds
// just for example purposes
pub struct Screen<T: Draw> {
  pub components: Vec<T>,
}

impl <T> Screen<T>
  where T: Draw {
    pub fn run(&self) {
      for component in self.components.iter() {
        component.draw();
      }
    }
  }
```