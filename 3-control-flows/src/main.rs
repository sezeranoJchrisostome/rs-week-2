fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    if height < 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }

    let age = 13;
    if age > 18 {
        println!("Adult");
    } else if age > 12 && age < 18 {
        println!("teenager");
    } else {
        println!("Child");
    }
}
