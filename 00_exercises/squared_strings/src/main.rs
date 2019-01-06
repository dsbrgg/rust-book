// closure answer ->
// let rows : Vec<&str> = s.split('\n').collect();
//     (0..rows.len()).map(|i| rows.iter().map(
//         |r| r.chars().nth(i).unwrap()).collect::<String>()
//     ).collect::<Vec<_>>().join("\n")

fn diag_1_sym(s: &str) -> String {
  let split = s.split("\n");
  let mut d1s: Vec<String> = Vec::new();

  for s_split in split {
    for (i, c) in s_split.chars().enumerate() {
      if d1s.get_mut(i) == None {
        d1s.push(String::from(""));
        d1s[i].push(c);
      } else {
        d1s[i].push(c);
      }
    }
  }

  d1s.join("\n")
}

// closure answer ->
// diag_1_sym(s).split('\n').map(
//   |r| r.chars().rev().collect::<String>()).collect::<Vec<_>>().join("\n")

fn rot_90_clock(s: &str) -> String {
  let d1s = diag_1_sym(s);

  let split = d1s.split("\n");
  let mut r90c: Vec<String> = Vec::new();

  for s_split in split {
    let mut ns = String::from(s_split);

    unsafe {
      let v = ns.as_mut_vec();
      v.reverse();
    }

    r90c.push(ns);
  }

  r90c.join("\n")
}

// closure answer ->
// s.split('\n').zip(diag_1_sym(s).split('\n')).map(
//   |(a, b)| a.to_owned() + "|" + b).collect::<Vec<_>>().join("\n")

fn selfie_and_diag1(s: &str) -> String {
  let d1s = diag_1_sym(s);
  let mut sd1: Vec<String> = Vec::new(); 

  let d1s_split: Vec<&str> = d1s.split("\n").collect();
  let selfie_split: Vec<&str> = s.split("\n").collect();
  
  let len = d1s_split.len();
  for i in 0..len {
    let d1s_value = d1s_split.get(i).unwrap().to_string();
    let selfie_value = selfie_split.get(i).unwrap().to_string();
    let complete_value = format!("{}|{}", selfie_value, d1s_value);
    
    sd1.push(complete_value);
  }

  sd1.join("\n")
}

// // first parameter: dots have to be replaced by function of one variable
fn oper(fct: fn(&str) -> String, s: &str) -> String {
  fct(s)
}

fn main() {
  let s = "abcd\nefgh\nijkl\nmnop";

  oper(diag_1_sym, &s[..]);
  oper(rot_90_clock, &s[..]);
  oper(selfie_and_diag1, &s[..]);
}
