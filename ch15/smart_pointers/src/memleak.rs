// Demonstrates how a memory leak can happen

use crate::banner;
use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil,
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None,
    }
  }
}

pub fn run() {
  banner::banner("Demonstrating a memory leak");
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

  println!("initial strong count of rc is {}", Rc::strong_count(&a));
  println!("a is {:?}", a);
  println!("a next item is {:?}", a.tail());

  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
  println!("strong count of rc after b is {}", Rc::strong_count(&a));
  println!("b next item is {:?}", b.tail());

  println!("adding b at the tail of a tail");
  if let Some(link) = a.tail() {
    // link will be dropped
    *link.borrow_mut() = Rc::clone(&b);
    println!("and dropping it again");
  }
  // but the reference counts remains
  println!("b rc count is now {:?} though", Rc::strong_count(&b));
  // println!("a rc count is now {:?}", Rc::strong_count(&a));

  println!("printing a would cause a stack overflow now");
  // circular reference
  // panics due to stack overflow
  // println!("a is now {:?}", a);
}
