fn main() {
  let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
  let mut c = vec!["A", "B", "C", "D"];
  assert_eq!(stock_list(b, c), "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

  b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
  c = vec!["A", "B"];
  assert_eq!(stock_list(b, c), "(A : 200) - (B : 1140)");
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
  use std::collections::HashMap;

  if list_art.len() == 0 || list_cat.len() == 0 {
    return String::new();
  }

  let mut stock = String::new();
  let mut totals: HashMap<String, u64> = HashMap::new();

  let codes: Vec<Vec<_>> = list_art
    .iter()
    .map(|m| {
      let split: Vec<_> = m.split(' ').collect();
      vec![
        split[0].chars().next().unwrap().to_string(),
        split[1].to_string(),
      ]
    })
    .collect();

  for category in list_cat {
    let t = totals.entry(category.to_string()).or_insert(0);
    for code in &codes {
      if category.to_string() == code[0] {
        *t += code[1].parse::<u64>().unwrap();
      }
    }

    match stock.len() == 0 {
      true => stock.push_str(&format!("({} : {})", category, t)),
      false => stock.push_str(&format!(" - ({} : {})", category, t)),
    }
  }

  stock
}
