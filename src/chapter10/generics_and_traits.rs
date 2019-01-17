// Can define a struct to use any type. The type that we eventually
// in the fields of the struct has to be the same type if only one
// type parameter is passed.
struct Something<T, U> {
    something: T,
    something_else: U,
}

// Can do the same with enums.
enum Option<T> {
    Some(T),
    None,
}

struct Course {
    name: String,
    subject: String,
    credits: u32,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Course {
    fn summarize(&self) -> String {
        format!(
            "{}, [{}], {} credits",
            self.name, self.subject, self.credits
        )
    }
}

fn enroll(enrollable: impl Summary) {
    println!("You have enrolled in: {}", enrollable.summarize());
}

// The following two functions differ only in parameter type
// and return type.
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = generic_largest(&char_list);
    println!("The largest char is {}", result);

    let course = Course {
        name: String::from("Calculus III"),
        subject: String::from("Mathematics"),
        credits: 4,
    };

    enroll(course);
}
