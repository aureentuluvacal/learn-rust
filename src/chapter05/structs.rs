#[derive(Debug)]
struct Food {
    name: String,
    category: String,
    delicious: bool,
}

impl Food {
    // Can define struct methods.
    fn print_food(&self) {
        println!("Nothing is more delicious than {}", self.name);
    }

    fn better_than(&self, other: &Food) {
        if self.delicious && !other.delicious {
            println!("{} is better than {}", self.name, other.name);
        } else if !self.delicious && other.delicious {
            println!("{} is better than {}", self.name, other.name);
        } else {
            println!("{} is about the same as {}", self.name, other.name);
        }
    }

    // And associated functions, like String::from().
    fn delicious_food(name: String, category: String) -> Food {
        Food {
            name,
            category,
            delicious: true,
        }
    }
}

#[derive(Debug)]
struct Point(i32, i32, i32);

fn create_food(name: String, category: String, delicious: bool) -> Food {
    Food {
        name,
        category,
        delicious,
    }
}

fn main() {
    // This entire instance is mutable. We cannot specify certain fields as mutable.
    let food1 = create_food(String::from("bacon"), String::from("meat"), true);
    let food2 = create_food(String::from("ham"), String::from("meat"), false);
    let good_food =
        Food::delicious_food(String::from("kale"), String::from("vegetable"));
    let point = Point(34, -9, 10);

    // Can copy values from struct to struct using "..struct_to_copy"
    // let food2 = Food {
    //     name: String::from("Pork Chop"),
    //     ..food1
    // };

    println!("This food is: {:?}", food1);
    println!("This food is: {:?}", good_food);
    food1.print_food();
    food1.better_than(&food2);
    println!("This point has coordinates: {:?}", point);
}
