fn main() {
  decomp(12); // "2^10 * 3^5 * 5^2 * 7 * 11"
  decomp(22); // "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19"
}

fn factor(n: &usize) -> u64 {
  let mut c = 0;
  let mut num = n.clone() as u64;

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

  let mut exp: HashMap<usize, i32> = HashMap::new();
  let mut factorial: usize = (2..=n).fold(1, |acc, num| acc.wrapping_mul(num as usize));

  let mut i = 2;

  loop {
    if factorial % i == 0 {
      factorial = factorial / i;

      let value = exp.get(&i);

      match value {
        None => exp.insert(i.clone(), 1),
        Some(v) => exp.insert(i.clone(), *v + 1),
      };

      if factorial / i == 0 { break; }
    } else {
      i += 1;

      if factor(&i) > 1 { i += 1; }
    }
  }



  println!("{:#?}", exp);

  String::new()
}
