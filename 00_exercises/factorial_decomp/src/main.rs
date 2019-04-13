fn main() {
  // decomp(12); // "2^10 * 3^5 * 5^2 * 7 * 11"
  decomp(22); // "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19"
}

fn factor(n: usize) -> u64 {
  let mut c = 0;
  let mut num = n as u64;

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

fn decomp(n: i32) -> String {
  use std::collections::HashMap;

  let mut primes: Vec<usize> = Vec::new();
  let mut exp: HashMap<usize, i32> = HashMap::new();
  // numbers will overflow when doing the factorization, so it needs wrapping
  // https://users.rust-lang.org/t/panicked-at-arithmetic-operation-overflowed/4290
  let mut factorial: usize = (2..=n).fold(1, |acc, num| acc.wrapping_mul(num as usize));

  let mut i = 2;

  loop {
    if factorial % i == 0 {
      factorial = factorial / i;
      println!("i -> {}\ni x {:?}\nfactorial -> {}\nmodulo -> {}\n", i, exp.get(&i), factorial, factorial % i);
      let value = exp.get(&i);

      match value {
        None => { exp.insert(i, 1);  primes.push(i); () },
        Some(v) => { exp.insert(i, *v + 1); () },
      };

      if factorial / i == 0 { break; }
    } else {
      i += 1;

      if factor(i) > 1 { i += 1; }
    }
  }

  let mut builder = String::new();

  for (i, p) in primes.iter().enumerate() {
    let t = exp.get(p).unwrap();

    if *t != 1 {
      builder.push_str(&format!("{}^{}", p, t));
    } else {
      builder.push_str(&format!("{}", p));
    }

    if i != primes.len() - 1 {
      builder.push_str(&" * "[..]);
    }
  }
  println!("{:?}", builder);
  builder
}
