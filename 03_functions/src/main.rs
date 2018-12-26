// -----------------------------------------------------------------
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// ------------------ Function Name Convention ---------------------
// Rust's convention is snake_case
// -----------------------------------------------------------------

// ------------------ Function Hoisting ----------------------------
// Rust doesn't care where the function is declared
// as long as it's declared somewhere (hoisting)
// -----------------------------------------------------------------

// ------------------ Function Arguments --------------------------
// function signature/interfaces must declare the arguments type
// this is good because it tells the compiles that the variable will
// not be needed anywhere else in the code to figure more stuff out
// -----------------------------------------------------------------

// ------------------ Function Bodies ------------------------------
// functions can end with expressions, since Rus is an
// expresion-based language, it's important to know what are
// statements and expressions and its distinctions
// -----------------------------------------------------------------

// ------------------ Statements && Expressions --------------------
// Creating a variable and a function for example, are a statement
// they do not return a value, since they are not an expression
// eg. let x = 6 || let x = (let y = 6);
// The above example runs ok on C or Ruby, but on Rust, declaring
// a variable won't itself return the value its assigned to it
// Expressions do return a value even if it's the value itself
// eg. let y = 6; --- the "6" is an expression
// Blocks ({}) that are used to create a new scope is an expression
// -----------------------------------------------------------------

// ------------------ Function Return Values -----------------------
// implicitly Rust returns the value of the last 
// expression inside the function body
// -----------------------------------------------------------------
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// -----------------------------------------------------------------

fn main() {
  println!("Hello, world!");

  another_function(500);

  let _your_char = str_for_you();

  println!("Quick laugh for you h{}h{}", _your_char, _your_char);
}

fn another_function(x: i32) {
  println!("Called from another function h\u{03FF}h\u{03FF}; params: {}", x);

  let y = {
    let u = 3;
    x + u // no need for semicolon here, or else you will turn this into a statement
  };

  println!("The value of the y expression is : {}", y);
}

fn str_for_you() -> char {
  'e'
}
