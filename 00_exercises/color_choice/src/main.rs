// TODO: FINISH BASED ON THE DESCRIPTION

fn check_choose(m: u64, n: u64) -> i64 {
  // binomial coefficient:  m = n! / (x! * (n - x)!)

  let mut c: i64 = 1;

  loop {
    let t = f(n) / f(c) * f(n - c);
    if t == m { t } else { c += 1; }
  }
}

fn f(n: u64) -> u64 {
  if n < 2 { 1 } else { n * f(n - 1) }
}

fn dotest(m: u64, n: u64, exp: i64) -> () {
  assert_eq!(check_choose(m, n), exp)
} 

#[test]
fn basics_check_choose() {
  dotest(6, 4, 2);
  dotest(4, 4, 1);
  dotest(35, 7, 3);
  dotest(36, 7, -1);
  dotest(184756, 20, 10);
}

fn main() {
  let t = f(6);
  println!("t = {}", t);
}
