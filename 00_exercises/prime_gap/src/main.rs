fn main() {
  assert_eq!(gap(2, 100, 110), Some((101, 103)));
  assert_eq!(gap(4, 100, 110), Some((103, 107)));
  assert_eq!(gap(6, 100, 110), None);
  assert_eq!(gap(8, 300, 400), Some((359, 367)));
}

// interesting implementation to check if it is prime
// assumption x > 2
// fn is_prime(x: u64) -> bool {
//   let sqrt_x = (x as f64).sqrt() as u64;
//   (2..sqrt_x + 1).all(|t| x % t != 0)
// }

fn is_prime(n: u64) -> bool {
  if n <= 3 {
    return n > 1;
  } else if n % 2 == 0 || n % 3 == 0 {
    return false;
  }

  let mut i = 5;
  
  while i * i <= n {
    if n % i == 0 || n % (i + 2) == 0 { return false; }
    i += 1;
  }

  true
}

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
  let primes = (m..=n)
    .filter(|x| is_prime(*x))
    .fold((0, 0), |mut tuple, x| {
      match tuple {
        (0, 0) => tuple.0 = x,
        (_, 0) => tuple.1 = x,
        (y, z) if ((z - y) as i32) != g => {
          tuple.0 = z; 
          tuple.1 = x;
        },
        _ => (),
      };

      tuple
    });

  if ((primes.1 - primes.0) as i32) == g {
    Some(primes)
  } else {
    None
  }
}