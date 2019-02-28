fn main() {
  examples_in_description();
}

pub fn highlight(code: &str) -> String {
  use std::collections::HashMap;

  let mut highlighted = String::new();
  let mut current_string = String::new();
  let mut current_color = String::new();
  let mut char_to_color: HashMap<char, &str> = HashMap::new();

  char_to_color.insert('L', "red");
  char_to_color.insert('F', "pink");
  char_to_color.insert('R', "green");
  char_to_color.insert('0', "orange");
  char_to_color.insert('1', "orange");
  char_to_color.insert('2', "orange");
  char_to_color.insert('3', "orange");
  char_to_color.insert('4', "orange");
  char_to_color.insert('5', "orange");
  char_to_color.insert('6', "orange");
  char_to_color.insert('7', "orange");
  char_to_color.insert('8', "orange");
  char_to_color.insert('9', "orange");

  let mut chars = code.chars().enumerate();

  let mut handle_chars = |color: &str, character: char, digit: bool, pass: bool, last: bool| {
    if pass == true {
      highlighted.push_str(&character.to_string());
      ()
    }

    let next = current_string.as_bytes().iter().next().unwrap_or(&0);
    let next_value = current_string.chars().next();
    let mut next_is_digit = false;

    if next_value.is_some() {
      next_is_digit = next_value.unwrap().is_digit(10);
    }

    let char_as_string = character.to_string();
    let this = char_as_string.as_bytes().iter().next().unwrap();

    if *next != 0 as u8 {
      if next == this || next_is_digit && digit {
        current_string.push(character);
      } else {
        highlighted.push_str(
          &format!(
            r#"<span style="color: {}">{}</span>"#,
            current_color,
            current_string
          )
        );

        current_color = color.to_string();
        current_string = character.to_string();
      }
    } else {
      current_color.push_str(color);
      current_string.push(character);
    }

    if last == true {
      highlighted.push_str(
        &format!(
          r#"<span style="color: {}">{}</span>"#,
          current_color,
          current_string
        )
      );
    }
  };

  loop {
    match chars.next() {
      Some((index, '(')) => handle_chars("", '(', false, true, index == code.len() - 1),
      Some((index, ')')) => handle_chars("", ')', false, true, index == code.len() - 1),
      Some((index, c)) if c.is_ascii_alphabetic() => handle_chars(char_to_color.get(&c).unwrap(), c, false, false, index == code.len() - 1),
      Some((index, n)) => handle_chars(char_to_color.get(&n).unwrap(), n, true, false, index == code.len() - 1),
      None => break
    }
  }

  highlighted
}

// fuck my life, this was the best
fn the_best() {
  extern crate regex;
  use regex::{Regex, Captures};

  pub fn highlight(code: &str) -> String {
    let re = Regex::new(r"F+|L+|R+|\d+").unwrap();
    re.replace_all(code, |c: &Captures| match c[0].chars().next().unwrap() {
      'F' => format!(r#"<span style="color: pink">{}</span>"#, &c[0]),
      'L' => format!(r#"<span style="color: red">{}</span>"#, &c[0]),
      'R' => format!(r#"<span style="color: green">{}</span>"#, &c[0]),
      _ => format!(r#"<span style="color: orange">{}</span>"#, &c[0]),
    }).to_string()
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

