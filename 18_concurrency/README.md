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

Many OS provide an API for creating threads. This model of using the OS API for creating threads is called *1:1*, meaning one operating system thread per one language thread.

Programming languages provide their own special implementation of thread. Programming language-provided threads are known as *green* threads, and languages that use these green threads will execute the, in the context of a different number of operating system threads. The green-threaded model is called the *M:N* model: there are `M` green threads pr `N` operating system threads, where `M` and `N` are not necessarily the same number.

Each model has its own advantages and trade-offs, and the trade-off most important to Rust is runtime support. *Runtime* is a confusing term and can have different meanings in different contexts.

In this context, by *runtime* we mean code that is included by the language in every binary. Every non-assembly language will have some amount of runtime code. Colloquially when people say a language has "no runtime", they often mean "small runtime". Smaller runtimes have fewer features but have the advantage of resulting in smaller binaries, which makes it easier to combine the language with other lanaguages in more contexts. Although many languages are okay with increasing the runtime size in exchange for more features, Rust needs to have nearly no runtime and cannot compromise on being able to call into C to maintain performance.

Green-threading M:N model requires a larger language runtime to manage threads. Rust provides natively only 1:1 threading but Rust is low-level enough that there are `crates` that implement `M:N` threading if you rather trade overhead for aspects such as more control over which threads run when and lower costs of context switching, for example.

## Message Passing to Transfer Data Between Threads

A popular approach to ensure safe concurrency is *message passing*, where threads or actors communicate by sending each other messages containing data.

One major toll Rust has for message-sending concurrency is a programming concept called *channel*. Imagine a channel of water, such a stream of a river. If you put something like a rubber duck or a boat into a stream, it will travel downstream to the end of the waterway.

A channel has two halves: a transmitter and a receiver. The transmitter is the upstream location where you put ruber ducks into the river, the receiver is where the rubber duck ends up downstream. A channel is said to be *closed* if either the transmitter or receiver is dropped.

> exercise ideas: chat system; system that perform calculations on different threads and sends to another to aggregate results

Ownership rules play a vital role in message sending because they help you write safe concurrent code. Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs.

Allowing to use a value after it's sent down to a *transmitter* is a bad idea: once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again. Other thread's modification could cause errors or unexpected results due to inconsistent or nonexistent data. Because of that, Rust won't allow it to happen and throw a compile error if this situation happens.
