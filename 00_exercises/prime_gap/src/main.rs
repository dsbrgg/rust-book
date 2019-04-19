fn main() {
  gap(2, 100, 110); // Some((101, 103))
  // gap(4, 100, 110); // Some((103, 107))
}

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
    .inspect(|x| println!("x -> {:?}\nis_prime -> {:?}\n", x, is_prime(*x)))
    .fold((0, 0), |mut tuple, x| {
      let p = is_prime(x);

      match tuple {
        (0, 0) if p => tuple.0 = x,
        (y, 0) if p => tuple.1 = x,
        (y, z) if p && ((y - z) as i64) < 0 => tuple.0 = x,
        (y, z) if p && z - y != g as u64 => tuple.1 = x,
        _ => (),
      };

      tuple
    });
    
  Some((1, 1))
}