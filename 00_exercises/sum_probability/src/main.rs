fn main() {
  rolldice_sum_prob(11, 2);
  rolldice_sum_prob(8, 3);
}

fn rolldice_sum_prob(sum:i32, dice_amount:i32) -> f64 {
  let mut matrix: Vec<_> = Vec::new();
  let all_comb = 6u32.pow(dice_amount as u32);

  println!("all possible combinations -> {:#?}", all_comb);

  for v in 1..=6 {
    matrix.push(Vec::new());

    let index: usize = v - 1;
    for _d in 1..dice_amount {
      (1..=6).for_each(|x| {
          let dv = x - 1;
          if matrix[index].len() == 6 {
            matrix[index][dv] += x;
          } else {
            matrix[index].push(v+x);
          }
      })
    }
  }

  let flatten = matrix.into_iter().flatten().collect::<Vec<usize>>();
  println!("flatten -> {:#?}", flatten);
  let filter = flatten.into_iter().filter(|&x| x == sum as usize).collect::<Vec<usize>>();

  println!("filter -> {:#?}", filter);

  // :(
  // let total_times = matrix
  //   .into_iter()
  //   .flatten()
  //   .collect::<Vec<usize>>()
  //   .into_iter()
  //   .filter(|&x| x == sum as usize)
  //   .collect::<Vec<usize>>()
  //   .len() as u32;

  // println!("total_times -> {:#?}", total_times);

  // (total_times / all_comb) as f64

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