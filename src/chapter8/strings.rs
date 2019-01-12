fn main() {
    // Strings are not arrays of characters. They are a vector wrapper with each element in a vector
    // an integer value of a character. h would be 104 (encoded as one byte) and З (ze) would be 208 and 151 (encoded as two).
    let mut s = String::from("crab");
    let s2 = " rangoon";
    let s3 = String::from("1 ");
    let s4 = String::from(" feast");

    s.push_str(s2); // This method does not take ownership of its parameters.
    
    println!("{} and{}", s, s2); // So we can still use s2.
    println!("{}", s3 + &s); // Concatenate strings with +. Can only add &str to String. s is deref coerced to &str. 
    
    let combined = format!("{}-{}-{}", s, s2, s4); // Use format macro for more concatting multiple strings.
    println!("{}", combined);
}
