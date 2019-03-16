pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      // both private so code from the outside can't directly interact with it
      // also, by being private, we make sure that Posts will always
      // start with its state as a new Draft
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  /* 
    ** This method takes a mutable reference to self,
    ** because we're changing the Post instance that is calling add_text
    ** it also doesn't interact with state at all so it does not depend
    ** on the state pattern
  */
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  // if value is published, we want to return the value
  // in the post's content field, otherwise return an empty string slice
  pub fn content(&self) -> &str {
    // we call the as_ref to borrow the value, which will return Option<&Box<dyn State>>
    // not using as_ref would cause an error because we can't move state out of the
    // borrowed &self of the function paramenter

    // This is one of the cases where we know more than the compiler
    // so we just call unwrap because we know Post will always have
    // Some value when those methods as done
    self.state.as_ref().unwrap().content(&self)
  }

  pub fn request_review(&mut self) {
    // we call the "take" method to take the Some value
    // out of the state Option and leave a None in its place
    // because Rust doesn't let us have unpopulated fields in structs
    // this will let us move the value out of Post instead of borrowing it
    if let Some(s) = self.state.take(){
      // this method consumes the current state and returns a new state
      self.state = Some(s.request_review())
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
 }

/* 
  ** The State trait will define the different
  ** Post states and the Draft.PendingReview and
  ** Draft.Published states will implement the State trait
*/
trait State {
  // now all types that implement State will need to 
  // implement the request_review method
  // Box<Self> means the method is only valid
  // when called on a Box holding the type
  // also, remember that this syntax is not using a reference
  // so it will take ownership and invalidate the old state value
  fn request_review(self: Box<Self>) -> Box<dyn State>;

  fn approve(self: Box<Self>) -> Box<dyn State>;

  // we add a default implementation for the content method
  // that will return an empty slice. This default will be used for
  // Draft and PendingReview states
  // we also need a lifetime annotation here because we're
  // taking a reference to a Post as an argument and returning
  // a reference to part of the post, so the lifetime of the
  // returned reference is related to the lifetime of the
  // post argument
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    ""
  }
}

struct Draft {}

impl State for Draft {
  // Draft will need to return a new boxed instance of PendingReview
  // which represents the state when a post is waiting for a review
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

struct PendingReview {}

impl State for PendingReview {
  // PendingReview also implements State but it won't do anything
  // it returns itself because requesting a review on a PendingReview
  // should mainting its state
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

struct Published {}

// all method on Published should return itself
// since the Published state should stay in its state
// for both cases
impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}