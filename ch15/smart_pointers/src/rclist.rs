use crate::banner;
use std::rc::Rc;

use List::{Cons, Nil};

pub fn run() {
  banner::banner("Making cons structures with rc");
  let a = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Nil)))));
  let b = Cons(1, Rc::clone(&a));
  let c = Cons(2, Rc::clone(&a));
  println!("a: {:?}", a.vec());
  println!("b: {:?}", b.vec());
  println!("c: {:?}", c.vec());
}

enum List<T> {
  Cons(T, Rc<List<T>>),
  Nil,
}

impl<T> List<T>
where
  T: Copy,
{
  // More efficient to return Vec of refs to T but
  // really just playing around with traits here
  fn vec(&self) -> Vec<T> {
    let mut result = vec![];
    let mut item = self;
    loop {
      match item {
        Cons(val, next) => {
          // val is a <&T> not a <T>
          result.push(*val);
          item = next;
        }
        Nil => break,
      }
    }
    result
  }
}
