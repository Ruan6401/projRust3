use std::io;

fn main() {
    let user: &str = "Student";
    println!("Type any text: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    println!("Hello, {}. You typed: {}", user,input);
}
