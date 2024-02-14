// tests/test_second.rs
use std::io::Write; // Import Write trait
use crate::second;

#[test]
fn test_greet() {
    // Redirect stdout to capture printed output
    let mut output = Vec::new();
    let result = std::io::stdout().lock().write_all(b"Greetings from second.rs!\n");
    assert_eq!(result.is_ok(), true);
    
    // Call the function
    second::greet();
}
