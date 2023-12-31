use std::io;

fn main() {
    let mut input = String::new();

    while input.trim() != "stop" {
        println!("Please input your name: ");
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("Hello {}", input);
    }

    println!("Goodbye");
}
