fn main() {
  // println!("{:?}", buddy(10, 50)); // Some((48, 75))
  println!("{:?}", buddy(1081180, 1103735)); // Some((1081184, 1331967)
}

fn sum_propers(n: i64) -> i64 {
  (1..n)
    .filter(|y| n % y == 0)
    .fold(0, |s, y| s + y)
}

fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
  (start..=limit)
    .fold(None, |mut result, n| {
      let m = sum_propers(n);
      let b = sum_propers(m - 1);

      if b - 1 == n { result = Some((n, m-1)); }

      result
    })
}