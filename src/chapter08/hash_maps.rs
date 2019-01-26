use std::collections::HashMap;

fn main() {
    // Data is stored on the heap, like vectors.
    // All keys must have same type and all values
    // must have same type. HashMap will own Strings and
    // copy Copyable types.
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 5);
    scores.insert(String::from("Blue"), 19); // The last insertion for a key will be its value.
    scores.entry(String::from("Blue")).or_insert(50); // Only inserts a balue if the hash has no value for that key.

    let blue_team = String::from("Blue");
    let score = scores.get(&blue_team); // Will return Some(&<T>) or None depending on if the value is present.

    let text = "bacon bacon double cheeseburger";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("The score for the Blue team is {:?}", score);
    println!("{:?}", map);
}
