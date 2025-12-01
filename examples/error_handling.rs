/// Example demonstrating error handling
use composerize_np::composerize;

fn main() {
    // Example 1: Valid command
    println!("=== Example 1: Valid Command ===");
    let valid_command = "docker run -p 80:80 nginx";
    
    match composerize(valid_command, "", "latest", 2) {
        Ok(yaml) => println!("Success!\n{}", yaml),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // Example 2: Missing image
    println!("=== Example 2: Missing Image (Error) ===");
    let invalid_command = "docker run -d -p 80:80";
    
    match composerize(invalid_command, "", "latest", 2) {
        Ok(yaml) => println!("Success!\n{}", yaml),
        Err(e) => eprintln!("Expected error: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // Example 3: Invalid format
    println!("=== Example 3: Invalid Format (Error) ===");
    let command = "docker run nginx";
    
    match composerize(command, "", "invalid-format", 2) {
        Ok(yaml) => println!("Success!\n{}", yaml),
        Err(e) => eprintln!("Expected error: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // Example 4: Proper error handling in production code
    println!("=== Example 4: Production-style Error Handling ===");
    
    fn convert_with_fallback(command: &str) -> String {
        composerize(command, "", "latest", 2)
            .unwrap_or_else(|e| {
                eprintln!("Conversion failed: {}", e);
                // Return a default compose file
                "services:\n  default:\n    image: alpine\n".to_string()
            })
    }
    
    let result = convert_with_fallback("docker run nginx");
    println!("Result:\n{}", result);
}
