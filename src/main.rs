use std::io;

fn main() {

    println!("Type any text: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    println!("You typed: {}", input);
}
