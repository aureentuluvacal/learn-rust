fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i; // We were returning the location of the first space but it's not helpful. So we're gonna use String slices.
            return &s[0..i];
        }
    }

    return &s[..];
}

fn main() {
    let string = String::from("bacon and cheese");
    let string2 = "crab rangoon";

    let first = first_word(&string);
    let first2 = first_word(&string2);

    println!("The first word in '{}' is '{}'", string, first);
    println!("The first word in '{}' is '{}'", string2, first2);
}
