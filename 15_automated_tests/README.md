# How to write tests

A test in Rust is a function that's annotated with the `test` attribute. **Attributes** are metadata about pieces of Rust code; one example is the `derive` attribute.

To run tests, use `cargo test`, Rust builds a test runner binary that runs the functions annotated with the `test` attribute and reports on whether each test function passes or fails.

> You can find tests examples here in the "adder" lib folder, run "cargo test" inside that folder!

## Controlling how tests run

`cargo run` compiles you code and then runs the resulting binary. `cargo test` compiles your code in test mode and runs the resulting binary. The default behaviour of the binary for tests is to run all the tests in parallel and capture output generated during test runs.

Some command line options go to `cargo test`, and some go to the resulting test binary. To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator `--` and then the ones that go to the test binary. Use `cargo test -- --help` to get options you can use after the separator.

## Running tests in parallel

When you run multiple tests, by default they run in parallel using threads. This means the tests will finish running faster so you can get feedback quicker on whether or not your code is working. Because the tests are running at the same time, make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.

If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the --test-threads flag and the number of threads you want to use to the test binary

```
// this will run with only one thread so, no parallelism
cargo test -- --test-threads=1
```

## Showing outputs in tests

By default, if a test passes, Rust’s test library captures anything printed to standard output. For example, if we call println! in a test and the test passes, we won’t see the println! output in the terminal; we’ll see only the line that indicates the test passed. If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.

If we want to see printed values for passing tests as well, we can disable the output capture behavior by using the --nocapture flag:

```
cargo test -- --nocapture
```

## Running tests by name

```
cargo test <function_name>
```

## Running multiple tests by name

```
// suppose you have two tests with the names: add_two and add_four
// following command will run both
cargo test add
```

## Running ignored tests

```
cargo test -- --ignored
```

## Rust tests organization

The Rust community thinks about tests in terms of two main categories: unit tests and integration tests. Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

### Unit Tests

The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected. You’ll put unit tests in the src directory in each file with the code that they’re testing. The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).

### The tests modules and `#[cfg(test)]`

`#[cfg(test)]` annotation tells Rust to compile and run the test code only when you can `cargo test`, not with `cargo build`. Since unit tests go in the same files as the code, you'll use `#[cfg(test)]` to specify that they shouldn't be included in the compiled result.

The attribute `cfg` stands for *configuration* and tells Rust that the following item should only be included given a certain configuration option.

### Testing Private Functions

```rust
pub fn add_two(a: i32) -> i32 {
  internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn internal() {
    assert_eq!(4, internal_adder(2, 2));
  }
}
```

Note that the `internal_adder` function is not marked as pub, but because tests are just Rust code and the `tests` module is just another module, you can bring `internal_adder` into a test’s scope and call it. If you don’t think private functions should be tested, there’s nothing in Rust that will compel you to do so.

### Integration Tests

In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API. Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. To create integration tests, you first need a tests directory.

### *tests* directory

Make a *tests* directory at the top level of the project, next to *src*. Cargo knows to look for integration test files in this directory.

You can still run independent tests within the tests folder:

```
cargo test --test <function_name>
```

Another convention is to separate tests within the tests folder with a `tests/common` folder. Inserting the files inside this *common* folder will let Rust know not to test that module was an integration test.

### Integration tests for binary crates

Only library crates(`lib.rs`) expose functions that other crates can use; binary crates(`main.rs`) are meant to be run on their own.