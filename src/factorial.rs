// src/factorial.rs

/// Calculates the factorial of a given number.
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
