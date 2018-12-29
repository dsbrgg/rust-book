mod mmm {
  use std::collections::HashMap;

  pub fn mean(list: &[usize]) -> usize {
    let sum: usize = list.iter().sum();

    sum / list.len()
  }

  pub fn median(list: &[usize]) -> usize {
    let mut _list: Vec<usize> = list.clone().iter().map(|x| *x).collect();

    _list.sort();
    _list.dedup();

    let m = _list.len() / 2;

    _list[m]
  }

  pub fn mode(list: &[usize]) -> usize {
    let mut m = (0, 0);
    let mut occurences = HashMap::new();

    for i in list {
      let count = occurences.entry(i).or_insert(0);
      *count += 1;
    }

    for (k, v) in occurences {
      if v > m.1 { m = (*k, v); }
    }

    m.0
  }
}

fn main() {
  let l = [ 8, 3, 1, 7, 1, 4, 6 ];

  println!("{}", mmm::mean(&l));
  println!("{}", mmm::median(&l));
  println!("{}", mmm::mode(&l));
}