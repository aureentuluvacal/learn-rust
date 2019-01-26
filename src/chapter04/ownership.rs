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
    // in the other primitive data types.
    println!("{} rangoons", num_rangoons);
}

fn print_something() {
    let mut s = String::from("something"); // Once this var goes out of scope it is automatically freed from memory by Rust.

    // Rust makes shallow copies of variables whose contents are stored in the heap.
    // So s1 and s2 would point to the same content in the heap and once this function
    // is over, both s1 and s2 would be freed from memory, causing an error. Rust
    // prevents this by invalidating s. We can use clone() to make a deep copy.
    // let s2 = s;
    s.push_str(", world!");

    println!("{}", s);
} // This is when s goes out of scope.

fn main() {
    let x = 4; // x is in scope
    print_something();
    count_rangoons(x); // x is still in scope because it's an integer, whose size is know at compile time, so a copy is made.
} // x goes out of scope.
