fn main() {
  returns_expected();
}

fn get_middle_char(s: &str) -> &str {
  let s_length = s.len();

  if s_length <= 2 { return s; }

  let is_even = s_length % 2 == 0;

  let middle_start = (s_length / 2) - 1;
  let middle_end = middle_start + 1;

  if is_even {
    &s[middle_start..=middle_end]
  } else {
    &s[middle_start+1..=middle_end]
  }
}

fn returns_expected() {
  assert_eq!(get_middle_char("test"), "es");
  assert_eq!(get_middle_char("testing"), "t");
  assert_eq!(get_middle_char("middle"), "dd");
  assert_eq!(get_middle_char("A"), "A");
}