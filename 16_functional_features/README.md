# Closures and Iterators

- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements
- The performance of these two features (Spoiler alert: they’re faster than you might think!)

Other Rust features, such as pattern matching and enums, which we’ve covered in other chapters, are influenced by the functional style as well. Mastering closures and iterators is an important part of writing idiomatic, fast Rust code.

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. These are encoded in the three Fn traits as follows:

- `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.

- `FnMut` can change the environment because it mutably borrows values.

- `Fn` borrows values from the environment immutably.

If you want to force the closure to take ownership of the values it uses in the environment, you can use the `move` keyword before the parameter list. This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.

# Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.to_iter();

for val in v1_iter {
  println!("Got: {}", val);
}
```

### Methods that consumes iterators

Methods that call `next` are called consuming adaptors, because calling them uses up the iterator. One example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling `next`, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete.

We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes ownership of the iterator we call it on.

```rust
#[test]
fn iterator_sum() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter();

  let total: i32 = v1_iter.sum();

  assert_eq!(total, 6);
}
```

*Quick note -* The `iter` method produces an iterator over **immutable** references. If we want to create an iterator that takes **ownership** of `v1` and returns owned values, we can call `into_iter` instead of iter. Similarly, if we want to iterate over **mutable** references, we can call `iter_mut` instead of `iter`.

### Methods that produces other iterators

Other methods defined on the `Iterator` trait, known as *iterator adaptors*, allow you to change iterators into different kinds of iterators. You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

```rust
let v1: Vec<i32> = vec![1, 2, 3];


let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

