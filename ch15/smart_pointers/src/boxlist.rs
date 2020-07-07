enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}

use List::{Cons, Nil};

pub fn run() {
  let b = Box::new(23);
  print!("b has {} in the box\n", b);

  print!("Printing the list\n");
  let list = Cons(
    "Hello",
    Box::new(Cons("from", Box::new(Cons("boxes", Box::new(Nil))))),
  );
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
}
