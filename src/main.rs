mod second;

// src/main.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(3, 4);
    println!("Result: {}", result);
    second::greet();
}
