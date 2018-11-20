extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("Secret number is: {}", secret_number);

    loop {
      println!("Please, input your guess.");

      let mut guess = String::new();

      // if expect returns an Result enum of Err
      // the program will crash
      io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

      // this match expession ({}) will handle
      // the Result enums that might return from
      // the parsing from the string to a u32
      // if Err, program will ask for input again
      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      println!("You guessed: {}", guess);

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        }
      }
    }
}
