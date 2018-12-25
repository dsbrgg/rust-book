fn main() {
  order("is2 Thi1s T4est 3a");
}

fn order(string: &str) {
  let mut s: Vec<&str> = string.split(' ').collect();
  s.sort_by(|a, b| a.find(char::is_numeric).cmp(&b.find(char::is_numeric)).reverse());

  println!("{:#?}", s.join("\u{0020}"));
}

// fn returns_expected() {
//   assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
//   assert_eq!(order(""), "");
// }
