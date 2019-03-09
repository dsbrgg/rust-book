
use std::thread;
use std::time::Duration;

// mpsc stands for multiple producer single consumer
// meaning a channel can have mutiple sending ends and 
// only one receiving end
use std::sync::{mpsc, Arc, Mutex};

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

  channels_iterator();

  println!("{}", "\n============================================\n");

  multiple_tx();

  println!("{}", "\n============================================\n");

  single_thread_mutex();

  println!("{}", "\n============================================\n");

  multiple_thread_mutex();

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

fn channels_iterator() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread")
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(2));
    }
  });

  // here we treat rx as an iterator
  // we can use the value directly
  // since we don't have code that pauses or delays
  // in the for loop, we can tell that the
  // main thread is waiting to receive values
  // from the spawned thread
  for received in rx {
    println!("Got: {}", received);
  }
}

fn multiple_tx() {
  let (tx, rx) = mpsc::channel();

  // this will give us a new sending handle
  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread")
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_millis(2));
    }
  });

  // pass the original sending end to this thread
  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you")
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(2));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}

fn single_thread_mutex() {
  let m = Mutex::new(5);

  {
    // we use the lock method to acquire the lock
    // this call will block the current thread so
    // it can't do any work until it's our turn to have the lock
    // this call would fail if another thread holding the lock panicked and
    // no one would ever be able to get the lock
    // by unwrapping, we have this thread panicking if we're
    // in that situation

    // Mutex<T> is a smart pointer
    // the lock method returns a MutexGuard that
    // implements Deref to point to our inner data
    // it also implements Drop which will release the lock
    // automatically when a MutexGuard goes out of scope
    // with these we won't forget to release the lock and
    // blocking the mutex from being used by other threads
    let mut num = m.lock().unwrap();
    *num = 6;
  }

  println!("m = {:?}", m);
}

fn multiple_thread_mutex() {
  // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations(share across threads)
  // the "A" stands for atomic, meaning it's an atomically reference counted type
  // atomics are a kind of concurrency primitive(check std::sync::atomic for more details)

  // not all primitives are atomic by default because
  // thread safety comes with a performance penalty that
  // should be used only when needed
  // if your program runs in a single thread
  // there's no need to have that, which would make your program slower

  // Arc<T> and Rc<T> have the same API
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    // we clone with Arc to let the counter have multiple ownership
    // just like when we saw on the last chapter with Rc<T>
    // the difference, like said before is that Arc is thread safe
    // if we didn't use Arc, we would move the counter variable
    // and Rust would throw a compile error since counter would
    // be moved on the first iteration
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}