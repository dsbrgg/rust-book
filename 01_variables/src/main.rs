const MAX_POINTS: u32 = 100_000;

fn main() {
  println!("the value of MAX_POINTS is: {}", MAX_POINTS);

  // mutable variable
  let mut x = 5;

  println!("the value of x is: {}", x);

  x = 6;

  println!("the value of x is: {}", x);

  // shadowing variable
  let y = 10;
  let y = y + 1;
  let y = y * 2;

  println!("the value of y is: {}", y);

  // shadowing also allows to change variable's type
  // mutable variables would give a compile error
  let spaces = "  ";
  let spaces = spaces.len();

  println!("the length of spaces is: {}", spaces);
}
