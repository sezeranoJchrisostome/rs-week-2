fn main() {
    // with range
    println!("with range");
    for x in 1..=10 {
        println!("x is {}", x);
    }

    println!("with range and rev");
    for x in (1..=10).rev() {
        println!("x is {}", x);
    }

    println!("loop over an array");
    let arr = [1, 2, 3, 4, 5];
    for x in arr {
        println!("x is {}", x);
    }
}
