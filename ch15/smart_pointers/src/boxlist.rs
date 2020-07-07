use crate::banner;

enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}

use List::{Cons, Nil};

pub fn run() {
  banner::banner("Cons lists");
  println!("Make a box");
  let b = Box::new(23);
  println!("b has {} in the box\n", b);

  print!("Make a list");
  let list = Cons(
    "Hello",
    Box::new(Cons("from", Box::new(Cons("boxes", Box::new(Nil))))),
  );

  println!("Print the list:");
  let mut item = list;
  loop {
    match item {
      Cons(val, next) => {
        println!("  {}", val);
        item = *next;
      }
      Nil => break,
    }
  }
  println!("");
}
