use std::io;
use std::thread;
use std::io::Write;
use std::time::Duration;

fn main() {
  print!("Counting(in minutes) :");
  io::stdout().flush().ok().expect("Could not flush stdout");

  let mut counter: u64 = 0;

  let mut user_seconds = String::new();
  io::stdin().read_line(&mut user_seconds)
    .expect("Failed to read line");

  let seconds_trim = user_seconds.trim();
  let seconds = seconds_trim.parse::<u64>().unwrap() * 60;

  loop {
    thread::sleep(Duration::from_secs(1));
    
    counter += 1;

    if counter < seconds {
      print!("{}\r", counter);
      io::stdout().flush().ok().expect("Could not flush stdout");
    } else {
      break;
    }
  }
}
