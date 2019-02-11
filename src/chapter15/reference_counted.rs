// Sometimes values have multiple owners, like a node in a tree. 
// Rc<T> enables multiple owners of the same data; Box<T> and 
// RefCell<T> have single owners.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("count after creating a is {}", Rc::strong_count(&a));

    // Rc::clone makes a shallow copy, increasing the number of references
    // by one, and is more performant.
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b is {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c is {}", Rc::strong_count(&a));
    }
    println!(
        "count after c goes out of scope is {}",
        Rc::strong_count(&a)
    );
}
