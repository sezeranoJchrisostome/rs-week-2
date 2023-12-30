fn main() {
    let mut height = 190;
    height = height - 20;

    let result = if height > 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };

    println!("The result is {}", result);

    let health = if height > 180 {
        "healthy"
    } else {
        "unhealthy"
    };
    println!("The health is {}", health);

    // shadowing to a deffent type, this is not the best practice to do so you must have a strong idea why you are doing this
    let health = if height > 180 {
        true
    } else {
        false
    };
}
