/// Example of converting between YAML and JSON formats
use composerize_np::{yaml_to_json, json_to_yaml};

fn main() {
    // Example YAML compose file
    let yaml_content = r#"
services:
  web:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    environment:
      - NODE_ENV=production
    volumes:
      - ./html:/usr/share/nginx/html
    restart: always
"#;

    // Convert YAML to JSON
    println!("=== Original YAML ===");
    println!("{}", yaml_content);
    
    match yaml_to_json(yaml_content, true) {
        Ok(json) => {
            println!("\n=== Converted to JSON ===");
            println!("{}", json);
            
            // Convert back to YAML
            match json_to_yaml(&json) {
                Ok(yaml) => {
                    println!("\n=== Converted back to YAML ===");
                    println!("{}", yaml);
                }
                Err(e) => eprintln!("Error converting to YAML: {}", e),
            }
        }
        Err(e) => eprintln!("Error converting to JSON: {}", e),
    }
}
