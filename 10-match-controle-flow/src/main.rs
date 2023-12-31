use std::io;

fn main() {
    println!("Please, enter a greeting!");

    let mut greeting = String::new();
    io::stdin().read_line(&mut greeting).expect("Failed to read line");

    greeting = greeting.trim().to_lowercase();
    match greeting.as_str() {
        "hello" => println!("Hello there!"),
        "hi" => println!("Hi there!"),
        _ => println!("I don't know what you mean :("),
    }
}
