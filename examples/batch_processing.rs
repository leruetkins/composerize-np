/// Example of batch processing multiple docker commands
use composerize_np::composerize;
use std::fs;

fn main() {
    // Multiple docker commands to convert
    let commands = vec![
        ("nginx", "docker run -d -p 80:80 nginx"),
        ("redis", "docker run -d -p 6379:6379 redis:alpine"),
        ("postgres", "docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=secret postgres:14"),
        ("mongodb", "docker run -d -p 27017:27017 -v mongo-data:/data/db mongo:latest"),
    ];

    println!("Converting {} docker commands...\n", commands.len());

    for (name, command) in commands {
        match composerize(command, "", "latest", 2) {
            Ok(yaml) => {
                let filename = format!("docker-compose-{}.yml", name);
                
                // Save to file
                if let Err(e) = fs::write(&filename, &yaml) {
                    eprintln!("Failed to write {}: {}", filename, e);
                } else {
                    println!("✓ Created: {}", filename);
                }
            }
            Err(e) => {
                eprintln!("✗ Failed to convert {}: {}", name, e);
            }
        }
    }

    println!("\nDone! Check the generated docker-compose-*.yml files.");
}
