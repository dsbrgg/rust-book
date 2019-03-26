fn main() {
  decomp(12); // "2^10 * 3^5 * 5^2 * 7 * 11"
}

fn decomp(n: i32) -> String {
  use std::collections::HashMap;

  let mut stop = false;
  let mut exp: HashMap<&i32, &i32> = HashMap::new();
  let mut factorial = (2..=n).fold(1, |acc, num| acc * num);
  
  let mut i = 2;

  while !stop {
    while factorial % i == 0 {
      factorial = factorial / i;

      let value = exp.get(&i);
      if value != None { 
        exp.insert(&i, &(*value.unwrap() + 1)); 
      } else {
        exp.insert(&i, &i);
      }
    }

    println!("{:#?}", exp);

    stop = true;
  }

  String::new()
}
