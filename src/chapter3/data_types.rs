fn main() {
    // An immutable variable. All Rust vars are immutable.
    let immutable = 2;
    println!("The of a is: {}", immutable);

    // A mutable variable.
    let mut mutable = 5;
    println!("The value of x is: {}", mutable);
    mutable = 6;
    println!("The value of x is: {}", mutable);

    // A constant, which requires a type annotation.
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let immutable = 4;
    println!("The shadowed value of a is: {}", immutable);

    let floating_64 = 90.4;
    println!("The value of floating_64 is: {}", floating_64);

    let floating_32: f32 = 12.4;
    println!("The value of floating_32 is: {}", floating_32);

    // A char. A Rust char represents Unicode Scalar Value so it can handle more than ASCII.
    let z = 'ℤ';
    println!("The value of z is: {}", z);

    // A tuple, a collection of values with a variety of types of fixed size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", y);

    // An array, a collection of fixed length. Data stored on stack and not the heap.
    // Useful when you know the collection should not change. All elements have the same type.
    let array: [u32; 5] = [1, 2, 3, 4, 5];
    println!("The value of the first item in array is: {}", array[0]);
}
