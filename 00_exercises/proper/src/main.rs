fn main() {
  println!("{:?}", buddy(10, 50)); // Some((48, 75))
  println!("{:?}", buddy(1081180, 1103735)); // Some((1081184, 1331967)
  println!("{:?}", buddy(2177, 2300)); // Some((2295, 2024))
}

// # Summation of proper divisors 
// https://www.geeksforgeeks.org/sum-of-all-proper-divisors-of-a-natural-number/

// sum divisors best solutions
// fn sum_divisors(n: i64) -> i64 {
//   (2..)
//     .take_while(|x| x * x <= n)
//     .filter(|x| n % x == 0)
//     .map(|x| if n / x == x { x } else { x + n / x })
//     .sum::<i64>()
//     + 1 // 1 is always a divisor
// }

fn sum_propers(n: i64) -> i64 {
  let mut r = 0;
  let mut d = 2;

  while (d as f64) <= (n as f64).sqrt() {
    if n % d == 0 {
      if d == (n / d) {
        r += d;
      } else {
        r += d + (n / d);
      }
    }

    d += 1;
  }

  r + 1
}

fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
  for n in start..=limit {
    let m = sum_propers(n);
    let b = sum_propers(m - 1);

    if b - 1 == n && m > n { return Some((n, m-1)); }
  }

  None
}