use std::rc::Rc;
use List::{Cons, Nil};

enum List {
  Cons(i32, Rc<List>),
  Nil,
}

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data: {}", self.data);
  }
}

fn test_rc() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

  println!("count after creating a = {}", Rc::strong_count(&a));

  let b = Cons(3, Rc::clone(&a));

  println!("count after creating b = {}", Rc::strong_count(&a));

  {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }

  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn main() {
  let b = Box::new(5);
  println!("b = {}", b);

  let c = CustomSmartPointer { data: String::from("my stuff") };
  let d = CustomSmartPointer { data: String::from("other stuff") };
  let e = CustomSmartPointer { data: String::from("some data") };

  println!("CustomSmartPointer created.");

  drop(e);

  println!("CustomSmartPointer dropped before the end of main.");

  println!("\n===================================\n");

  test_rc();
}
