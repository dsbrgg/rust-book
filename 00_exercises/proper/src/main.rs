fn main() {
  println!("{:?}", buddy(10, 50)); // Some((48, 75))
  println!("{:?}", buddy(1081180, 1103735)); // Some((1081184, 1331967)
}

fn sum_propers(n: i64) -> i64 {
  (1..n)
    .filter(|y| n % y == 0)
    .sum()
}

fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
  for n in start..=limit {
    let m = sum_propers(n);
    let b = sum_propers(m - 1);

    if b - 1 == n { return Some((n, m-1)); }
  }

  None
}