fn dont_give_me_five(start: isize, end: isize) -> isize {
  let mut d = 0;

  for n in (start..end) {
    if !n.to_string().contains('5') { d += 1; }
  }

  return d + 1;
}

// ------------------------------------- main

fn main() {
  returns_expected();
}

fn returns_expected() {
  assert_eq!(dont_give_me_five(1, 9), 8);
  assert_eq!(dont_give_me_five(4, 17), 12);
}

// start 4, end 22
//  4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22

// start 5, end 22
// 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22

// start 6, end 22
// 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22