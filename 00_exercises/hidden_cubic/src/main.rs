fn main() {
  is_sum_of_cubes("00 9026315 -827&()"); // "0 0 Lucky"
  // is_sum_of_cubes("0 9026315 -827&()"); // "0 0 Lucky"
  is_sum_of_cubes("Once upon a midnight dreary, while100 I pondered, 9026315weak and weary -827&()"); // "Unlucky"
}

extern crate regex;
use regex::Regex;

fn is_sum_of_cubes(s: &str) -> String {
  let mut add = 0;
  let mut sum = String::new();
  let re = Regex::new(r"(\d){1,3}").unwrap();

  re.captures_iter(s)
    .for_each(|sp| {
      let c = sp[0].chars();
      let mut og = String::new();

      let result = c.fold(0, |mut acc, n| {
        og.push(n);

        acc += n
          .to_digit(10)
          .unwrap()
          .pow(3);

        acc
      });

      let parsed = og.parse::<u32>().unwrap();

      if parsed == result {
        add += result;
        sum.push_str(
          &format!("{} ", &result.to_string())
        ); 
      }
    });

  if sum.len() == 0 { 
    sum.push_str("Unlucky");
    return sum;
  }

  sum.push_str(
    &format!("{} Lucky", add.to_string())
  );

  sum
}