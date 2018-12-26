fn main() {
  returns_expected();
}

fn order(string: &str) -> String {
  let mut s: Vec<_> = string.split(' ').collect();

  s.sort_by(|a, b| {
    let cc: Vec<&str> = a.matches(char::is_numeric).collect();
    let nc: Vec<&str> = b.matches(char::is_numeric).collect();
    
    cc[0].cmp(nc[0])
  });

  String::from(s.join("\u{0020}"))
}

fn returns_expected() {
  assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
  assert_eq!(order(""), "");
}
