# Patterns and Matching

Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple. A pattern consists of some combination of the following:

- Literals;
- Destructured arrays, enums, structs or tuples;
- Variables;
- Widlcards;
- Placeholders

These components describe the shape of the data we're working with, which we then match against values to determine wether our program has the correct data to continue running a particular piece of code.

-----------------------------------

## Places Patterns can be used

### `match` Arms

```rust
match VALUE {
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
}
```

A particular pattern `_` will match anything, but it never binds to a variable, so it's often used in the last match arm. This can be uself when you want to ignore any value not specified, for example.

### Mixing `if let` expressions

```rust
fn main() {
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
    // notice we can shadow the variable
    // we couldn't use conditionals here though
    // only on the next scope(curly brackets)
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the background color");
    }
  } else {
    println!("Using blue as the background color");
  }
}
```

The downside of `if let` expressions is that they are not exhaustive like the `match` expressions. If we miss a handling in some case, the compiler would not alert us to the possible logic bug.

### `while let` conditional loops

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
  println!("{}", top);
}
```

### Function parameters

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
  println!("Current location: ({}, {})", x, y);
}

fn main() {
  let point = (3, 5);
  print_coordinates(&point);
}
```

## Refutability

Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable. An example would be `x` in the statement `let x = 5;` because `x` matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable. An example would be `Some(x)` in the expression `if let Some(x) = a_value` because if the value in the `a_value` variable is `None` rather than `Some`, the `Some(x)` pattern will not match.

## Matching

- **Literals**

```rust
let x = 1;

match x {
  1 => println!("one"),
  2 => println!("two"),
  3 => println!("three"),
  _ => println!("anything"),
}
```

- **Named variables**

Named Variables are irrefutable patterns that match any value. Variables declared as part of a patterns inside the `match` expression will shadow those with the same name outside the `match` construct, as is the case with all variables

```rust
fn main() {
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

- **Multiple Patterns**

```rust
let x = 1;

match x {
  1 | 2 => println!("one or two"),
  3 => println!("three"),
  _ => println!("anything"),
}
```

- **Range of Values**

```rust
let x = 5;

match x {
  1 ... 5 => println!("one through five"),
  _ => println!("something else"),
}

// chars are also valid ranges

let x = 'c';

match x {
  'a' ... 'j' => println!("early ASCII letter"),
  'k' ... 'z' => println!("late ASCII letter"),
  _ => println!("something else"),
}
```

- **Destructuring Structs**

```rust
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let p = Point { x: 0, y: 7 };

  // too verbose and very common for you
  // to want to declare with the same
  // fields of the struct
  let Point { x: a, y: b } = p;
  assert_eq!(0, a);
  assert_eq!(7, b);

  // this is also valid an better
  // only using the same fields
  // of the struct
  let Point { x, y } = p;

  // match that separates values
  match p {
    // points that lie directly on x(when y = 0)
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    // points that lie directly on y(when x = 0)
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }
}
```

- **Destructuring Enums**

```rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

fn main() {
  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    },
    Message::Move { x, y } => {
      println!(
        "Move in the x direction {} and in the y direction {}",
        x,
        y
      );
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r,
        g,
        b
      )
    }
  }
}
```

- **Destructuring Nested Structs & Enums**

```rust
enum Color {
  Rgb(i32, i32, i32),
  Hsv(i32, i32, i32)
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(Color),
}

fn main() {
  let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r,
        g,
        b
      )
    },
    Message::ChangeColor(Color::Hsv(h, s, v)) => {
      println!(
        "Change the color to hue {}, saturation {}, and value {}",
        h,
        s,
        v
      )
    },
    _ => ()
  }
}
```

- **Destructuring References**

```rust
let points = vec![
  Point { x: 0, y: 0 },
  Point { x: 1, y: 5 },
  Point { x: 10, y: -3 },
];

let sum_of_squares: i32 = points
  .iter()
  .map(|&Point { x, y }| x * x + y * y)
  .sum();
```

- **Destructuring Structs and Tuples**

```rust
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

## Ignoring Values in a Pattern

- **An Entire Value**

```rust
// using _ as a wildcard pattern will match any value and NOT BIND to the value
fn foo(_: i32, y: i32) {
  println!("This code only uses the y parameter: {}", y);
}

fn main() {
  foo(3, 4);
}
```

- **Parts of a Value with a nested `_`**

```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
  (Some(_), Some(_)) => {
    println!("Can't overwrite an existing customized value");
  }
  _ => {
    setting_value = new_setting_value;
  }
}

println!("setting is {:?}", setting_value);


let numbers = (2, 4, 8, 16, 32);

match numbers {
  (first, _, third, _, fifth) => {
    println!("Some numbers: {}, {}, {}", first, third, fifth)
  },
}
```

- **Remaining Parts of a Value with `..`**

```rust
// Using with structs
struct Point {
  x: i32,
  y: i32,
  z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
  Point { x, .. } => println!("x is {}", x),
}

// Using with tuples
let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    },
  }
```

## Match Guards

```rust
let num = Some(4);

match num {
  Some(x) if x < 5 => println!("less than five: {}", x),
  Some(x) => println!("{}", x),
  None => (),
}
```

## `@` Bindings

The *at* operator(`@`) let us create a variable that holds a value at the same time we're testing that value to see whether it matches a pattern. The variable is also only usable within that one pattern.

```rust
enum Message {
  Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
  Message::Hello { id: id_variable @ 3...7 } => {
    println!("Found an id in range: {}", id_variable)
  },
  Message::Hello { id: 10...12 } => {
    println!("Found an id in another range")
  },
  Message::Hello { id } => {
    println!("Found some other id: {}", id)
  },
}
```