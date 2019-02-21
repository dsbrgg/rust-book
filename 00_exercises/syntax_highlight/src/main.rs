fn main() {
    highlight("FFFR345F2LL");
}

pub fn highlight(code: &str) -> String {
  // Implement your syntax highlighter here
  code.to_string()
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

