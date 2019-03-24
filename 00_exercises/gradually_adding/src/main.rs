fn main() {
  // test();
  println!("{}", add(&[4,-3,-2]));
}

fn add(args: &[i64]) -> i64 {
  args
    .into_iter()
    .enumerate()
    .fold(0, |acc, (i, e)| acc + (e * ((i as i64) + 1)))
}

fn test() {
  assert_eq!(add(&[]), 0);
  assert_eq!(add(&[4,-3,-2]), -8);
}