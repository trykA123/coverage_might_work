// tests/second_tests.rs
use coverage_might_work::second;

#[test]
fn test_greet() {
    // Redirect stdout to capture printed output
    let mut output = Vec::new();
    let result = std::io::stdout().read_to_end(&mut output);
    assert_eq!(result.is_ok(), true);
    
    // Call the function
    second::greet();
    
    // Check the output
    assert_eq!(output, b"Greetings from second.rs!\n");
}