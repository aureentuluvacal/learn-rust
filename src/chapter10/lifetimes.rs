use std::fmt::Display;

// Can also specify lifetimes for values in a struct.
struct BestQuote<'a> {
    quote: &'a str, // This won't go out of scope until the value it's referencing goes out scope.
}

// Both x and y and whatever we return need to live as long as the lifetime 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let quote = "The really important kind of freedom involves attention and awareness and discipline, and being able truly to care about other people and to sacrifice for them over and over in myriad petty, unsexy ways every day.";
    let b = BestQuote { quote };

    longest_with_an_announcement(string1.as_str(), string2, "YEAH!");
}
