fn main() {
    // Vectors hold values of the same type that are stored
    // next to each other in memory. This makes them fast.
    // Useful when we have a list of items.
    let mut v: Vec<i32> = Vec::new();
    v.push(100);
    v.push(4);

    // Can also use vec macro that infers the type.
    let macro_v = vec![1, 2, 3];

    match v.get(2) {
        Some(third) => println!("The third element of vector 2 is: {}", third),
        None => println!("Ain't no third here")
    }

    // This will crash the program if the element we're accessing doesn't exist.
    // Also it is an immutable reference to a mutable object. We can't add more
    // values to it.
    let third : &i32 = &macro_v[2]; 

    println!("The third element of vector 2 is: {}", third);
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
} // When a vector goes out of scope, it and its contents are dropped.
