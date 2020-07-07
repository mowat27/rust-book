use std::ops::Deref;

use crate::banner;

struct MyBox<T> {
  value: T,
}

impl<T> MyBox<T> {
  fn new(value: T) -> MyBox<T> {
    MyBox { value }
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

pub fn run() {
  banner::banner("MyBox");
  let mybox = MyBox::new(23);
  println!("My box holds {}", *mybox);

  let mybox = MyBox::new("Bob");
  println!("My box now holds {}", *mybox);
}
