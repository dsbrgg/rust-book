struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data: {}", self.data);
  }
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
}
