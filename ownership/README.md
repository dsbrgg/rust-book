# Ownership

Rust ownership is a feature unique to this language.
It makes memory safety guarantees without needing a garbage collector.

All programs must manage the computer's memory while running. Some languages
have garbage collection that will constantly look for no longer used memory.
Other languages the programmer has to explicitly allocate and free memory.

In Rust uses ownership which is a set of rules that the compiler checks at compile time.
None of the features from ownership slows down your program while running.

Before diving too deep at this, let's look at other concepts...

## The Stack and the Heap

Both the stack and the heap are parts of memory that are available to your code to use
at runtime, but structered in diffent ways. Stack stores values in the order it gets them
and removes them in the opposite order (*last-in*, *first-out*). Adding cata is the same as
*pushing onto the stack* and removing data is *popping off the stack*.

The stack is fast because it never has to search for a place to put new data or a place to
get data because the place is **always on top**. Another thing is that all data in the stack
must take up a known, fixed size.

Data with unknown size at compile time or size that might change is **stored on the heap** intead. When you put data on the heap, you ask for some amount of space; the operating system
finds an empty spot somewhere that is big enough, **marks it as in use** and returns a *pointer* which is the address of that location. Because the pointer is a known, fixed size,
you can store the pointer on the stack but to get the actual data, you have to follow the pointer.

>  Heap analogy -

> Think of being seated at a restaurant. When you enter, you state the number of people in your  group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.
Consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). 
> Allocating a large amount of space on the heap can also take time.

Accessing data in the heap is slower because you have to follow a pointer to get to the data.
Contemporary processor are faster if they jump around less in memory.

When calling a function, the values passed to it and potentially pointers to data on the heap,
as well as the function's local variables, they are all pushed onto the stack.
When the function is done, all the values are popped off of the stack.

These memory problems of track data from parts of the code that should be stacked on the heap,
minimizing the amount of duplicate data on the heap and cleaningup unused data on it so you don't
run out of space are all problems addressed on the ownership feature.

## Ownership Rules

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. when the owner does out of scope, the value will be dropped.

## Variable Scope

```rust
{                   // s is not valid here, not declared
  let s = "hello"; // s is valid from this point forward
}                 // this scope is over,s is no longer valid 
```

- When **s** comes *into scope*, it is valid.
- It remains valid until it goes *out of scope*.

## The `String` Type

For a better understanding of how Rust handles data stored on the heap
and how it cleans up data, we will have to look into a more complex data types.

We will focus on the parts of `String` that relate to ownership.
These aspects also apply to other complex data types provided by the standard
library and the ones you will be creating.

String literals are convenient but not suitable for all situations. They are immutable and
not every string can be known when we write code (eg. user input string). For that,
Rust has a `String` type which is allocated on the heap and because of that is able to 
store an amount of text that is unknown to us at compile time. The `::` you will see next
is used to allow calling a particular function (`from`) under the `String` type (method calling).

```rust
let s = String::from("hello");
```

This string just created can be muted.

```rust
let mut s = String::from("hello");

s.push_str("", "world!");

println!("{}", s);
```

The difference here from string literals is how they deal with memory.

## Memory Allocation

String literals are fast and effecient because they are hardcoded directly into the final
executable. Unfortunately they can't always be used because there's no way to put a
blob of memory with size unknown and that might change into that piece of variable.

With `String` type, to support a growable/mutable piece of text, we allocate memory 
on the heap, unknown at compile time, to hold its contents, this means:

- Memory must be requested from the OS at runtime.
- A way of returning this memory to the OS when we're done, is needed.

In languages with a *garbage collector (GC)*, the GC keeps track and cleans up
memory that isn't being used anymore and there's no need to think about it.
Without a GC, it's the programmer responsability to identify when memory is no
longer being used and call code to explicitly return it. This had been a historical
difficulty for programmers because:

- If you forget about it, you'll waste memory;
- If you do it too early, you'll have an invalid variable;
- If you do it twice, it's a bug too.

For all `allocate` there should be its pair, which is `free`.

In Rust, a different path is taken. Memory is automatically returned once th variable
that owns it goes out of scope.

There's a natural point we can return the memory our `String` needs to the operating system that
is when the variable goes out of scope. Rust has a specific function for this action which is
`drop`. It is called automatically at the closing body (`}`).

> Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.

## Ways Variables and Data Interact: Move

Lets look an example with integers:

```rust
let x = 5;
let y = x;
```

Since these are two simple values with a known, fixed size, these two has `5` as a value which are pushed onto the stack.

Now, a `String` version:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Even though this looks exactly as the previous example, it is not.
Under de cofers, a `String` is made up of three parts: a pointer to the memory that
holds the contents of the string, a length and a capacity. This group of data is stored
on the stack. The memory on the heap is the one with the actual contents:

**s1 group data :point_down:**

name | value
---- | ------
pointer  | 0x283u28
len      | 5
capacity | 5

**memory at 0x283u28 :point_down:**

index | value
----- | -----
0     | h
1     | e
2     | l
3     | l
4     | o

> Quick note
- len: how much memory, in bytes, the contents of the `String` is currently using.
- capacity: the total amount of memory, in bytes, that the `String` has received from the OS.

The difference between length and capacity matters, but not for now. :smirk:

When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the pointer, the length
and the capacity that are on the stack. Meaning that the table for **s1 group data** would be
*exactly the same* for **s2 group data** (both point to the same location in memory).

This representation is *not* the same as if the heap data was copied as well. `s2` would have
another pointer and doing operations like `s2 = s1` could be very expensive in terms of runtime
performance if data on the heap were large.

In this exact situation we're dealing with, we would also have a bug. Because Rust calls `drop`
on the end of the function body, both `s1` and `s2` will try to free the same memory.
This is the *double free* mentioned earlier and is one of the memory safety bugs Rust
tries to prevent to prevent memory corurption and security vulnerabilities.

Since we're using Rust, it will automatically interpret this assignment as `s1` no longer being
valid and therefore, Rust doesn't need to free anything when `s1` goes out of scope. If you try
to `s1` after `s2` is created, it won't work:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

An error will occur because Rust prevents you from using an invalidated reference:

```
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

The concept of copying the pointer, length and capacity without copying the data probably sounds
like making a *shallow copy* but, because Rust also invalidates the first variable, it is called 
a *move* instead. We would read this assignment as `s1` being moved into `s2`.

There's also a design choice that's implied here: Rust will never automcatically create a "deep"
copy of your data. Any *automatic* copying can be assumed to be inexpensive in terms of runtime
performance.

## Ways Variables and Data Interact: Clone

If a ddep copy of the heap data is wanted and not only the stack data, we can use a common method called `clone`.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This will make the heap data be copied over from `s1` to `s2`. When you use `clone`, you know
that some arbitrary code is being executed and that code may be expensive.

## Stack-Only Data: Copy

Rust has a special annotation called `Copy` that can be placed on types like integers that
are stored on the stack. If a type has the `Copy` trait, an older variable is still usable
after assignment. It won't let us annoatate a type with the `Copy` trati if the type, or any
of its parts has implemented the `Drop` trait. *Appendix C - Derivable Traits* goes more in depth
about the `Copy` annotation to your type.

As a general rule, any gorup of simple scalar values can be `Copy` and nothing that requires allocation or is some form of resource is `Copy`. Some types that are `Copy`:

- All integers
- Booleans
- Floating points
- Characters
- Tuples which contain only `Copy` types(eg. `(u32, f64)`)

## Ownership and Functions

```rust
fn main() {
  let s = String::from("hello"); // s comes into scope

  takes_ownership(s);  // s value moves into the function
                      // so is no longer valid here

  let x = 5;        // x comes into scope

  makes_copy(x);   // x would move into the function
                  // but i32 is COpy, so it's okay to still
                 // use x afterward
} // Here x goes out fo scope, then s. But because s value was moved, nothing
 // special happens

fn takes_ownership(some_string: String) { // some_string comes into scope
  printlen!("{}", some_string);
} // Here some_string goes ot of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  printlen!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

## Return Values and Scope

