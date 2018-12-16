fn dont_give_me_five(start: isize, end: isize) -> isize {
  let next = start;

  let n = next.to_string();
  let e = n.len();

  let last: isize = &n[e-1..].parse().unwrap();

  let is_five = last == 5;
  let less_than_five = last < 5;
  let greater_than_five = last > 5;
  
  match last {
    5 => println!("Is five"),
    _ => (),
  }
}

// ------------------------------------- main

fn main() {
  println!("{:#?}", dont_give_me_five(1, 9));
}

// fn returns_expected() {
//   assert_eq!(dont_give_me_five(1, 9), 8);
//   assert_eq!(dont_give_me_five(4, 17), 12);
// }