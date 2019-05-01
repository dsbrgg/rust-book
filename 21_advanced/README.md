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

## Dereferencing a Raw Pointer

Rustc ensures references are always valid. Unsafe Rust has two new types called *raw pointers* that are similar to references. As with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`(the asterisk **isn't** the dereference operator, it's part of the type name). In the context of raw pointers, *immutable* means that the pointer can't be directly assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:

- Ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
- Aren't guaranteed to point to valid memory
- Are allowed to be null
- Don't implement any automatic cleanup

This allows for greater performance or the ability to interface with another language or hardware where Rust's guarantees don't apply.

One major use case for raw pointer is when interfacing with C code. Another case is when building up safe abstractions that the borrow checker doesn't understand.

```rust
let mut num = 5;

/*
  ********* SAFE ***************

  * Notice there's no need to use 'unsafe'
  * We can create raw pointers in safe code
  * We can't dereference raw pointers outside an unsafe block
  ------------------------------------------------------------
  * Because we've created these raw pointers
  * directly from references guaranteed to be valid
  * we know these particular raw pointers are valid
  ------------------------------------------------------------
  * Notice also here that we have both a mutable pointer
  * and an immutable pointer, pointing to the same location
  * in memory, potentially creating a data race!
*/
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

/*
  * Creating raw pointers does no harm; only when we try
  * to access the value that it points at that we
  * might end up dealing with an invalid value
*/
unsafe {
  println!("r1 is {}", *r1);
  println!("r2 is {}", *r2);
}

/*
  ********* UNSAFE ***************

  * Here we create a raw pointer to an
  * arbitrary location in memory
  * trying to use arbitraty memory is undefined:
  * there might be data at that address or there might be not
  * the compiler might optimize the code so there is no memory access,
  * or the program might error with segmentation fault
*/
let address = 0x012345usize;
let r = address as *const i32;
```

## Calling Unsafe Function or Method

```rust
/*
  * The unsafe keyword in this context indicates the function
  * has requirements we need to uphold when we call this function
  * because Rust can't guarantee we've met these requirements
*/
unsafe fn dangerous() {}

/*
  * By calling an unsafe function within an `unsafe` block
  * we're saying that we've read this function's documentation
  * and take responsability for it
*/
unsafe {
  dangerous();
}
```

## Creating a safe abstraction over unsafe code

Wrapping unsafe code in a safe function is a common abstraction. Let's study the function `split_at_mut` from the `std` library to better understand this.

```rust
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();

  assert!(mid <= len);

  // This will panic
  // Rust borrow checker can't understand that
  // we're borrowing different parts of the slice,
  // it only knows we're borrowing from the same slice twice
  // This is fundamentally ok since the two slices don't overlap
  // but the compiler isn't smart enough to know this
  // unsafe code is needed here
  (&mut slice[..mid],
   &mut slice[mid..])

   // ----- CORRECT WAY ----------

   use std::slice;

   let len = slice.len();
   // with the following method, we access the raw pointer of the slice
   // in this case, it will return a '*mut i32'
   let ptr = slice.as_mut_ptr();

   assert!(mid <= len);

  // slice::from_raw_parts_mut is unsafe because it takes a raw pointer
  // and must trust that this pointer is valid
  // the offset method is also unsafe because it must trust that the
  // offset location is also a valid pointer

   unsafe {
     (slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
   }
}

let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &must v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

## `extern` function to call external code

When Rust needs to interact with code written in another language, it uses the `extern` keyword, that facilitates the creation and use of a *Foreign Function Interface(FFI)*.

FFI is a way for a programming language to define functions and enable a different programming language to call those functions.

```rust
// here we list the names and signatures of external functions from
// another language we want to call. The "C" part defines the
// application binary interface (ABI). The ABI defines how to call
// the function at the assembly level
extern "C" {
  fn abs(input: i32) -> i32;
}

fn main() {
  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
}
```

You can also call Rust's function on other languages by adding the `extern` keyword and specify the ABI to use just before the `fn` keyword.

```rust
// the no_mangle annotation is due to Mangling in the compiler.
// mangling is when the compiler changes the name we've given a function
// to a different name that contains more information for the other parts of
// the compilation process to consume, but is less human readable
// for Rust's function to be nameable by other languages, we must disable this
#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}
```