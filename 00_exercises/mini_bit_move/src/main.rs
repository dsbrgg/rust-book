fn main() {
  interpreter("10", "1010101"); // "0101010"
  interpreter("100", "1111111111"); //"0101010101"
}

fn interpreter(cmd: &str, bits: &str) -> String {
  let mut index = 1;
  let mut result = String::new();
  let mut verifier = cmd.chars().peekable();
  let mut instructions = cmd.chars().cycle();
  
  loop {
    verifier.next();

    let instruction = instructions.next();
    println!("{:?} - {}", instruction, index);
    match instruction {
      Some('0') => {
        index += 1;

        if verifier.peek() == Some(&&'0') { result.push('0'); }
      },
      Some('1') => {
        if index > bits.len() { break; }

        if bits[index-1..index] == "0"[..] {
          result.push('1');
        } else {
          result.push('0');
        }
      },
      _ => ()
    }
  }
  
  println!("{}", result);
  result
}
