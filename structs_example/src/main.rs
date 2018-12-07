#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );

  println!("=========================");

  let rect1 = Rectangle { width: 30, height: 50 };

  println!("Current rectangle is: {:#?}", rect1);

  println!(
    "The area of the rectangle is {} square pixels.", 
    area_as_struct(&rect1)
  );

  println!("=========================");

  let rect2 = Rectangle { width: 50, height: 50 };

  println!("Current rectangle is: {:#?}", rect2);

  println!(
    "The area of the rectangle is {} square pixels.", 
    rect2.area()
  );
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

// here the area has a signature of one parameter which will access the Rectangle
// instance for each field (width and height) which makes it much more meaningful

fn area_as_struct(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}