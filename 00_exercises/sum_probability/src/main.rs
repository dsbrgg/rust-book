fn main() {
  // rolldice_sum_prob(11, 2);
  // println!("======================================================================");
  // rolldice_sum_prob(8, 3);
  // println!("======================================================================");
  // rolldice_sum_prob(22, 3);
  // println!("======================================================================");
  rolldice_sum_prob(13, 7);
}

fn rolldice_sum_prob(sum:i32, dice_amount:i32) -> f64 {
  // I'm too bad at this :(
  static DICE: [usize; 6] = [ 1, 2, 3, 4, 5, 6 ];
  
  let mut permutations: Vec<Vec<_>> = Vec::new();
  
  let repeat = (dice_amount - 1) as u32;
  let all_v_comb = 6u32.pow(repeat);
  let all_comb = 6u32.pow(dice_amount as u32);

  for v in &DICE {
    let mut last_repetition: usize = 0;
    for d in 0..all_v_comb {
      let comb_remainder = (d % 6) as usize;
      let dice_value = ((d % 6) + 1) as usize;

      if comb_remainder == 0 {
        permutations.push((0..6).map(|_| v.clone()).collect());
        last_repetition += 1;
      }

      let index = permutations.len() - 1;

      if repeat != 1 {
        (1..repeat).for_each(|_| permutations[index][comb_remainder] += last_repetition);
      }

      permutations[index][comb_remainder] += dice_value;
    }
  }

  println!("{:#?}", permutations);

  let total_times = permutations
    .into_iter()
    .flatten()
    .collect::<Vec<usize>>()
    .into_iter()
    .filter(|&x| x == sum as usize)
    .collect::<Vec<usize>>()
    .len() as f64;

  println!("total_times -> {:#?}", total_times);
  println!("all_comb -> {:#?}", all_comb);
  println!("(total_times / all_comb) -> {:.32}", (total_times / (all_comb) as f64).abs());

  (total_times / (all_comb) as f64)
}

fn assert_fuzzy_eq(actual:f64,expected:f64,eps:f64) {
  assert!((actual-expected).abs()<eps,format!("Expected {}, but got {}", expected, actual));
}

#[test]
fn returns_expected() {
  assert_fuzzy_eq(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
  assert_fuzzy_eq(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
  assert_fuzzy_eq(rolldice_sum_prob(8, 3),0.0972222222222, 1e-10);
  assert_fuzzy_eq(rolldice_sum_prob(13, 7),0.0032757487425697303, 1e-10);
}