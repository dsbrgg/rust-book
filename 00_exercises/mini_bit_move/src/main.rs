fn main() {
  // interpreter("10", "1010101"); // "0101010"
  // interpreter("100", "1111111111"); //"0101010101"
  interpreter("110", "1010");
}

// I didn't do this, it was the best answer
fn interpreter(tape: &str, data: &str) -> String {
  let mut instructions = tape.chars().cycle();
  let mut bits: Vec<char> = data.chars().collect();
  
  bits.into_iter()
    .map(|mut c| {
      while instructions.next().unwrap() == '1' {
        if c == '0' {
          c = '1'
        } else {
          c = '0'
        }
      }

      c
    }).collect()
}
