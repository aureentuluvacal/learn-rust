#[derive(Debug)]
enum Plan {
    Enterprise,
    Pro,
    Plus,
    Standard
}

fn generate_bill(plan : Plan) -> u32 {
    match plan {
        Plan::Enterprise => 1000,
        Plan::Pro => 750,
        Plan::Plus => 500,
        Plan::Standard => 250
    }
}

fn main() {
    let plan = Plan::Pro;

    println!("The {:?} plan", Plan::Enterprise);
    println!("The {:?} plan is ${}", Plan::Pro, generate_bill(plan));
}
