fn main() {
  decomp(12); // "2^10 * 3^5 * 5^2 * 7 * 11"
}

fn factor(n: &i32) -> u64 {
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

  let mut stop = false;
  let mut exp: HashMap<&i32, i32> = HashMap::new();
  let mut factorial = (2..=n).fold(1, |acc, num| acc * num);

  let mut i = 2;

  while !stop {
    factorial = factorial / i;

    if factorial % i == 0 {
      let value = exp.get(&i);

      match value {
        None => exp.insert(&i, i),
        Some(v) => exp.insert(&i, *v + 1),
      };
    } else {
      i += 1;

      if factor(&i) > 1 && (i as i32) < factorial {
        i += 1;
      } else {
        stop = true;
      } 
    }
  }

  String::new()
}
