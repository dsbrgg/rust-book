// -----------------------------------------------------------------
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// ------------------ Control Flow ---------------------------------
// Unlike Javascript, values passed to if will not be automatically
// converted to Boolean types. Expressions must be explicit.
// -----------------------------------------------------------------
//
// ------------------ Using if in a let statement ------------------
// Since ifs are an expression, it's possible to use it on the right
// side of a let statement. If expressions can branch "arms" like
// an else arm, and in these cases, it's necessary that all those
// arms returns the same variable type, or else, Rust will panick
// because Rust needs to know which type th variable will be at
// compile time.
// -----------------------------------------------------------------
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// -----------------------------------------------------------------

fn main() {
  let number = 3;
  let not: bool = false;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  let will_return_8 = if_let_statement(not);

  println!("Told you it would return {}!", will_return_8);
}

fn if_let_statement(for_you: bool) -> i8 {
  let lets_see = if for_you {
    2
  } else {
    8
  };

  return lets_see;
}
