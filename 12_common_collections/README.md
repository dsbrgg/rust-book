# Common Collections

- *Vector* allows you to store a variable number of values next to each other.
- *string* is a collection of characters.
- *Hash map* allows you to associate a vlue with a particular key.

Unlike arrays or tuples, these collections are stored on the heap which means it's not necessary to know their size at compile time. Each has different capabilities and costs.

For other kinds of collections on the standard library, [check this out](https://doc.rust-lang.org/std/collections/index.html).

## Storing Lists of Values with Vectors

*Vectors* allow you to store more than one value in a single data structure and puts all the values next to each other in memory. Vectors can only store values of the same type.

### Creating a new vector:

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

### Reading elements of vectors:

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

### Iterating over vector values:

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

Rust needs to know what types will be in a vector at compile time so it knows exactly how much memory on the heap will be needed to store each element.

Enums are defined under the same enum type so, when we need to store elements of a different type in a vector, we can define and use an enum.

```rust
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

let row = vec![
  Spreadsheet::Int(3),
  Spreadsheet::Float(10.12),
  Spreadsheet::Text(String::from("blue")),
];
```

## What is a String?

Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

Rust has only one string type in the core language, which is the string slice(`str`, usually `&str`). String literals are stored in the binary output of the program and are therefore string slices.

`String` type, which is provided by Rust's standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

### Creating a new `String`:

```rust
let mut s = String::new();

let s = String::from("initial contents");
```

You can also create a `String` with whichever data type that has the `Display` trait, including string literals.

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly
let s = "initial contents".to_string();
```

Since strings are UTF-8 encoded, we can include any properly encoded data in them:

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

### Updating a `String`:

```rust
let mut s1 = String::from("foo");
let s2 = "bar";

// push_str doesn't take ownership
s.push_str("bar");

// so s2 is still valid here
println!("s2 is {}", s2);

// push adds a single char
let mut s = String::from("lo");
s.push("l");
```

### Concatenating with `+` or `format!` macro

- The reason for using a reference in the right hand operator using `+`, is because of the method called from this operation(`add`)

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 has been moved here so, it can no longer be used
```

The `add` method called under the hood has somewhat this type of signature:

```rust
fn add(self, s: &str) -> String {}
```

This means that it takes ownership of `self`(first argument, notice no `&`), which means that `self` will be moved here and no longer be valid after that. The second argument will make rust use *deref coercion* to turn the `&String`(from the first example) to a `&String[..]`. This operation looks like a lot of copies are being made but the implementation is more efficient than that.

For more complicated string combining:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

`format!` will return a `String` with its contents. It also **doesn't** take ownership of its paramenters.

### How Rust stores strings in memory (Also why no indexing :sad:)

A `String` is a wrapper over a `Vec<u8>`.

```rust
let len1 = String::from("Holla").len();
let len2 = String::from("Здравствуйте").len();
```

The first example is pretty clear. `len` will return 4, which means the vector storing it is 4 bytes long. Each of the letters takes 1 byte when encoded with UTF-8.

The second example might look like it will return a 12 from `len` but Rust's answer is 24. Each Unicode scalar value in that string takes 2 bytes of storage.

Therefore, an index into the string's bytes will not always correlate to a valid unicode scalar value.

```rust
// invalid code ahead
let hello = "Здравствуйте";
let answer = &hello[0];
```

What should be the `answer`? When encoded in UTF-8, the first byte of `З`(Cyrillic letter Ze) is `208` and the second `151`, so `answer` should be `208` but this is not a valid character on its own.

A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

Instead of using indexing, it is possible to just get a range from a string with string slices:

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```

Following the same example above, you have to be careful though because, the encoding from the above characters takes 2 bytes so, doing `&hello[0..1]` would crash your program since it wouldn't understand the `char` from only that specific byte.

### Iterating over strings

```rust
for c in "नमस्ते".chars() {
  println!("{}", c);
}

// will print
// न
// म
// स
//् these are diacritics that don't make sense on their own
// त
//े these are diacritics that don't make sense on their own
```

Or iterating over the bytes:

```rust
for b in "नमस्ते".bytes() {
  println!("{}", b);
}

// would print
// 224
// 164
// --snip--
// 165
// 135
```

You can also get grapheme clusters(closest to the actual chars as we know) from strings, but it's very complex and not available from standard libraries. Use crates.io for this.

### Summary (for strings)

Strings are more complicated than it looks, every programming language makes different choices about how to present this complexity. In Rust, programmers have to put more thought into handling UTF-8 data upfront but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

### Creating Hash Maps

Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be one of any type. Going through the basic API  for `HashMap<K, V>`:

```rust
// notice we have to import HashMaps on the standard library
use std::collections::HashMaps;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Like vectors, hash maps store their data on the heap. This `HashMap` has the keys of the type `String` and values of type `i32`. Like vectors, they're homogeneous: all the keys must have the same type and all the values must have the same type.

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
```

Here we're using `collect` that uses a vector of tuples, where each tuple consists of a key and its value, which will build the `HashMap`. The `zip` method creates the vector of tuples. The type annotation `HashMap<_, _>` is needed here because it's possible to `collect` into many different data structures and Rust doesn't know which you want unless you specify. For the parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

### Hash Maps Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values. Inserting references, won't move the values into the hash map, the values that the reference point to must be a valid for at least as long as the hash map is valid.

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// field_name and field_value are not valid here because
// they were moved into the hashmap
```

### Accessing Values in a Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 15);

let team_name = String::from("Blue");

// score returns an Option<&V> (Some(&V))
// if no value with that key then, None
let score = score.get(&team_name);
```

### Iterating through Hash Maps

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
  println!("{} : {}", key, value);
}
```

### Updating the Hash Map

```rust
// overwriting a value
use std::collections::HashMap;

let mut scores = HashMap::new();

let mut team_blue = String::from("Blue");
let mut team_yellow = String::from("Yellow");

scores.insert(&team_blue, 10);
scores.insert(&team_blue, 25);

println!("{:#?}", scores);

// only inserting a value if the key has no value

// this will return an Entry enum that represents
// a value that might or might not exist
scores.entry(&team_yellow).or_insert(50);
scores.entry(&team_blue).or_insert(50);

println!("{:#?}", scores);


// updating a value based on old value

let text = "hellow wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
  // or_insert actually returns a mutable reference (&mut V) to that value for this key
  // in order to assign to that value, we must dereference count using the asterisk (*)
  let count = map.entry(word).or_insert(0);
  *count += 1;
}

println!("{:#?}", map);
```