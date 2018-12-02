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

fn calculate_length(s: &String) -> usize {
    s.len()
}
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


