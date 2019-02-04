// Drop lets us customize what happens when a value goes out of scope.
// Pointers implement Drop and when they go out of scope, the data
// they are pointing to also goes out of scope, for example.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    // We can use drop(c) to force values to be dropped early, but it isn't common to do.

    println!("CustomSmartPointers created");
} // drop is automatically called on CustomSmartPointer here where c and d go out of scope.
