fn example_function() {
    println!("Bacon");

    another_function(37);
}

// Function with a typed argument.
fn another_function(x: i32) {
    println!("{} crab rangoons", x);
}

fn main() {
    example_function();
}
