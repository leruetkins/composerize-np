/// Advanced example with custom processing
use composerize_np::{composerize, get_service_name};

fn main() {
    // Example: Process command and extract service name
    let docker_command = "docker run -d -p 8080:80 --name my-app nginx:alpine";
    
    // Extract service name from image
    let image = "nginx:alpine";
    let service_name = get_service_name(image);
    println!("Service name extracted: {}", service_name);
    
    println!("\n{}\n", "=".repeat(50));
    
    // Convert with different formats
    let formats = vec![
        ("latest", "Latest (no version)"),
        ("v3x", "Docker Compose v3.x"),
        ("v2x", "Docker Compose v2.x"),
    ];
    
    for (format, description) in formats {
        println!("=== {} ===", description);
        
        match composerize(docker_command, "", format, 2) {
            Ok(yaml) => {
                // Show first few lines
                let lines: Vec<&str> = yaml.lines().take(5).collect();
                println!("{}", lines.join("\n"));
                println!("...\n");
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    println!("{}\n", "=".repeat(50));
    
    // Example: Different indentation levels
    println!("=== Different Indentation ===");
    
    for indent in [2, 4] {
        println!("\nIndent: {} spaces", indent);
        match composerize("docker run -p 80:80 nginx", "", "latest", indent) {
            Ok(yaml) => {
                let lines: Vec<&str> = yaml.lines().take(4).collect();
                println!("{}", lines.join("\n"));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
