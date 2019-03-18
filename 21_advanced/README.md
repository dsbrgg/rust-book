# Advanced Features

Here we'll cover:

- Unsafe Rust; opting out of some of Rust's guarantees
- Advanced lifetimes; syntax for complex lifetime situations
- Advanced traits; associated types, default type parameters, fully qualified syntax, supertraits and the newtype pattern in relation to traits
- Advanced types; more about the newtype pattern, type aliases, the never type, dynamically sized types
- Advanced functions and closures; function pointers and returning closures
- Macros; ways to define code taht defines more code at compile time

------------------

## Unsafe

Rust has a second language hidden inside it that doesn't enforce these memory safety guarantees, it's called *unsafe* Rust.

Unsafe Rust exists because, by nature, static analysis is conservative. When the compiler tries to determine whether or not code upholds the guarantees, it's better for it to reject some valid programs rather than accept some invalid programs. You can use unsafe code to tell the compiler "Trust me, I know what I'm doing". If you use unsafe code inconrrectly, problems due to memory unsafety, such as null pointer dereferencing, can occur.

Another reason Rust has an unsafe language inside of it is because underlying computer hardware is inherently unsafe. If Rust didn't let you do unsafe operations, you couldn't do certain tasks. Rust needs to allow you to do low-level systems programming, such as directly interacting with the operating system or even writing your own operating system. Working with low-level systems programming is one of the goals of the language.

You can take four actions in unsafe Rust, called *unsafe superpowers*, that you can't in safe Rust. These include:

- Dereference a raw pointer;
- Call an unsafe function or method;
- Access or modify a mutable static variable;
- Implement an unsafe trait;

`unsafe` doesn't turn off the borrow checker or disable any other of Rust's safety checks: if you use a reference in unsafe code, it will still be checked. The `unsafe` keyword only gives you access to these for features that are then not checked by the compiler for memory safety. You'll still get some degree of safety inside of an unsafe block.

`unsafe` also does not mean the code inside is necessarily dangerous or that it will have memory safety problems; as a programmer you'll ensure the code inside an `unsafe` block will access memory in a valid way.

With `unsafe`, you'll know that any errors related to memory safety must be within an `unsafe` block. Keep `unsafe` blocks small.

Isolate unsafe code as much as possible, it's best to enclose unsafe code within a safe abstraction and provide a safe API. Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited. Wrapping unsafe code in a safe abstraction prevents uses of the `unsafe` from leaking out into all the places that you or your users might want to use the functionality implemented with `unsafe` code, because using a safe abstraction is safe.