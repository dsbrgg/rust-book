fn main() {
  basic_tests();
}

fn nb_dig(n: i32, d: i32) -> i32 {
  (0..=n)
    .fold(0, |mut acc, k| {
      let kk = k.pow(2);
      let ks = kk.to_string();
      ks.chars().for_each(|s| {
        let sd = s.to_digit(10).unwrap() as i32;
        if sd == d { acc += 1; }
      });

      acc
    })
}

fn dotest(n: i32, d: i32, exp: i32) -> () {
  println!("n: {:?}", n);
  println!("d: {:?}", d);
  let ans = nb_dig(n, d);
  println!("actual:\n{:?}", ans);
  println!("expect:\n{:?}", exp);
  println!("{}", ans == exp);
  assert_eq!(ans, exp);
  println!("{}", "-");
}

fn basic_tests() {
  dotest(550, 5, 213);
  dotest(5750, 0, 4700);  
}