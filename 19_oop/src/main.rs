/* 
  * Implementing an OOP Design Pattern
    *** The state pattern has some internal state, which is represented by
    *** a set of state objects and the value's behavior changes based on
    *** the internal state. The state objects share functionality.
    *** Each state object is responsible for its own behavior and for
    *** governing when it should change into another state.
    *** The value that holds a state object knows nothing about the
    *** different behavior of the states or when to transition
    *** between the states. This means that we won't need to 
    *** change the code of the value holding the state or the
    *** code that uses the value. We'll only need to update the code
    *** inside one of the state objects to change its rules or perhaps 
    *** add more state objects.


  * Building a blog post workflow
    *** Functionalities ->
      **** 1. A blog starts as an empty draft.
      **** 2. When a draft is done, a review of the post is required.
      **** 3. When a post is approved, it gets published.
      **** 4. Only published blog posts return content to print, unapproved posts
              can't accidentally be published.
*/

use oop::Post;

fn main() {
  // this will create a new draft blog post
  // we'll only interact with this Post type
  // the states will change in response to its methods
  // called by our library's user
  let mut post = Post::new(); 

  // this will add the blog post into the draft state
  post.add_text("I ate a salad for lunch today");
  // getting it's content before approval will return nothing
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());

  let mut another_post = Post::new();

  another_post.add_text("I ate a salad for lunch today");
  assert_eq!("", another_post.content());

  another_post.request_review();
  assert_eq!("", another_post.content());

  another_post.reject();
  assert_eq!("", another_post.content());
}
