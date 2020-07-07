use std::convert::From;
use std::ops::Deref;

use crate::banner;

// tuple struct works well here
// because we never need to reference
// the contents of the box using dot
// syntax
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(value: T) -> MyBox<T> {
    MyBox(value)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> Drop for MyBox<T> {
  fn drop(&mut self) {
    println!("! Dropping MyBox instance");
    drop(self);
  }
}

impl From<MyBox<&str>> for String {
  fn from(src: MyBox<&str>) -> String {
    String::from(*src)
  }
}

impl From<&MyBox<&str>> for String {
  fn from(src: &MyBox<&str>) -> String {
    String::from(**src)
  }
}

impl From<MyBox<usize>> for String {
  fn from(src: MyBox<usize>) -> String {
    format!("{}", *src)
  }
}

pub fn run() {
  banner::banner("MyBox");
  let mybox = MyBox::new(23);
  println!("My box holds {}", *mybox);

  // Interestingly the println macro does something that means
  // moved value gets printed before the message is printed
  println!("From usize to String: {}", String::from(mybox));

  let mybox = MyBox::new("Bob");
  println!("My new box holds {}", *mybox);
  println!("From borrow to String: {}", String::from(&mybox));
  let s = coerce_str(&mybox);
  println!("Used deref coercion to get {}", s);
  println!("From &str to String: {}", String::from(&mybox));
}

// totally unnecessary but proves the point
fn coerce_str(s: &str) -> &str {
  s
}
