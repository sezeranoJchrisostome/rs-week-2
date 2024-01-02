fn own_vec(mut vector: Vec<i32>) {
    let mut my_new_vec = Vec::new();
    my_new_vec.push(1);
    println!("{:?}", my_new_vec);
}

fn borrow_vec(mut vector: &Vec<i32>) -> Vec<&i32> {
    let mut my_new_vec = Vec::new();
    for values in vector {
        my_new_vec.push(values);
    }
    my_new_vec.push(&1);
    println!("{:?}", my_new_vec);
    my_new_vec
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}


fn borrow_string(s: &String) {
    println!("{}", s);
}
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    // this compiles no problem!
    own_integer(my_int);
    println!("{}", my_int);
    
    borrow_string(&my_string);

    own_string(my_string); // take ownership of my_string
    // this is using my_string which has also moved and is invalid
    // println!("{:?}", my_string); // this will not compile!


    borrow_vec(&my_vec);

    own_vec(my_vec);
    // but this is using my_vec which was borrowed (moved) and yet is now invalid
    // println!("{:?}", new_vec); // this will not compile!

}
