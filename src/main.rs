// If we care about only one case then we use if let.
fn main() {
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("Three");
    } else {
        // Can also include an else block which functions the same as
        // a "_" in a match.
        println!("Not three!");
    }
}
