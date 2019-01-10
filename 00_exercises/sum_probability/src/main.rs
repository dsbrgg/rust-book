fn main() {
  rolldice_sum_prob(11, 2);
}

fn rolldice_sum_prob(sum:i32, dice_amount:i32) -> f64 {
  let mut matrix: Vec<Vec<_>> = Vec::new();

  for d in 0..dice_amount {
    matrix.push(Vec::new());
    for p in 1..=6 {
      matrix[d as usize].push(p);
    }
  }

  println!("{:#?}", matrix);

  0.1
}

// fn assert_fuzzy_eq(actual:f64,expected:f64,eps:f64) {
//   assert!((actual-expected).abs()<eps,format!("Expected {}, but got {}", expected, actual));
// }

// #[test]
// fn returns_expected() {
//   assert_fuzzy_eq(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
//   assert_fuzzy_eq(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
//   assert_fuzzy_eq(rolldice_sum_prob(8, 3),0.0972222222222, 1e-10);
// }