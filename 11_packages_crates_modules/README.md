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

