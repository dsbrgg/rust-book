fn pig_latin(string: &String) -> String {
  let vowels: [char; 5] = [ 'a', 'b', 'c', 'd', 'e' ];
  let start_w_vowel: bool = vowels.iter().position(|c| c == &string.chars().next().unwrap()) != None;

  if start_w_vowel {
    return format!("{}-{}", string, "hay");
  } else {
    let mut s = string.clone();

    let l = s.remove(0);
    let fs = format!("{}{}", l, "ay");

    return format!("{}-{}", s, fs);
  }
}

fn main() {
  let f = String::from("first");
  let a = String::from("apple");

  println!("{:?}", pig_latin(&f));
  println!("{:?}", pig_latin(&a));
}
