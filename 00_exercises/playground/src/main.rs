fn main() {
  let n = 1;
  let x: Vec<_> = (0..6).map(|_| n).collect();
  println!("{:#?}", x);
}
