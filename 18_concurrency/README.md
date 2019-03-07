# Concurrency

*Concurrent programming*, where different parts of a programa execute independently, and *parallel programming* where different parts of a program execute at the same time, are becoming increasingly important as computers take advantage of their multiple processors.

Rust team discovered that the ownership and type systems are powerful set of tools to help manage memory safety *and* concurrency problems. Because of those tools, many concurrency errors are compile-time errors in Rust rather than runtime errors.

> on the book, they use concurrent for every situation instead of differentiate between parallel and concurrent programs

Higher level languages offer little control over concurrent problems but lower level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware.

Here we'll see:

- Creating threads to run multiple pieces of code at the same time
- *Message-passing* concurrency where channels send messages between threads
- *Shared-state* concurrency, where multiple threads have access to some piece of data
- The `Sync` and `Send` traits, which extende Rust's concurrency guarantees to user-defined types as well as types provided by the standard library

---------------------------------------------------------

## Threads to run code simultaneously

Executed program's code is run in a *process* and the OS manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run there independent parts are called *threads*.

Splitting computation in your program into multiple threads can improve performance because the program does multiple tasks at the same time. The problem with threads are:

- Race conditions, where threads are accessing data or resources in an inconsistent order
- Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
- Bugs that happen only in certain situations and are hard to reproduce and fix reliably