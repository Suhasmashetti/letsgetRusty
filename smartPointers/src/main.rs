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
fn main () {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    *a.borrow_mut() = 10;
    *b.borrow_mut() = 20;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("value = {:?}", value);

}