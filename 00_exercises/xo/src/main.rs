fn main() {
  returns_expected();
}

fn xo(string: &'static str) -> bool {
  let mut total_x = 0;
  let mut total_o = 0;

  let parsed = string.to_lowercase();

  parsed.chars().for_each(|c| {
    if c == 'x' { total_x += 1; }
    else if c == 'o' { total_o += 1; }
  });
  
  total_x == total_o
}

// best graded implementation
fn another_xo(string: &'static str) -> bool {
  string.chars().fold(0, |a, c|{
    match c {
      'x' | 'X' => a + 1,
      'o' | 'O' => a - 1,
      _ => a
    }
  }) == 0
}

fn returns_expected() {
  assert_eq!(xo("xo"), true);
  assert_eq!(xo("Xo"), true);
  assert_eq!(xo("xxOo"), true);
  assert_eq!(xo("xxxm"), false);
  assert_eq!(xo("Oo"), false);
  assert_eq!(xo("ooom"), false);
}