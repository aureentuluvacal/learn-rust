// Just gonna rip out a paragraph from the book:
//
// "Some languages have garbage collection that constantly looks for no longer used
// memory as the program runs; in other languages, the programmer must explicitly
// allocate and free the memory. Rust uses a third approach: memory is managed
// through a system of ownership with a set of rules that the compiler checks at
// compile time."
//
// The first approach is like JS, the second like C, while Rust introduces ownership.
fn count_rangoons(num_rangoons: i32) {
    // num_rangoons is in this function's scope. It is stored on the stack when it's
    // scope then popped off when it is no longer in scope. This behavior is mirrored
    // in the other immutable primitive data types.
    println!("{} rangoons", num_rangoons);
}

fn main() {
    let mut s = String::from("something");
    s.push_str(", world!");

    println!("{}", s);
    count_rangoons(4);
}
