use crate::banner;

pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;
    let pct_of_max = self.value as f64 / self.max as f64;

    if pct_of_max >= 1.0 {
      self.messenger.send("Error: quota exceeded");
    } else if pct_of_max >= 0.9 {
      self.messenger.send("Urgent: close to quota");
    } else if pct_of_max >= 0.75 {
      self.messenger.send("Warning: at or over 75% of quota");
    }
  }
}

struct ScreenMessenger {}

impl Messenger for ScreenMessenger {
  fn send(&self, msg: &str) {
    println!("{}", msg);
  }
}

pub fn run() {
  banner::banner("Limit tracker");
  let messenger = ScreenMessenger {};
  let mut tracker = LimitTracker::new(&messenger, 10);

  for i in 0..11 {
    tracker.set_value(i);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
      let mut vec = self.sent_messages.borrow_mut();
      vec.push(String::from(msg));

      // This would complile but panic at runtime
      // let mut _vec2 = self.sent_messages.borrow_mut();
    }
  }

  #[test]
  fn messages_sent() {
    let messenger = MockMessenger::new();
    let mut tracker = LimitTracker::new(&messenger, 10);
    for i in 0..=10 {
      tracker.set_value(i);
    }
    assert_eq!(3, messenger.sent_messages.borrow().len());
  }
}
