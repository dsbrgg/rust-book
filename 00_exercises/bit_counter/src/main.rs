// ============================================================
// Write a function that takes an integer as input, 
// and returns the number of bits that are equal to one 
// in the binary representation of that number. 
// You can guarantee that input is non-negative.

// Example: The binary representation of 1234 is 10011010010, 
// so the function should return 5 in this case
// ============================================================

fn main() {
  returns_expected();
}

fn count_bits(n: i64) -> u32 {
  let mut total = 0;
  let mut decimal = n;

  while decimal != 0 {
    let remainder = decimal % 2;
    
    total += remainder;
    decimal = decimal / 2;
  }

  total as u32
}

fn returns_expected() {
  assert_eq!(count_bits(0), 0);
  assert_eq!(count_bits(4), 1);
  assert_eq!(count_bits(7), 3);
  assert_eq!(count_bits(9), 2);
  assert_eq!(count_bits(10), 2);
}

// >> will divide by 2
// << will multiply by 2
fn test_shift() -> i64 { 6 >> 1 }