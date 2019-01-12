// We can use the Result enum to handle recoverable errors, like trying to write
// to a file that doesn't exist.

use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
