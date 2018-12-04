fn main() {
  let mut s = String::from("hello world");
  let word = first_word(&s); // word will get the value 5

  s.clear(); // This empties the String, making it equal to ""

  println!("string length or first word index: {}", word);
  // world still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> usize {
  //Because we need to go through the String element by element and check whether a value is a space,
  // we’ll convert our String to an array of bytes using the as_bytes method:
  let bytes = s.as_bytes();

  // Next, we create an iterator over the array of bytes using the iter method.
  // For now, know that iter is a method that returns each element in a collection and that enumerate wraps the result 
  // of iter and returns each element as part of a tuple instead. The first element of the tuple returned from enumerate is the index, 
  // and the second element is a reference to the element. 
  // This is a bit more convenient than calculating the index ourselves.
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}