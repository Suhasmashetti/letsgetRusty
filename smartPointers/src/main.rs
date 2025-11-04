use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// Reference Counting (multiple ownership but can mutate the value)
// fn main () {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let _b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let _c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }


//refcell (multiple ownership but can mutate the value)
// fn main () {
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::clone(&value);
//     let b = Rc::clone(&value);

//     *a.borrow_mut() = 10;
//     *b.borrow_mut() = 20;

//     println!("a = {:?}", a);
//     println!("b = {:?}", b);
//     println!("value = {:?}", value);

// }

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
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
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
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
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(101);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

fn main() {}