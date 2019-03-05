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


fn node_example() {
  use std::rc::{Rc, Weak};
  use std::cell::RefCell;

  #[derive(Debug)]
  struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
  }

  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![])
  });

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );

  {
    let branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
      "branch strong = {}, weak = {}",
      Rc::strong_count(&branch),
      Rc::weak_count(&branch)
    );

    println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf)
    );
  }

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );
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

  node_example();
}
