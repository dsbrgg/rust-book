fn main() {
  // kprimes_step(2, 2, 0, 50);
  // kprimes_step(6, 14, 2113665, 2113889);
  kprimes_step(2, 8, 27553, 27829);
  // println!("k primes for 10 -> {}", factor(10));
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
fn kprimes_step(k: i32, step: i32, m: u64, n: u64) -> Option<Vec<(u64, u64)>> {
  // factor a value to get it's prime factors
  // https://www.geeksforgeeks.org/almost-prime-numbers/
  // http://www.mesacc.edu/~scotz47781/mat120/notes/radicals/simplify/images/examples/prime_factorization.html
  fn factor(mut num: u64) -> Option<u64> {
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

    Some(c)
  }

  let mut kprimes: Vec<(u64, u64)> = Vec::new();

  for number in m..n {

    let mut current = number.clone();
    let mut next = number.clone() + (step as u64);

    if number != 0 {
      if let Some(current_factor) = factor(current) {
        if current_factor == (k as u64) {
          if next <= n {
            if let Some(next_factor) = factor(next) {
              if next_factor == (k as u64) {
                kprimes.push((current, next));
              }
            }
          }
        }
      }
    }
  }

  println!("{:#?}", kprimes);

  Some(kprimes)
}
