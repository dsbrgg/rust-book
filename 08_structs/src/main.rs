#[derive(Debug)] // trait to be able to print struct
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let email = String::from("some@email.com");
  let username = String::from("my_username");

  let my_user = build_user(email, username);
  let my_other_user = User {
    email: String::from("another@email.com"),
    username: String::from("another_username"),
    ..my_user // struct update syntax
  };

  println!("My user: {:?}", my_user);

  println!("Other user: {:?}", my_other_user);
}

fn build_user(email: String, username: String) -> User {
  User {
    username, // field init shorthand
    email, // field init shorthand
    sign_in_count: 1,
    active: true,
  }
}