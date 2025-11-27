pub mod mappings;
pub mod parser;

use indexmap::IndexMap;
use serde_yaml::Value;
use std::fs;
use std::path::Path;

pub fn composerize(
    input: &str,
    _existing_compose: &str,
    format: &str,
    _indent: usize,
) -> Result<String, String> {
    let (image, command, args) = parser::parse_docker_command(input)?;
    
    let network = args.get("network")
        .or_else(|| args.get("net"))
        .and_then(|v| v.first())
        .map(|s| s.as_str())
        .unwrap_or("default");
    
    let mut service_value = parser::build_compose_value(&args, network)?;
    
    // Add image
    if let Value::Mapping(ref mut map) = service_value {
        map.insert(
            Value::String("image".to_string()),
            Value::String(image.clone())
        );
        
        // Add command if present
        if !command.is_empty() {
            map.insert(
                Value::String("command".to_string()),
                Value::String(command.join(" "))
            );
        }
    }
    
    let service_name = get_service_name(&image);
    
    let mut services = IndexMap::new();
    services.insert(service_name, service_value);
    
    let version = match format {
        "v2x" => Some("2".to_string()),
        "v3x" => Some("3".to_string()),
        "latest" => None,
        _ => return Err(format!("Unknown format: {}", format)),
    };
    
    let mut compose = IndexMap::new();
    
    if let Some(v) = version {
        compose.insert(
            Value::String("version".to_string()),
            Value::String(v)
        );
    }
    
    compose.insert(
        Value::String("services".to_string()),
        Value::Mapping(services.into_iter()
            .map(|(k, v)| (Value::String(k), v))
            .collect())
    );
    
    // Collect used networks and volumes
    let (networks, volumes) = collect_resources(&args);
    
    // Add networks section if present
    if !networks.is_empty() {
        let mut networks_map = IndexMap::new();
        for net in networks {
            if net != "default" && net != "bridge" && net != "host" && net != "none" {
                let mut net_config = IndexMap::new();
                net_config.insert(
                    Value::String("external".to_string()),
                    Value::Bool(true)
                );
                networks_map.insert(net.to_string(), Value::Mapping(net_config.into_iter().collect()));
            }
        }
        if !networks_map.is_empty() {
            compose.insert(
                Value::String("networks".to_string()),
                Value::Mapping(networks_map.into_iter()
                    .map(|(k, v)| (Value::String(k), v))
                    .collect())
            );
        }
    }
    
    // Add volumes section if there are named volumes
    if !volumes.is_empty() {
        let mut volumes_map = IndexMap::new();
        for vol in volumes {
            volumes_map.insert(vol.to_string(), Value::Null);
        }
        compose.insert(
            Value::String("volumes".to_string()),
            Value::Mapping(volumes_map.into_iter()
                .map(|(k, v)| (Value::String(k), v))
                .collect())
        );
    }
    
    let compose_value = Value::Mapping(compose.into_iter().collect());
    
    serde_yaml::to_string(&compose_value)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

/// Collects used networks and named volumes from arguments
fn collect_resources(args: &IndexMap<String, Vec<String>>) -> (Vec<String>, Vec<String>) {
    let mut networks = Vec::new();
    let mut volumes = Vec::new();
    
    // Collect networks
    if let Some(nets) = args.get("network").or_else(|| args.get("net")) {
        for net in nets {
            if !networks.contains(net) {
                networks.push(net.clone());
            }
        }
    }
    
    // Collect named volumes (not bind mounts)
    if let Some(vols) = args.get("volume").or_else(|| args.get("v")) {
        for vol in vols {
            // Named volume if it doesn't start with / or . or ~
            if !vol.starts_with('/') && !vol.starts_with('.') && !vol.starts_with('~') {
                if let Some(vol_name) = vol.split(':').next() {
                    if !volumes.contains(&vol_name.to_string()) {
                        volumes.push(vol_name.to_string());
                    }
                }
            }
        }
    }
    
    (networks, volumes)
}

pub fn get_service_name(image: &str) -> String {
    let name = if image.contains('/') {
        image.split('/').last().unwrap_or(image)
    } else {
        image
    };
    
    let name = if name.contains(':') {
        name.split(':').next().unwrap_or(name)
    } else {
        name
    };
    
    name.to_string()
}

/// Converts docker run command to JSON
pub fn composerize_to_json(
    input: &str,
    existing_compose: &str,
    format: &str,
    indent: usize,
) -> Result<String, String> {
    let (image, command, args) = parser::parse_docker_command(input)?;

    let network = args
        .get("network")
        .or_else(|| args.get("net"))
        .and_then(|v| v.first())
        .map(|s| s.as_str())
        .unwrap_or("default");

    let mut service_value = parser::build_compose_value(&args, network)?;

    if let Value::Mapping(ref mut map) = service_value {
        map.insert(
            Value::String("image".to_string()),
            Value::String(image.clone()),
        );

        if !command.is_empty() {
            map.insert(
                Value::String("command".to_string()),
                Value::String(command.join(" ")),
            );
        }
    }

    let service_name = get_service_name(&image);

    let mut services = IndexMap::new();
    services.insert(service_name, service_value);

    let version = match format {
        "v2x" => Some("2".to_string()),
        "v3x" => Some("3".to_string()),
        "latest" => None,
        _ => return Err(format!("Unknown format: {}", format)),
    };

    let mut compose = IndexMap::new();

    if let Some(v) = version {
        compose.insert(Value::String("version".to_string()), Value::String(v));
    }

    compose.insert(
        Value::String("services".to_string()),
        Value::Mapping(
            services
                .into_iter()
                .map(|(k, v)| (Value::String(k), v))
                .collect(),
        ),
    );

    // Collect used networks and volumes
    let (networks, volumes) = collect_resources(&args);
    
    // Add networks section if present
    if !networks.is_empty() {
        let mut networks_map = IndexMap::new();
        for net in networks {
            if net != "default" && net != "bridge" && net != "host" && net != "none" {
                let mut net_config = IndexMap::new();
                net_config.insert(
                    Value::String("external".to_string()),
                    Value::Bool(true)
                );
                networks_map.insert(net.to_string(), Value::Mapping(net_config.into_iter().collect()));
            }
        }
        if !networks_map.is_empty() {
            compose.insert(
                Value::String("networks".to_string()),
                Value::Mapping(networks_map.into_iter()
                    .map(|(k, v)| (Value::String(k), v))
                    .collect())
            );
        }
    }
    
    // Add volumes section if there are named volumes
    if !volumes.is_empty() {
        let mut volumes_map = IndexMap::new();
        for vol in volumes {
            volumes_map.insert(vol.to_string(), Value::Null);
        }
        compose.insert(
            Value::String("volumes".to_string()),
            Value::Mapping(volumes_map.into_iter()
                .map(|(k, v)| (Value::String(k), v))
                .collect())
        );
    }

    let compose_value = Value::Mapping(compose.into_iter().collect());

    // Convert to JSON
    let json_value: serde_json::Value = serde_yaml::from_value(compose_value)
        .map_err(|e| format!("Failed to convert to JSON: {}", e))?;

    if indent > 0 {
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))
    } else {
        serde_json::to_string(&json_value)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))
    }
}

/// Converts YAML to JSON
pub fn yaml_to_json(yaml_content: &str, pretty: bool) -> Result<String, String> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(yaml_content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    let json_value: serde_json::Value = serde_yaml::from_value(yaml_value)
        .map_err(|e| format!("Failed to convert to JSON: {}", e))?;

    if pretty {
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))
    } else {
        serde_json::to_string(&json_value)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))
    }
}

/// Converts JSON to YAML
pub fn json_to_yaml(json_content: &str) -> Result<String, String> {
    let json_value: serde_json::Value = serde_json::from_str(json_content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let yaml_value: serde_yaml::Value = serde_json::from_value(json_value)
        .map_err(|e| format!("Failed to convert to YAML: {}", e))?;

    serde_yaml::to_string(&yaml_value).map_err(|e| format!("Failed to serialize YAML: {}", e))
}

/// Converts file from one format to another
pub fn convert_file(
    input_path: &Path,
    output_path: &Path,
    output_format: &str,
) -> Result<(), String> {
    let content =
        fs::read_to_string(input_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let input_ext = input_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let result = match (input_ext, output_format) {
        ("yml" | "yaml", "json") => yaml_to_json(&content, true)?,
        ("json", "yml" | "yaml") => json_to_yaml(&content)?,
        _ => {
            return Err(format!(
                "Unsupported conversion: {} to {}",
                input_ext, output_format
            ))
        }
    };

    fs::write(output_path, result).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_nginx() {
        let result = composerize("docker run nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("nginx"));
        assert!(yaml.contains("image: nginx"));
    }

    #[test]
    fn test_with_ports() {
        let result = composerize("docker run -p 80:80 nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("ports:"));
        assert!(yaml.contains("80:80"));
    }

    #[test]
    fn test_with_environment() {
        let result = composerize("docker run -e NODE_ENV=production nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("environment:"));
        assert!(yaml.contains("NODE_ENV=production"));
    }

    #[test]
    fn test_with_volumes() {
        let result = composerize("docker run -v /data:/app nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("volumes:"));
        assert!(yaml.contains("/data:/app"));
    }

    #[test]
    fn test_with_name() {
        let result = composerize("docker run --name my-app nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("container_name: my-app"));
    }

    #[test]
    fn test_with_restart() {
        let result = composerize("docker run --restart always nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("restart: always"));
    }

    #[test]
    fn test_privileged() {
        let result = composerize("docker run --privileged nginx", "", "latest", 2);
        if let Err(e) = &result {
            eprintln!("Error: {}", e);
        }
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("privileged: true"));
    }

    #[test]
    fn test_interactive_tty() {
        let result = composerize("docker run -it ubuntu bash", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("stdin_open: true"));
        assert!(yaml.contains("tty: true"));
        assert!(yaml.contains("command: bash"));
    }

    #[test]
    fn test_memory_limit() {
        let result = composerize("docker run --memory 512m nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("memory: 512m"));
    }

    #[test]
    fn test_cpu_limit() {
        let result = composerize("docker run --cpus 2.5 nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("cpus: 2.5"));
    }

    #[test]
    fn test_multiple_ports() {
        let result = composerize("docker run -p 80:80 -p 443:443 nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("80:80"));
        assert!(yaml.contains("443:443"));
    }

    #[test]
    fn test_multiple_env_vars() {
        let result = composerize("docker run -e VAR1=value1 -e VAR2=value2 nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("VAR1=value1"));
        assert!(yaml.contains("VAR2=value2"));
    }

    #[test]
    fn test_complex_command() {
        let result = composerize(
            "docker run -d -p 8080:80 --name web -e NODE_ENV=production --restart always nginx:alpine",
            "",
            "latest",
            2
        );
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("8080:80"));
        assert!(yaml.contains("container_name: web"));
        assert!(yaml.contains("NODE_ENV=production"));
        assert!(yaml.contains("restart: always"));
        assert!(yaml.contains("image: nginx:alpine"));
    }

    #[test]
    fn test_version_v2x() {
        let result = composerize("docker run nginx", "", "v2x", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("version: '2'") || yaml.contains("version: \"2\""));
    }

    #[test]
    fn test_version_v3x() {
        let result = composerize("docker run nginx", "", "v3x", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("version: '3'") || yaml.contains("version: \"3\""));
    }

    #[test]
    fn test_version_latest_no_version() {
        let result = composerize("docker run nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(!yaml.contains("version:"));
    }

    #[test]
    fn test_get_service_name_simple() {
        assert_eq!(get_service_name("nginx"), "nginx");
    }

    #[test]
    fn test_get_service_name_with_tag() {
        assert_eq!(get_service_name("nginx:alpine"), "nginx");
    }

    #[test]
    fn test_get_service_name_with_registry() {
        assert_eq!(get_service_name("docker.io/library/nginx"), "nginx");
    }

    #[test]
    fn test_get_service_name_with_registry_and_tag() {
        assert_eq!(get_service_name("docker.io/library/nginx:1.21"), "nginx");
    }

    #[test]
    fn test_healthcheck() {
        let result = composerize(
            "docker run --health-cmd 'curl -f http://localhost' --health-interval 30s nginx",
            "",
            "latest",
            2
        );
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("healthcheck:"));
        assert!(yaml.contains("test:"));
        assert!(yaml.contains("interval: 30s"));
    }

    #[test]
    fn test_labels() {
        let result = composerize("docker run -l app=web -l env=prod nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("labels:"));
        assert!(yaml.contains("app=web"));
        assert!(yaml.contains("env=prod"));
    }

    #[test]
    fn test_hostname() {
        let result = composerize("docker run --hostname myhost nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("hostname: myhost"));
    }

    #[test]
    fn test_user() {
        let result = composerize("docker run --user 1000:1000 nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("user: 1000:1000"));
    }

    #[test]
    fn test_workdir() {
        let result = composerize("docker run --workdir /app nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("working_dir: /app"));
    }

    #[test]
    fn test_entrypoint() {
        let result = composerize("docker run --entrypoint /bin/sh nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("entrypoint:"));
        assert!(yaml.contains("/bin/sh"));
    }

    #[test]
    fn test_cap_add() {
        let result = composerize("docker run --cap-add NET_ADMIN nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("cap_add:"));
        assert!(yaml.contains("NET_ADMIN"));
    }

    #[test]
    fn test_dns() {
        let result = composerize("docker run --dns 8.8.8.8 nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("dns:"));
        assert!(yaml.contains("8.8.8.8"));
    }

    #[test]
    fn test_no_image_error() {
        let result = composerize("docker run -d", "", "latest", 2);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No image specified"));
    }

    #[test]
    fn test_invalid_format() {
        let result = composerize("docker run nginx", "", "invalid", 2);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown format"));
    }

    #[test]
    fn test_composerize_to_json() {
        let result = composerize_to_json("docker run -p 80:80 nginx", "", "latest", 2);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("\"nginx\""));
        assert!(json.contains("\"80:80\""));
        assert!(json.contains("\"services\""));
    }

    #[test]
    fn test_yaml_to_json() {
        let yaml = r#"
services:
  nginx:
    image: nginx
    ports:
      - 80:80
"#;
        let result = yaml_to_json(yaml, true);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("\"nginx\""));
        assert!(json.contains("\"80:80\""));
    }

    #[test]
    fn test_json_to_yaml() {
        let json = r#"{
  "services": {
    "nginx": {
      "image": "nginx",
      "ports": ["80:80"]
    }
  }
}"#;
        let result = json_to_yaml(json);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("nginx"));
        assert!(yaml.contains("80:80"));
    }

    #[test]
    fn test_yaml_to_json_to_yaml() {
        let original_yaml = r#"
services:
  nginx:
    image: nginx
    ports:
      - 80:80
"#;
        let json = yaml_to_json(original_yaml, false).unwrap();
        let yaml = json_to_yaml(&json).unwrap();
        assert!(yaml.contains("nginx"));
        assert!(yaml.contains("80:80"));
    }

    #[test]
    fn test_json_with_complex_structure() {
        let result = composerize_to_json(
            "docker run -d -p 8080:80 -e NODE_ENV=production --restart always nginx",
            "",
            "v3x",
            2,
        );
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"3\""));
        assert!(json.contains("\"NODE_ENV=production\""));
        assert!(json.contains("\"restart\""));
        assert!(json.contains("\"always\""));
    }

    #[test]
    fn test_networks_section() {
        let result = composerize("docker run --network ml-net nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("networks:"));
        assert!(yaml.contains("ml-net:"));
        assert!(yaml.contains("external: true"));
    }

    #[test]
    fn test_volumes_section() {
        let result = composerize("docker run -v data:/data -v cache:/cache nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("volumes:"));
        assert!(yaml.contains("data:"));
        assert!(yaml.contains("cache:"));
    }

    #[test]
    fn test_no_volumes_for_bind_mounts() {
        let result = composerize("docker run -v /host:/container nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        // Should not have volumes section for bind mounts
        let lines: Vec<&str> = yaml.lines().collect();
        let volumes_line = lines.iter().position(|&l| l.starts_with("volumes:"));
        assert!(volumes_line.is_none());
    }

    #[test]
    fn test_mixed_volumes() {
        let result = composerize("docker run -v data:/data -v /host:/host nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        // Should have volumes section only for named volume
        assert!(yaml.contains("volumes:"));
        assert!(yaml.contains("data:"));
        // But service volumes should have both
        assert!(yaml.contains("- data:/data"));
        assert!(yaml.contains("- /host:/host"));
    }

    #[test]
    fn test_default_network_not_in_section() {
        let result = composerize("docker run nginx", "", "latest", 2);
        assert!(result.is_ok());
        let yaml = result.unwrap();
        // Should not have networks section for default network
        let lines: Vec<&str> = yaml.lines().collect();
        let networks_line = lines.iter().position(|&l| l.starts_with("networks:"));
        assert!(networks_line.is_none());
    }

    #[test]
    fn test_full_compose_with_resources() {
        let result = composerize(
            "docker run -d --name ml-service --network ml-net -v ml-models:/models -v ml-cache:/cache nginx",
            "",
            "latest",
            2
        );
        assert!(result.is_ok());
        let yaml = result.unwrap();
        // Check all sections
        assert!(yaml.contains("services:"));
        assert!(yaml.contains("networks:"));
        assert!(yaml.contains("volumes:"));
        assert!(yaml.contains("ml-net:"));
        assert!(yaml.contains("ml-models:"));
        assert!(yaml.contains("ml-cache:"));
    }

    #[test]
    fn test_mount_to_volume_conversion() {
        let result = composerize(
            "docker run --mount=type=bind,source=/host/data,target=/container/data,readonly nginx",
            "",
            "latest",
            2
        );
        assert!(result.is_ok());
        let yaml = result.unwrap();
        // Check that mount is converted to short syntax
        assert!(yaml.contains("/host/data:/container/data:ro"));
        // Should not have raw mount string
        assert!(!yaml.contains("type=bind"));
    }

    #[test]
    fn test_mount_without_readonly() {
        let result = composerize(
            "docker run --mount=type=bind,source=/src,target=/dst nginx",
            "",
            "latest",
            2
        );
        assert!(result.is_ok());
        let yaml = result.unwrap();
        assert!(yaml.contains("/src:/dst"));
        assert!(!yaml.contains(":ro"));
    }
}
