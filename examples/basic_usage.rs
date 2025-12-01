/// Basic example of using composerize-np as a library
use composerize_np::{composerize, composerize_to_json};

fn main() {
    // Example 1: Convert docker run to YAML
    let docker_command = "docker run -d -p 80:80 --name web nginx:alpine";
    
    match composerize(docker_command, "", "latest", 2) {
        Ok(yaml) => {
            println!("=== YAML Output ===");
            println!("{}", yaml);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // Example 2: Convert docker run to JSON
    let docker_command = "docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=secret mysql:8";
    
    match composerize_to_json(docker_command, "", "latest", 2) {
        Ok(json) => {
            println!("=== JSON Output ===");
            println!("{}", json);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // Example 3: Complex command with volumes and networks
    let docker_command = "docker run -d \
        --name ml-service \
        --network ml-net \
        -v ml-models:/models \
        -v ml-cache:/cache \
        -p 5000:5000 \
        -e MODEL_PATH=/models \
        tensorflow/tensorflow:latest-gpu";
    
    match composerize(docker_command, "", "v3x", 2) {
        Ok(yaml) => {
            println!("=== Complex Example (v3x) ===");
            println!("{}", yaml);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
