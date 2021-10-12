// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x); // xの値は{}です
// }

fn main() {
    let x = another_function(5, 6);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) -> i32 {
    x + y
}
