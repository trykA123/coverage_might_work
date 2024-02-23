// second.rs

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
        std::io::stdout().flush().unwrap(); // Flush to make sure previous output is captured
        let result = std::io::stdout().lock().set(&mut output);
        greet();
        assert_eq!(String::from_utf8(output).unwrap(), "Greetings from second.rs!\n");
        // Restore stdout
        std::io::stdout().flush().unwrap();
        std::io::stdout().lock().set(result.unwrap());
    }
}
