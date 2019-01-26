fn loops(num_times: i32) {
    let mut counter = 0;
    let a = [10, 20, 30, 40, 50];

    loop {
        if counter == num_times {
            break;
        }

        println!("I love crab rangoons!");
        counter += 1;
    }

    while counter < 4 {
        println!("More love!");
        counter += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn main() {
    let x = 10;

    // The condition must be a bool. No type casting like JS or Ruby.
    if x > 5 {
        println!("This will always be true.");
    } else if x == 3 {
        println!("Not getting here.");
    } else {
        println!("We're never getting here.");
    }

    // Ifs behave like Ruby. In Rust, ifs are an expression.
    let y = if x > 12 {
        0 // These expressions have to match type.
    } else {
        3 // This one can't be a string while the other is an int.
    };

    println!("{} y's", y);

    loops(y);
}
