// main.rs

mod second;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let result_add = add(3, 4);
    let result_subtract = subtract(7, 4);
    println!("Result of addition: {}", result_add);
    println!("Result of subtraction: {}", result_subtract);
    second::greet();
    second::farewell();
}
