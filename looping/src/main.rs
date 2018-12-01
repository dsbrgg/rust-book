// -----------------------------------------------------------------
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// ------------------ Looping ---------------------------------
// Rust has three kinds of loops: loop, while and for.
// loop: will run infinitely, unless a break statement is made.
// loop can also return values
// -----------------------------------------------------------------
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// -----------------------------------------------------------------

fn main() {
  let loop_for = true;
  let loop_while = false;
  let loop_forever = false;

  if loop_for == true {
    _loop_for();
  }

  if loop_while == true {
    _loop_while();
  }

  if loop_forever == true {
    _loop_forever();
  }
}

fn _loop_for() {
  let a = [ 10, 20, 30, 40, 50 ];

  for element in a.iter() {
    println!("the value is: {}", element);
  }

  for number in (1..4).rev() {
    println!("the value from .rev(): {}", number);
  }
}

fn _loop_while() {
  let mut number = 3;

  while number != 0 {
    println!("{}", number);

    number -= 1;
  }

  println!("Done!");
}

fn _loop_forever() {
  // kind of a while loop way using loop only as an expression
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  assert_eq!(result, 20);
}

