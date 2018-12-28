fn main() {
  returns_expected();
}

fn DNA_strand(dna: &str) -> String {
  let mut d = dna.chars();
  let mut s = String::new();
  
  loop {
    match d.next() {
      Some('A') => s.push('T'),
      Some('T') => s.push('A'),
      Some('G') => s.push('C'),
      Some('C') => s.push('G'),
      None => break,
      _ => (),
    }
  };

  s
}

// way better ->
//  dna.chars()
//   .map(|c| match c {
//     'A' => 'T',
//     'C' => 'G',
//     'G' => 'C',
//     'T' => 'A',
//     _ => panic!()
//   }
//   )
//   .collect()

fn returns_expected() {
  assert_eq!(DNA_strand("AAAA"),"TTTT");
  assert_eq!(DNA_strand("ATTGC"),"TAACG");
  assert_eq!(DNA_strand("GTAT"),"CATA");
}