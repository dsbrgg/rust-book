fn main() {
  interpreter("10", "1010101"); // "0101010"
  interpreter("100", "1111111111"); //"0101010101"
}

fn interpreter(cmd: &str, bits: &str) -> String {
  let mut index = 1;
  let mut result = String::new();
  let mut last_cmd = String::new();
  let instructions = cmd.chars().cycle();
  
  for instruction in instructions {
    println!("{} - {}", instruction, index);
    match instruction {
      '0' => {
        if last_cmd.len() > 0 && last_cmd[..] == "0"[..] { 
          result.push('0');
        } else {
          last_cmd.push('0');
        }

        index += 1;
      },
      '1' => {
        if last_cmd.len() > 0 && last_cmd[..] != "1"[..] { 
          last_cmd.pop();
          last_cmd.push('1'); 
        }

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
