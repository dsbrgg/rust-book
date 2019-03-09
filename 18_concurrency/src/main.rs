use std::thread;
use std::time::Duration;

fn main() {
  println!("{}", "\n============================================\n");

  basic_threads();

  println!("{}", "\n============================================\n");

  wait_threads();

  println!("{}", "\n============================================\n");

  closure_threads();

  println!("{}", "\n============================================\n");

  channels();
  
  println!("{}", "\n============================================\n");
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

fn channels() {
  // mpsc stands for multiple producer single consumer
  // meaning a channel can have mutiple sending ends and 
  // only one receiving end
  use std::sync::mpsc;

  // mpsc::channel returns a tuple
  // first element is the sending end
  // sencond element in the receiving end
  let (tx, rx) = mpsc::channel();

  // the spawned thread needs to own the tx end
  // of the channel to be able to send messages
  // through the channel
  thread::spawn(move || {
    let val = String::from("hi");
    // the send method returns a Result<T, E>
    // if the rx end has already been dropped
    // the send operation will return an error
    tx.send(val).unwrap();
  });

  // rx end has two useful methods: recv and try_recv

  // recv will block the main thread's execution and wait
  // until a value is sent down the channel

  // try_recv doesn't block but will return a Result<T, E>
  // immediately: an Ok value holding the message or an Err value
  // if there aren't any messages
  // using try_recv is useful if this thread has other work to do while
  // waiting for messages, we could write a loop that calls try_recv
  // every so often, handles a message if one is available and otherwise
  // does other work until checking again
  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}