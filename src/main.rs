// We can use the Result enum to handle recoverable errors, like trying to write
// to a file that doesn't exist.

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let f = File::open("hello.txt");

    // Handle the error with a match, which isn't as sexy as using closures and unwrap_or_else.
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!(
                    "Tried to create file but there was a problem: {:?}",
                    e
                ),
            },
            other_error => panic!(
                "There was a problem opening the file: {:?}",
                other_error
            ),
        },
    };

    // Or handle by propogating error to the what is calling the code.
}
