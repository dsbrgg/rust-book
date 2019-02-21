fn main() {
    highlight("FFFR345F2LL");
}

pub fn highlight(code: &str) -> String {
  // Implement your syntax highlighter here
  let mut highlighted = Vec::new();
  let mut current_char = '';
  let mut chars = code.chars();

  while chars.next() != None {
    match chars.next().unwrap() {
      'F' => highlighted.push(r#"<span style="color: pink">F</span>"#),
      'L' => highlighted.push(r#"<span style="color: red">L</span>"#),
      'R' => highlighted.push(r#"<span style="color: green">R</span>"#),
      '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => highlighted.push(r#"<span style="color: orange">1</span>"#),
    }
  }
}

// r#(raw string) negates the need to escape special carachaters
fn examples_in_description() {
  assert_eq!(
    highlight("F3RF5LF7"),
    r#"<span style="color: pink">F</span><span style="color: orange">3</span><span style="color: green">R</span><span style="color: pink">F</span><span style="color: orange">5</span><span style="color: red">L</span><span style="color: pink">F</span><span style="color: orange">7</span>"#,
  );
  assert_eq!(
    highlight("FFFR345F2LL"),
    r#"<span style="color: pink">FFF</span><span style="color: green">R</span><span style="color: orange">345</span><span style="color: pink">F</span><span style="color: orange">2</span><span style="color: red">LL</span>"#,
  );
}

