# Packages, Crates and Modules

Here you will see examples of:

- *Packages*; **Cargo** feature that let you build, test and share crates.
- *Crates*; tree of modules that produce a library or executable.
- *Modules*; the `use` keyword that let you control the scope and privacy of paths.
- *path* is the naming of an item sucha as a struct, function or module.

## Packages and Crates

- A *crate* is a binary or library.
- The *crate root* is a source file that is used to know how to build a crate.
- A *package* has a *Cargo.toml* that describes how to build one or more crates.

When typing `cargo new <name>` we're crating a package.

Cargo convention is taht if you have a `src` folder containing a `main.rs` file, Cargo knows this pacakge contains a binary crate with the same name as the package and `src/main.rs` is its crate root. The same goes for `src/lib.rs`.

A package can contain zero or one library crates and as many binary crates as you'd like. But there must be at least one crate(either lib or bin) in a package.

A package containing both `src/main.rs` and `src/lib.rs`, has two crates: a library and a binary, both with the same name. The package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate crate.

## Module System

- Modules, a way to organize code and control the privacy of paths;
- Paths, a way to name items;
- `use` a keyword to bring a path into scope:
- `pub` a keyword to make items public;
- Renaming items when bringing them into scope with the `as` keyword;
- Using external packages;
- Nested paths to clean up large `use` lists;
- Using the glob operator to bring everything in a module into scope:
- How to split modules into individual files;

```rust
mod sound {
  mod instrument {
    mod woodwind {
      fn clarinet() {}
    }
  }

  mod voice {}
}

fn main() {}
```

`src/main.rs` and `src/lib.rs` are called crate roots because the contents of either of these two files form a module named `crate` at the root of the crate's module tree. Following the example above we would have the following hierarchy:

```
crate
  -> sound
    -> instrument
      -> woodwind
    -> voice
```

The entire module is rooted under the implicit module named `crate`.


## Paths for Referring Items in Module Tree

A *path* can take two forms:

- An *absolute path* starts from a crate root by using a crate name or a literal `crate`.
- A *relative path* starts from the current module and uses `self`, `super` or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separates by double colons(`::`).

```rust
mod sound {
  mod instrument {
    fn clarinet() {}
  }
}

fn main() {
  // Absolute path
  crate::sound::instrument::clarinet();

  // Relative path
  sound::intrument::clarinet();
}
```

## Modules as the Privacy Boundary

Modules are the *privacy boundary* in Rust. If you want to make an item like a function or a struct private, you put it in a module. Here are the rules:

- All items are private by default.
- You can use the `pub` keyword to make an item public.
- You aren't allowed to use private code defined in module that are children of the current module.
- You are allowed to use any code defined in ancestor modules or the current module.

```rust
mod sound {
  pub mod instrument {
    pub fn clarinet() {} // making the module public will not make its contents public
  }
}

fn main() {
  // Absolute path
  crate::sound::instrument::clarinet();

  // Relative path
  sound::intrument::clarinet();
}
```

## Starting Relative Paths with `super`

```rust
mod instrument {
  fn clarinet() {
    super::breathe_in();
  }
}

fn breathe_in() {}
```

The `clarinet` function is in the `instrument` module, so we can use `super` to go to the parent module of `instrument`, which in this case is `crate`, the root. From there, we look for `breathe_in`, and find it.

The reason for using `super` rather than an absolute path with `crate` is that using `super` is more flexible if you update your code to have a different module hierarchy, for example:

```rust
mod sound {
  mod instrument {
    fn clarinet() {
      super::breathe_in();
    }
  }

  fn breathe_in() {}
}
```

## Using `pub` with Structs and Enums

Note: On the following example it is seen that because `plant::Vegetable` has a private field, the struct needs to provide a public associated function that constructs an instance of `Vegetable`(used conventional `new` here).

```rust
mod plant {
  pub struct Vegetable {
    pub name: String,
    id: i32,
  }

  impl Vegetable {
    pub fn new(name: &str)  -> Vegetable {
      name: String::from(name),
      id: 1,
    }
  }
}

fn main() {
  let mut v = plant::Vegetable::new("squash");

  v.name = String::from("butternut squash");
  println!("{} are delicious", v.name);

  // Next line won't compile if uncommented:
  // println!("The ID is {}", v.id);
}
```

Since the enum is set as `pub`, all its variants are accessible.

```rust
mod menu {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

fn main() {
  let order1 = menu::Appetizer::Soup;
  let order2 = menu::Appetizer::Salad;
}
```

## `use` keyword to Bring Paths into a Scope

Adding a `use` path in a scope is similar to creating a symbolic link in a filesystem.

```rust
mod sound {
  pub mod instrument {
    pub fn clarinet() {}
  }
}

use crate::sound::instrument;

fn main() {
  instrument::clarinet();
  instrument::clarinet();
  instrument::clarinet();
}
```

If you want to bring an item into scope with a relative path, instead of starting from a name in the current scope, you must start the path with `self`.

Using absolute paths make updates easier if the code calling the items moves to a different place in the module tree but the code defining the items does not.

```rust
mod sound {
  pub mod instrument {
    pub fn clarinet() {
      // Function body code goes here
    }
  }
}

mod performance_group {
  use crate::sound::instrument;

  pub fn clarinet_trio() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
  }
}

fn main() {
  performance_group::clarinet_trio();
}
```

## Idiomatic `use` Paths for Functions vs. Other Items

For functions, it's considered idiomatic to specify function's parent module with `use`. It makes it more clear that the function isn't locally defined while minimizing repetition.

```rust
mod sound {
  pub mod instrument {
    pub fn clarinet() {
      // Function body code goes here
    }
  }
}

use crate::sound::instrument::clarinet;

fn main() {
  // NOT IDIOMATIC
  clarinet();
  clarinet();
  clarinet();
}
```

For structs, enums and otehr items, it's the opposite, for example the `HashMap` struct:

```rust
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);
}
```

The only exceptions for this is if the `use` statements would bring two items with the name name into scope, which isn't allowed:

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {}
fn function2() -> io::Result<()> {}
```

## Renaming Types Brought Into Scope

Refactoring the example above: 

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}
```

## Re-exporting Names with `pub use`

```rust
mod sound {
  pub mod instrument {
    pub fn clarinet() {
      // Function body code goes here
    }
  }
}

mod performance_group {
  pub use crate::sound::instrument;

  pub fn clarinet_trio() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
  }
}

fn main() {
  performance_group::clarinet_trio();
  performance_group::instrument::clarinet();
}
```

## Nested Paths

```rust
// Different use keywords
use std::cmp::Ordering;
use std::io;

// Nested
use std::{cmp::Ordering, io};

// Duplicate paths
use std::io;
use std::io::Write;

// DEduplicate paths
use std::io::{self, Write};

// Brings all public definitions into scope(glob operator)
use std::collections::*;
```

## Separating modules into Different Files

Check `src/main.rs` and `src/sound.rs`!

```rust
mod sound;
```

By using `mod sound` with a semicolon, we tell Rust to load the contents of the module from another file with the **same name as the module**.

