fn main() {
  // assert_eq!(dig_pow(89, 1), 1); // should return 1 since 8¹ + 9² = 89 = 89 * 1
  // assert_eq!(dig_pow(92, 1), -1); // should return -1 since there is no k such as 9¹ + 2² equals 92 * k

  dig_pow(3456789, 5);
}

fn dig_pow(n: i64, p: i32) -> i64 {
  let mut k: i64 = 0;
  let mut i = p.clone() as u32;

  let t: i64 = n
    .to_string()
    .chars()
    .fold(0, |acc, d| {
      let digit = d.to_digit(10).unwrap() as i64;
      let sum = acc + digit.pow(i);
      i += 1;
      sum
    });

  while n * k <= t && n * k != t { k += 1; }
  
  if n * k == t { return k; }

  -1
}