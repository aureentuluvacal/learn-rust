use std::ops::Deref;

struct MyBox<T>(T); // heh

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0 // We return a reference because we don't want MyBox to lose ownership of the value.
    }
}

fn main() {
    let x = 5;
    let y = &x; // Assigning y as a pointer to x.
    // Can also use Box to exhibit the same functionality.
    // let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Dereferencing y to get to the value of x. Behind the scenes it's *(y.dref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Deref coercion turns &MyBox<String> into &String.

    // If that didn't happen we'd have to do hello(&(*m)[..])
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
