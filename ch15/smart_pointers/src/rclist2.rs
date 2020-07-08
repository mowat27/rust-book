// Simplified example so I could ask on discord

use std::rc::Rc;

use List::{Cons, Nil};

enum List<T> {
  Cons(T, Rc<List<T>>),
  Nil,
}

impl<T> List<T> {
  fn vec(&self) -> Vec<&T> {
    let mut result = vec![];
    let mut item = self;
    loop {
      match item {
        Cons(val, next) => {
          // why is val a &T not a T?
          result.push(val);
          item = next;
        }
        Nil => break,
      }
    }
    result
  }
}

pub fn run() {
  let a = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Nil)))));
  println!("List a: {:?}", a.vec());
}
