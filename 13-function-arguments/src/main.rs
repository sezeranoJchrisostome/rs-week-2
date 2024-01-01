use std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn average(sum: i32, numbers: &[i32]) -> i32 {
    sum / numbers.len() as i32
}

fn main() {
    let mut numbers = Vec::new();
    let mut i = 1;
    while i < 5 {
        let mut input = String::new();

        println!("Please, enter number to sum");

        io::stdin()
            .read_line(&mut input)
            .expect("Sorry, we accept number");

        let input: i32 = input.trim().parse().expect("Please enter a number");

        numbers.push(input);
        i += 1;
    }
    let result = sum(&numbers);
    let avg = average(result, &numbers);
    println!("The sum is {} and average is {}", result, avg);
}
