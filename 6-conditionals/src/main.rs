fn main() {
    let maybe_number = Some(42);
    let maybe_number: Option<Option<()>> = None;
    if let Some(number) = maybe_number {
        println!("The number is {:?}", number);
    }
    else {
        println!("The number is None");
    }
}
