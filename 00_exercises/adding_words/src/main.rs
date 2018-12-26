fn main() {
  struct Arith {
    value: &'static str,
  }

  impl Arith {
    fn add(&self, v: &str) -> &str {
      // terrible ughh...
      let values = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty"
      ];

      let a = values.iter().enumerate().find(|&n| n.1 == &v).unwrap();
      let c = values.iter().enumerate().find(|&n| n.1 == &self.value).unwrap();

      values[a.0 + c.0]
    }
  }

  let t = Arith { value: "one" };

  t.add("seven");
}
