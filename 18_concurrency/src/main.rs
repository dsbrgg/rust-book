use std::thread;
use std::time::Duration;

fn main() {
  basic_threads();

  println!("{}", "============================================");

  wait_threads();

  println!("{}", "============================================");

  closure_threads();
}

fn basic_threads() {
  // this thread will always stop 
  // when the main thread stops
  // meaning this loop will never reach 10
  // unless from_millis is bigger on the main thread
  thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      // this will force the thread to stop execution
      // allowing a different thread to run
      // threads usually take turns but it isn't guaranteed
      // it depends on how the OS schedule the threads
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread", i);
    thread::sleep(Duration::from_millis(1));
  }
}

// the above function will not wait until the spawned
// thread is finished, here we will see a method for
// making sure all threads execute until they finish
fn wait_threads() {
  // the return type of thread::spawn
  // is JoinHandle, it an owned value that
  // when called with a "join" method, it will wait
  // for its thread to finish
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  // uncomment this to see that
  // this will me main() execute the 
  // spawned thread entirely first to then
  // proceed to the main function
  // handle.join().unwrap();

  for i in 1..5 {
    println!("hi number {} from the main thread", i);
    thread::sleep(Duration::from_millis(1));
  }

  // calling "join" blocks the thread currently running
  // until the thread represented by the handle terminates
  // "Blocking" a thread means that the thread is prevented from
  // performing work or exiting
  handle.join().unwrap();
}

// here we will use the "move" keyword before the parameter list of a closure
// to force the closure to take ownership of the values it uses in the environment
// this technique is good when creating threads in order to transfer ownership
// from one thread to another
fn closure_threads() {
  let v = vec![1, 2, 3];

  // we need to use the move keyword here
  // or else Rust won't compile because
  // the thread may outlive the "v" variable reference
  // which would invalidate the program and crash it during runtime
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  handle.join().unwrap();
}