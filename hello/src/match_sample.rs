enum Color {
    Red,
    Blue,
    Green,
}

pub fn match_sample() -> () {
    let c = Color::Red;

    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        // Color::Green => println!("Green"),
    }
}
