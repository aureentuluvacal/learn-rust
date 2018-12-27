fn example_function() {
    println!("Bacon");

    another_function(37);
    println!("{} more crab rangoons!", yet_another_function());
}

// Function with a typed argument.
fn another_function(x: i32) {
    let y = {
        let z = 4; // statement, doesn't return a value, ends in semicolon
        x + z + 1 // expression, returns a value
    };

    println!("{} crab rangoons", y);
}

// Can specify the type with an arrow after the arguments.
fn yet_another_function() -> i32 {
    5 // Just like Ruby there's an implied return in a function like this.
}

fn main() {
    example_function();
}
