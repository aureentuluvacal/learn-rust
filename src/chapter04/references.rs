fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // If I wanted to use s1 outside of calculate_length I can pass a reference to s1 instead of returning it from the function.

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // calculate_length borrows the value of the parameter from the outer scope.
    // It never takes ownership of s and, therefore, doesn't drop if from memory once
    // it goes out of scope.

    // References are immutable unless you specify the variable and the reference to the
    // variable as mutable. We can have only one mutable reference to a particular piece
    // of data in a particular scope and can have many immutable references. We cannot have
    // both immutable and mutable references because if data changes, there's no way
    // of synchronizing the value.
    s.len()
}
