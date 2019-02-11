// RefCell<T> enforces the borrowing rules at runtime. If there is a break
// in the rules, then the program panics and exits. Because RefCell<T> allows
// mutable borrows checked at runtime, you can mutate the value inside the
// RefCell<T> even when the RefCell<T> is immutable (the interior immutability
// pattern)

#![allow(dead_code)]

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
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

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger.send(
                    "Urgent warning: You've used up over 90% of your quota!",
                );
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    // Rc be combined with Refcell to have a mutatable value with multiple owners
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// And in lib.rs

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::RefCell;

//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger { sent_messages: RefCell::new(vec![]) }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//              self.sent_messages.borrow_mut().push(String::from(message));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

//         limit_tracker.set_value(80);

//         assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
//     }
// }
