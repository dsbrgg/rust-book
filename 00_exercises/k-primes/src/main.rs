fn main() {
  println!("k primes for 10 -> {}", factor(10));
}

// fn test() {
//   kprimes_step(2, 2, 0, 50);
//   // kprimes_step(6, 14, 2113665, 2113889);
//   // kprimes_step(2, 10, 0, 50);
//   // kprimes_step(5, 20, 0, 50);
// }

/**
 * `k` (integer > 0) which indicates the type of k-primes we are looking for
 * `step` (integer > 0) which indicates the `step` we want to find between two k-primes
 * `start` (integer >= 0) which gives the start of the search (start inclusive)
 * `nd` (integer >= start) which gives the end of the search (nd inclusive)
 */
// fn kprimes_step(k: i32, step: i32, m: u64, n: u64) -> Option<Vec<(u64, u64)>> {
//   return Some(vec![(5, 5)])
// }


fn factor(mut num: u64) -> u64 {
  let mut c = 0;

  while num % 2 == 0 {
    num = num / 2;
    c += 1;
  }

  let mut i = 3;
  while i <= (num as f64).sqrt() as u64 + 1 {
    while num % i == 0 {
      num = num / i;
      c += 1;
    }

    i = i + 2;
  }

  if num > 2 { c += 1; }

  c
}