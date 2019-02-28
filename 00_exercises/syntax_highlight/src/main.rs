fn main() {
  highlight("FFFR345F2LL");
}

pub fn highlight(code: &str) -> String {
  use std::collections::HashMap;

  let mut highlighted = String::new();
  let mut current_char = String::new();
  let mut current_color = String::new();
  let mut char_to_color: HashMap<char, &str> = HashMap::new();

  char_to_color.insert('L', "red");
  char_to_color.insert('F', "pink");
  char_to_color.insert('R', "green");

  let mut chars = code.chars();

  // highlighted.push_str(r#"<span style="color: pink">F</span>"#),,
  // highlighted.push_str(r#"<span style="color: red">L</span>"#),
  // highlighted.push_str(r#"<span style="color: green">R</span>"#),

  let handle_chars = |color: &str, character: char| {
    let next = current_char.as_bytes().iter().next().unwrap_or(&0);

    let char_as_string = character.to_string();
    let this = char_as_string.as_bytes().iter().next().unwrap();
    
    if *next != 0 as u8 {
      if next == this {
        current_color.push_str(color);
        current_char.push(character);
      } else {
        highlighted.push_str(
          &format!(
            r#"<span style="color: {}">{}</span>"#,
            current_color,
            current_char
          )
        );

        current_color = color.to_string();
        current_char = character.to_string();
      }
    } else {
      current_color.push_str(color);
      current_char.push(character);
    }
  };

  loop {
    match chars.next() {
      Some('(') => (),
      Some(')') => (),
      Some(n) if n.is_ascii_digit() => println!("{} -> {}", "is numeric", n),
      Some(c) if c.is_ascii_alphabetic() => handle_chars(
        char_to_color.get(&c).unwrap(), 
        c
      ),
      Some(_) => break,
      None => break
    }
  }

  println!("{:?}", highlighted);

  highlighted
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

