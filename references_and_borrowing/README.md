# Reference and Borrowing

Here's a quick example of a function that receives a reference to an object as a parameter
so it won't take ownership of the value:

following code can be run on *src/main.rs* with **cargo run**

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}  // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

Ampersands are *references* that allow you to refer to some value without taking ownership of it.

**reference in function parameter (s: &String) :point_down:**

name | value
---- | -----
pointer | 0x9f9f9f

**reference from value passed to the function (&s1)**

name | value
---- | -----
pointer  | 0x9f9f9f
len      | 5
capacity | 5 

**memory address (0x9f9f9f)**

index | value
----- | -----
0     | h
1     | e
2     | l
3     | l
4     | o

> Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

The `&s1` lets us create a reference that refers to the value of `s1` but does not own it.
Since it does not own it, the value it points to will not be dropped when the reference goes
out of scope.

We call having references as function parameters as *borrowing*. As in real life, if a person
owns something, you can borrow it from them. When you're done, you have to give it back.

Here's an example of trying to modify something from a reference:

```rust
fn main() {
  let s = String::from("hello");

  change(&s);
}

fn change(some_string: &String) {
  some_string.push_str(", world");
}
```

This example will throw an error:

```
error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- use `&mut String` here to make mutable
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ cannot borrow as mutable
```

References are also not immutable by default. We're not allowed to mutate something we have a 
reference to.

## Mutable References

Fixing the above error by adding this:

```rust
fn main() {
  let mut s = String::from("hello");

  change(&mut s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
```

Mutable references have one big restriction: you can only have one mutable reference to a 
particular piece of data in a particular scope. If you try to run the following:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

You'll get the following error:

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```

The benefit of having this restriction is that Rust can prevent data races at compile time.
**Data race** is similar to a race condition and happens when these three behaviours occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

Data races can be difficult to diagnose and fix; Rust prevents this by not even allowing the
code to be compiled.

Having different scopes will allow for multiple mutable references, just not `simultaneous` ones:

```rust
let mut s = String::from("hello");

{
  let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

Another rule for references exists when combining mutable and immutable references:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
```

Error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as
immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // no problem
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

Users of an immutable reference don't expect the values to suddenly change out from under them.
However, multiple immutable references are okay because no one who is just reading the data has
the ability to affect anyone else's reading of the data.

## Dangling References

Languages with pointers have a common situation where you may create a *dangling pointer*, 
whic is a pointer that references a location in memory that may have been given to someone
else, by freeing some memory while presenving a pointer to that memory. In Rust the compiler
guarantees that references will never be dangling references: if you have reference to some data,
the data will not go out of scope before the reference to the data goes.

Let's see an example:

```rust
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
  let s = String::from("hello");

  &s  // we return a reference to the String, s
}    // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!
```

Error:

```
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

**The Rules of References**

- At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
- References must always be valid.