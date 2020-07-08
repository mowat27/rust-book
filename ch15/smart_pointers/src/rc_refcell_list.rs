use crate::banner;
use std::cell::RefCell;
use std::rc::Rc;

use List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
  Cons(Rc<RefCell<T>>, Rc<List<T>>),
  Nil,
}

pub fn run() {
  banner::banner("Making mutable cons structures with Rc and RefCell");

  println!("Creating a list");
  let value = Rc::new(RefCell::new(10));
  let a = Rc::new(Cons(value.clone(), Rc::new(Nil)));
  let b = Cons(Rc::new(RefCell::new(20)), a.clone());

  println!("a is {:?}", a);
  println!("b is {:?}", b);

  println!("\nModifying last item in list");
  *value.borrow_mut() += 100;
  println!("a is now {:?}", a);
  println!("b is now {:?}", b);
}
