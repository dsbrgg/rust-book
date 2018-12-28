# Common Collections

- *Vector* allows you to store a variable number of values next to each other.
- *string* is a collection of characters.
- *Hash map* allows you to associate a vlue with a particular key.

Unlike arrays or tuples, these collections are stored on the heap which means it's not necessary to know their size at compile time. Each has different capabilities and costs.

For other kinds of collections on the standard library, [check this out](https://doc.rust-lang.org/std/collections/index.html).

## Storing Lists of Values with Vectors

*Vectors* allow you to store more than one value in a single data structure and puts all the values next to each other in memory. Vectors can only store values of the same type.

Creating a new vector:

```rust
// new empty vector with associated function new()
let v: Vec<i32> = Vec::new();

// with vec! macro to initialize with predetermined data
let v = vec![1, 2, 3];
```

Updating a vector:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Reading elements of vectors:

* Notice how `&` is used to get the reference to a position in the vector
  * If the index is out of bounds, it will `panic`
* The `.get()` method from vectors returns an `Option<T>`
  * This is more error friendly, for example, when the data might be coming from an input

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
  Some(third) => println!("The third element is {}", third),
  None => println!("There's no third element"),
}
```

Iterating over vector values:

- To change the value that the mutable reference refers to, we have to dereference it(`*`).

```rust
let v = vec![100, 32, 7];

// immutable reference
for i in &v {
  println!("{}", i);
}

// mutable reference (map over elements)
for i in &mut v {
  *i += 50;
}
```

When a vector gets dropped(its scope ends), all of its contents are also dropped. It can get more complicated when you start to introduce references to the elements of the vectors.

When the program has a valid reference, the borrow checker enforces the ownership and the borrowing rules to ensure this reference and any other references to the contents of the vector remain valid.

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is : {}", first);
```

The above example will result in this error:

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  --> src/main.rs:10:5
   |
8  |     let first = &v[0];
   |                  - immutable borrow occurs here
9  |
10 |     v.push(6);
   |     ^^^^^^^^^ mutable borrow occurs here
11 |
12 |     println!("The first element is: {}", first);
   |                                          ----- borrow later used here
```

This error because of how vectors works: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn't room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. Borrowing rules prevent that.

## Enum to Store Multiple Types