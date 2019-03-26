fn main() {
  decomp(12); // "2^10 * 3^5 * 5^2 * 7 * 11"
}

fn decomp(n: i32) -> String {
  let mut factorial = (2..=n).fold(1, |acc, num| acc * num);
  
  String::new()
}
