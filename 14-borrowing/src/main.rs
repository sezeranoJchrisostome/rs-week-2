fn own_vec(mut vector: &Vec<i32>) -> Vec<&i32> {
    let mut my_new_vec = Vec::new();
    for values in vector {
        my_new_vec.push(values);
    }
    my_new_vec.push(&1);
    my_new_vec
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    own_integer(my_int);
    println!("{}", my_int);

    own_string(&my_string); 
    println!("{:?}", my_string); // this will not compile!

    let new_vec = own_vec(&my_vec);
    println!("{:?}", new_vec); // this will not compile!
}
