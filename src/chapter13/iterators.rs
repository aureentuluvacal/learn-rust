fn main() {
    let vec = vec![1, 10, -7];

    // Rust iterators and lazy and do nothing until they're used.
    let vec_iter = vec.iter();

    // Like in a for loop.
    for val in vec_iter {
        println!("Got: {}", val);
    } // vec_iter goes out of scope here.

    let vec_iter2 = vec.iter();

    // We can use methods that consume the iterator like sum().
    let sum: i32 = vec_iter2.sum(); // sum takes ownership of the iterator.

    println!("And here's the sum: {}", sum);

    // We can also use methods that produce other iterators, called iterator adaptors.
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // Iterators are lazy and must be consumed.

    println!("Mapped values: {:?}", v2);

    let counter = Counter::new();
}

// We can also implement our own iterators.
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
