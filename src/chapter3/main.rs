fn main() {
    // An immutable variable.
    let a = 2;
    println!("The of a is: {}", a);

    // A mutable variable.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // A constant, which requires a type annotation.
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let a = 4;
    println!("The shadowed value of a is: {}", a);

    let floating_64 = 90.4;
    println!("The value of floating_64 is: {}", floating_64);

    let floating_32: f32 = 12.4;
    println!("The value of floating_32 is: {}", floating_32);

    // A char. A Rust char represents Unicode Scalar Value.
    let z = 'ℤ';
    println!("The value of z is: {}", z);
}
