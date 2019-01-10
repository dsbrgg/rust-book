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

