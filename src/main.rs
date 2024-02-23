// main.rs

mod second;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(3, 4);
    println!("Result: {}", result);
    second::greet();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(-1, -1), -2);
    }
}
