#[derive(Debug)]
// Different discrete types of a thing. 
enum Plan {
    Enterprise,
    Pro,
    Plus,
    Standard
}

// Can define structs in enums.
enum Coordinates {
    Cartesian { x: i32, y: i32, z: i32 },
    Spherical { pi: f32, phi: f32, r: i32 }
}

// Can add methods to enums just like structs.
impl Plan {
    fn generate_bill(&self) -> u32 {
        match self {
            Plan::Enterprise => 1000,
            Plan::Pro => 750,
            Plan::Plus => 500,
            Plan::Standard => 250
        }
    }
}

struct Company {
    plan: Plan,
    name: String
}

fn main() {
    let coordinates = Coordinates::Cartesian {
        x: 3, 
        y: -1,
        z: 0
    };
    let company = Company {
        plan: Plan::Pro,
        name: String::from("Crab Rangoon")
    };

    println!("The {:?} plan", Plan::Enterprise);
    println!("Company {} is on the {:?} plan and owes ${}", company.name, company.plan, company.plan.generate_bill());
}
