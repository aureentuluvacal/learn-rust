enum Color {
  Red,
  Blue,
  Green
}

enum MassEffectEnding {
  Good,
  Terrible(Color)
}

fn what(ending: MassEffectEnding) -> String {
  match ending {
    MassEffectEnding::Good => String::from("Doesn't exist!"),
    MassEffectEnding::Terrible(color) => String::from("The color doesn't matter!")
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // We need to cover EVERY case in a match.
    // _ is for everything else when we don't want to define
    // every single branch.
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let ending = MassEffectEnding::Terrible(Color::Blue);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} and {:?}", six, none);

    println!("Oh wow a Mass Effect ending. {}", what(ending));
}
