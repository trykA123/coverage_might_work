// second.rs

use std::io::Write;

pub fn greet() {
    println!("Greetings from second.rs!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        // We can capture the output of the function to check if it's correct.
        let mut output = Vec::new();
        // Redirect stdout to capture output
        let mut stdout = std::io::stdout();
        stdout.lock().flush().unwrap(); // Flush to make sure previous output is captured
        greet();
        stdout.lock().write_all(b"Greetings from second.rs!\n").unwrap(); // Writing expected output
        let result = stdout.lock().write_all(&mut output);
        assert!(result.is_ok()); // Ensure writing to output is successful
        assert_eq!(String::from_utf8(output).unwrap(), "Greetings from second.rs!\n"); // Compare actual with expected output
        // Restore stdout
        stdout.lock().flush().unwrap();
    }
}
