// Rust doesn’t have exceptions. Instead, it has the type Result<T, E>
// for recoverable errors and the panic! macro that stops execution
// when the program encounters an unrecoverable error.

fn main() {
    // For errors that happen that are unrecoverable Rust uses the panic! macro.
    // panic!("crash and burn");
    let v = vec![1, 2, 3];

    v[99];
}
