#[derive(Debug)]
pub struct Rectangle {
  length: u32,
  width: u32,
}

impl Rectangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.length > other.length && self.width > other.width
  }
}


pub fn add_two(a: i32) -> i32 {
  a + 2
}

// changed this to not include name to show assertion with details
pub fn greeting(_name: &str) -> String {
  format!("Hello {}!", "")
}

// Because the tests module is an inner module,
// we need to bring the code under test in the outer module
// into the scope of the inner module
// we use a glob here so anything we define in the outer module
// is available to this tests module
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle { length: 8, width: 7 };
    let smaller = Rectangle { length: 5, width: 1 };

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle { length: 8, width: 7 };
    let smaller = Rectangle { length: 5, width: 1 };

    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn it_adds_two() {
    assert_eq!(4, add_two(2));
  }

  #[test]
  fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
      result.contains("Carol"),
      "Greeting did not contain name, value was {}", result
    );
  }
}