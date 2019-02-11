// Strings and vectors are smart pointers. They own the data that they are
// pointing to. Smart pointers also have metadata like Strings ensuring that
// they're always UTF-8 characters. They also implement the Dref and Drop traits.
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> 
// allows only immutable borrows checked at compile time; RefCell<T> allows
// immutable or mutable borrows checked at runtime.

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil}; // So we can just use Cons and Nil.

fn main() {
    // Let's start with Box, which allows us to store data on the heap,
    // rather than the stack. This is when to use Box:
    //    - When you have a type whose size can’t be known at
    //      compile time and you want to use a value of that
    //      type in a context that requires an exact size
    //    - When you have a large amount of data and you want
    //      to transfer ownership but ensure the data won’t be
    //       copied when you do so
    //    - When you want to own a value and you care only that
    //      it’s a type that implements a particular trait rather
    //      than being of a specific type

    let b = Box::new(5); // We wouldn't really do this but it's an example.
    println!("b = {}", b);

    // Since a cons list has indeterminant size, we can wrap the values of the
    // list in a Box to point to whatever values the list holds. That way the
    // size of the list is the sum of the Boxes and the size of the integers.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
