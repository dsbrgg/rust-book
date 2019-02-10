// standard crates
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// Traits
use std::hash::Hash;
use std::clone::Clone;

struct Cacher<T, A>
  where T: Fn(A) -> u32,
        A: Eq + Hash + Clone
{
  calculation: T,
  value: HashMap<A, u32>
}

impl<T, A> Cacher<T, A>
  where T: Fn(A) -> u32,
        A: Eq + Hash + Clone
{
  fn new(calculation: T) -> Cacher<T, A> {
    Cacher {
      calculation,
      value: HashMap::new()
    }
  }

  fn value(&mut self, arg: A) -> u32 {
    match self.value.get(&arg) {
      Some(v) => *v,
      None => {
        let v = (self.calculation)(arg.clone());
        self.value.insert(arg, v);
        v
      }
    }
  }
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  let another_user_specified_value = 20;
  let another_simulated_random_number = 4;

  generate_workout_plan(
    simulated_user_specified_value,
    simulated_random_number
  );

  println!("\n======================\n");

  generate_workout_plan(
    another_user_specified_value,
    another_simulated_random_number
  );
}

fn generate_workout_plan(intensity: u32, random_number: u32) {
  // type annotation for closures is optional
  let mut expensive_closure = Cacher::new(|num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      expensive_closure.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_closure.value(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_closure.value(intensity)
      );
    }
  }
}