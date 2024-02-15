// tests/test_main.rs
use coverage_might_work::add; // Import the `add` function from the `coverage_might_work` crate (which includes the `main` module)

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
