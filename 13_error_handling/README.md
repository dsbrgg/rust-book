# Error Handling

Rust groups errors in two major categories: *recoverable* and *unrecoverable*. Recoverable errors(like a file not found) is better to be reported and the operation to be retried; unrecoverable errors are symptons from bugs, like trying to access an index beyond the an array limits.

Rust doesn't have exceptions, instead it has a type `Result<T, E>` for recovarable errors and the `panic!` macro that stops execution whe the program encounters an unrecoverable error.

## Unrecoverable errors with `panic!`

`panic!` will print a failure message, unwind and clean up the stack, then quit.

> Unwinding means Rust walks back up the stack and cleans up the data from each function it encounters. This requires a lot of work. You can set it to immediately abort by setting it on the Cargo.toml under the [profile] section

```rust
fn main() {
  let v = vec![1, 2, 3];

  // BUG !!!
  v[99];
}
```

Other languages, like C, would attempt to give you exactly what you asked for in this situation, even though it isn't what you want. You'll get whatever is at the location in memory that would correspond to that element in the vector, even though the memory doesn't belong to the vector. This is called *buffer overread* and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn't be allowed to that is stored after the array.

Rust will prevent this sort of vulnerability:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', /checkout/src/liballoc/vec.rs:1555:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

You can also set `RUST_BACKTRACE` env variable to get a more verbose output from `panic!` macro:

```
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /checkout/src/liballoc/vec.rs:1555:10
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic_bounds_check
             at /checkout/src/libcore/panicking.rs:58
  10: <alloc::vec::Vec<T> as core::ops::index::Index<usize>>::index
             at /checkout/src/liballoc/vec.rs:1555
  11: panic::main
             at src/main.rs:4
  12: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:99
  13: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:459
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:61
  14: main
  15: __libc_start_main
  16: <unknown>
```

## Recoverable Errors with `Result` enum

```rust
enum Result <T, E> {
  Ok(T),
  Err(E),
}
```

`T`represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant.

We'll look into an example where we can explore this functionality:

```rust
use std::fs:File;

fn main() {
  let f = File::open("hello.txt");
}
```

We can know that `File::open` will return a `Result` by peeking into the documentation or asking the compiler: giving a type to a variable that we know is not the return type from the function, will make the compiler let us know that the types don't match.

```
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
             found type `std::result::Result<std::fs::File, std::io::Error>`
```

Let's see how the `Result` enum can be handled with a `match` expression:

```rust
use std::fs:File;

fn main() {
  let f = File::open("hello.txt"):

  let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file {:?}", error)
    },
  }
}
```

Like the `Option` enum, The `Result` enum is brought to scope in the prelude, so we don't need to specify `Result::` before its variants.

Matching on Different Errors:

```rust
use std::fs::File;
use std::io:ErrorKind;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Tried to create file but there was a problem {:?}", e),
      },
      other_error => panic!("There was a problem opening the file {:?}", other_error),
    }
  }
}
```

Matching with different Errors using closures:

```rust
use std::fs:File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("hello.txt").map_err(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Tried to create file but there was a problem {:?}", error);
      })
    } else {
      panic!("There was a problem opening the file {:?}", error);
    }
  })
}
```

## Shortcutes for Panic on Error

If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro.

Other method is `expect` which is similar to `unwrap` but let us also choose the `panic!` error message.

```rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt").unwrap();
  let f = File::open("hello.txt").expect("Faile to open hello.txt");
}
```

Message from `unwrap`:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

Message from `expect`:

```
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

## Propagating errors

Instead of handling error within functions, you can return the error to the calling code so that it can decide what to do, this is known as propagating the error and gives more control to the calling code.

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}
```

## The `?` Operator

The `?` place after a `Result` value is defined to work in almost the same way as the `match` expressions we defined to handle the `Result` values.

Error values taken by `?` go through the `from` function, defined in the `From` trait in the standard library, which is used to convert errors from one type into another. When `?` calls the `from` function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the `from` function to define how to convert itself to the returned error type, `?` takes care of the conversion automatically.

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}
```

You can also chain method calls immediately after the `?` operator:

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}
```

In this specific example there's even a shorter way to do it:

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
```

Good note here is that `?` operator is supposed to work only with `Result` enums.

## To `panic!` or not to `panic!`

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

- The bad state is not something that’s expected to happen occasionally.
- Your code after this point needs to rely on not being in this bad state.
- There’s not a good way to encode this information in the types you use.