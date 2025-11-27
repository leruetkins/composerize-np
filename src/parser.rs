use crate::mappings::{get_mappings, strip_quotes, parse_key_value_list, is_boolean_flag, ArgType};
use indexmap::IndexMap;
use serde_yaml::Value;

pub fn parse_docker_command(input: &str) -> Result<(String, Vec<String>, IndexMap<String, Vec<String>>), String> {
    // Handle bash-style line continuation (backslash + newline)
    let cleaned = input
        .trim()
        .replace("\\\n", " ")  // Bash-style: \ + newline
        .replace("\\\r\n", " ") // Windows-style: \ + CRLF
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    // Remove docker/podman run/create
    let re = regex::Regex::new(r"^(?:docker|podman)\s+(?:run|create|container\s+run|service\s+create)\s+")
        .map_err(|e| format!("Regex error: {}", e))?;
    
    let cleaned = re.replace(&cleaned, "").to_string();
    
    let mut args: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut positional = Vec::new();
    let mut tokens: Vec<String> = Vec::new();
    
    // Improved tokenizer with escaping support
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';
    let mut chars = cleaned.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                // Escaping - add next character as is
                if let Some(next_ch) = chars.next() {
                    current.push(next_ch);
                }
            }
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
                current.push(ch);
            }
            c if c == quote_char && in_quotes => {
                in_quotes = false;
                current.push(ch);
            }
            c if c.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }
    
    let mut i = 0;
    
    while i < tokens.len() {
        let token = &tokens[i];
        
        if token.starts_with("--") {
            let flag_part = token.trim_start_matches("--");
            
            // Check for --flag=value format
            if flag_part.contains('=') {
                let parts: Vec<&str> = flag_part.splitn(2, '=').collect();
                if parts.len() == 2 {
                    args.entry(parts[0].to_string())
                        .or_insert_with(Vec::new)
                        .push(strip_quotes(parts[1]));
                }
                i += 1;
            } else if is_boolean_flag(flag_part) {
                // Boolean flag
                args.entry(flag_part.to_string())
                    .or_insert_with(Vec::new)
                    .push("true".to_string());
                i += 1;
            } else if i + 1 < tokens.len() && !tokens[i + 1].starts_with('-') {
                // Flag with value via space
                args.entry(flag_part.to_string())
                    .or_insert_with(Vec::new)
                    .push(strip_quotes(&tokens[i + 1]));
                i += 2;
            } else {
                // Unknown flag without value - treat as boolean
                args.entry(flag_part.to_string())
                    .or_insert_with(Vec::new)
                    .push("true".to_string());
                i += 1;
            }
        } else if token.starts_with('-') && token.len() > 1 && !token.chars().nth(1).unwrap().is_numeric() {
            let flags = token.trim_start_matches('-');
            
            // If it's a single character, check for value
            if flags.len() == 1 {
                if i + 1 < tokens.len() && !tokens[i + 1].starts_with('-') {
                    args.entry(flags.to_string())
                        .or_insert_with(Vec::new)
                        .push(strip_quotes(&tokens[i + 1]));
                    i += 2;
                } else {
                    args.entry(flags.to_string())
                        .or_insert_with(Vec::new)
                        .push("true".to_string());
                    i += 1;
                }
            } else {
                // Multiple boolean flags (e.g., -it)
                for flag_char in flags.chars() {
                    args.entry(flag_char.to_string())
                        .or_insert_with(Vec::new)
                        .push("true".to_string());
                }
                i += 1;
            }
        } else {
            // This is image or command
            positional.push(strip_quotes(token));
            i += 1;
            
            // Everything else is command
            while i < tokens.len() {
                positional.push(strip_quotes(&tokens[i]));
                i += 1;
            }
            break;
        }
    }
    
    let image = positional.first().ok_or("No image specified")?.clone();
    let command = positional.into_iter().skip(1).collect();
    
    Ok((image, command, args))
}

pub fn build_compose_value(
    args: &IndexMap<String, Vec<String>>,
    network: &str,
) -> Result<Value, String> {
    let mappings = get_mappings();
    let mut service = serde_yaml::Mapping::new();
    
    for (key, values) in args {
        if let Some(mapping) = mappings.get(key) {
            if mapping.path.is_empty() {
                continue; // Ignore (e.g., --rm, --detached)
            }
            
            for value in values {
                let path = mapping.path.replace("¤network¤", network);
                apply_mapping(&mut service, &path, value, &mapping.arg_type)?;
            }
        }
    }
    
    Ok(Value::Mapping(service))
}

fn apply_mapping(
    service: &mut serde_yaml::Mapping,
    path: &str,
    value: &str,
    arg_type: &ArgType,
) -> Result<(), String> {
    let parts: Vec<&str> = path.split('/').collect();
    
    match arg_type {
        ArgType::Array => {
            set_nested_array(service, &parts, value);
        }
        ArgType::Switch => {
            let bool_val = value == "true";
            set_nested_value(service, &parts, Value::Bool(bool_val));
        }
        ArgType::Value => {
            // Special handling for healthcheck test
            if parts.len() > 0 && parts[parts.len() - 1] == "test" && parts.contains(&"healthcheck") {
                // Convert to CMD-SHELL format
                let test_array = vec![
                    Value::String("CMD-SHELL".to_string()),
                    Value::String(value.to_string())
                ];
                set_nested_value(service, &parts, Value::Sequence(test_array));
            } else {
                set_nested_value(service, &parts, Value::String(value.to_string()));
            }
        }
        ArgType::IntValue => {
            let int_val = value.parse::<i64>()
                .map_err(|_| format!("Invalid integer: {}", value))?;
            set_nested_value(service, &parts, Value::Number(int_val.into()));
        }
        ArgType::FloatValue => {
            let float_val = value.parse::<f64>()
                .map_err(|_| format!("Invalid float: {}", value))?;
            set_nested_value(service, &parts, Value::Number(serde_yaml::Number::from(float_val)));
        }
        ArgType::Envs => {
            let env_value = if value.contains('=') {
                let parts: Vec<&str> = value.splitn(2, '=').collect();
                format!("{}={}", parts[0], strip_quotes(parts[1]))
            } else {
                value.to_string()
            };
            set_nested_array(service, &parts, &env_value);
        }
        ArgType::Map => {
            let map = parse_key_value_list(value, ',', '=');
            set_nested_map(service, &parts, map);
        }
        ArgType::MapArray => {
            // MapArray is an array of objects (e.g., for --mount)
            // Check mount type
            if value.starts_with("type=tmpfs") {
                // Convert mount format to docker run style for tmpfs
                let tmpfs_value = convert_mount_to_tmpfs(value);
                set_nested_array(service, &["tmpfs"], &tmpfs_value);
            } else if value.starts_with("type=bind") || value.starts_with("type=volume") {
                // Convert mount to short syntax for volumes
                let volume_value = convert_mount_to_volume(value);
                set_nested_array(service, &["volumes"], &volume_value);
            } else {
                // Regular mount (bind, volume) goes to volumes
                set_nested_array(service, &parts, value);
            }
        }
        ArgType::Networks => {
            if value.matches(|c| c == ':').count() == 0 
                && !["host", "bridge", "none"].contains(&value) 
                && !value.starts_with("container:") {
                // Named network
                let mut network_map = IndexMap::new();
                network_map.insert(
                    Value::String(value.to_string()),
                    Value::Mapping(serde_yaml::Mapping::new())
                );
                set_nested_value(service, &["networks"], Value::Mapping(network_map.into_iter().collect()));
            } else {
                // network_mode
                set_nested_value(service, &["network_mode"], Value::String(value.to_string()));
            }
        }
        ArgType::Ulimits => {
            parse_ulimit(service, &parts, value)?;
        }
        ArgType::Gpus => {
            parse_gpus(service, value)?;
        }
        ArgType::DeviceBlockIOConfigRate | ArgType::DeviceBlockIOConfigWeight => {
            // Эти типы обрабатываются в оригинале, но пока упрощаем
            set_nested_value(service, &parts, Value::String(value.to_string()));
        }
    }
    
    Ok(())
}

fn set_nested_value(map: &mut serde_yaml::Mapping, path: &[&str], value: Value) {
    if path.is_empty() {
        return;
    }
    
    if path.len() == 1 {
        map.insert(Value::String(path[0].to_string()), value);
        return;
    }
    
    let key = Value::String(path[0].to_string());
    let nested = map.entry(key.clone())
        .or_insert_with(|| Value::Mapping(serde_yaml::Mapping::new()));
    
    if let Value::Mapping(nested_map) = nested {
        set_nested_value(nested_map, &path[1..], value);
    }
}

fn set_nested_array(map: &mut serde_yaml::Mapping, path: &[&str], value: &str) {
    if path.is_empty() {
        return;
    }
    
    if path.len() == 1 {
        let key = Value::String(path[0].to_string());
        let arr = map.entry(key.clone())
            .or_insert_with(|| Value::Sequence(Vec::new()));
        
        if let Value::Sequence(seq) = arr {
            seq.push(Value::String(value.to_string()));
        }
        return;
    }
    
    let key = Value::String(path[0].to_string());
    let nested = map.entry(key.clone())
        .or_insert_with(|| Value::Mapping(serde_yaml::Mapping::new()));
    
    if let Value::Mapping(nested_map) = nested {
        set_nested_array(nested_map, &path[1..], value);
    }
}

fn set_nested_map(map: &mut serde_yaml::Mapping, path: &[&str], value: IndexMap<String, Value>) {
    if path.is_empty() {
        return;
    }
    
    if path.len() == 1 {
        let key = Value::String(path[0].to_string());
        let existing = map.entry(key.clone())
            .or_insert_with(|| Value::Mapping(serde_yaml::Mapping::new()));
        
        if let Value::Mapping(existing_map) = existing {
            for (k, v) in value {
                existing_map.insert(Value::String(k), v);
            }
        }
        return;
    }
    
    let key = Value::String(path[0].to_string());
    let nested = map.entry(key.clone())
        .or_insert_with(|| Value::Mapping(serde_yaml::Mapping::new()));
    
    if let Value::Mapping(nested_map) = nested {
        set_nested_map(nested_map, &path[1..], value);
    }
}

fn parse_ulimit(map: &mut serde_yaml::Mapping, path: &[&str], value: &str) -> Result<(), String> {
    let parts: Vec<&str> = value.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid ulimit format: {}", value));
    }
    
    let limit_name = parts[0];
    let limit_value = parts[1];
    
    let full_path = format!("{}/{}", path.join("/"), limit_name);
    let full_parts: Vec<&str> = full_path.split('/').collect();
    
    if limit_value.contains(':') {
        let limits: Vec<&str> = limit_value.split(':').collect();
        if limits.len() == 2 {
            let soft = limits[0].parse::<i64>()
                .map_err(|_| format!("Invalid soft limit: {}", limits[0]))?;
            let hard = limits[1].parse::<i64>()
                .map_err(|_| format!("Invalid hard limit: {}", limits[1]))?;
            
            let mut limit_map = IndexMap::new();
            limit_map.insert("soft".to_string(), Value::Number(soft.into()));
            limit_map.insert("hard".to_string(), Value::Number(hard.into()));
            
            set_nested_map(map, &full_parts, limit_map);
        }
    } else {
        let limit = limit_value.parse::<i64>()
            .map_err(|_| format!("Invalid limit: {}", limit_value))?;
        set_nested_value(map, &full_parts, Value::Number(limit.into()));
    }
    
    Ok(())
}

fn convert_mount_to_tmpfs(mount_str: &str) -> String {
    // Converts --mount type=tmpfs,destination=/tmp,tmpfs-size=256m,tmpfs-mode=1777
    // to format /tmp:rw,noexec,nosuid,size=256m
    
    let mut destination = String::new();
    let mut options = Vec::new();
    
    for part in mount_str.split(',') {
        let kv: Vec<&str> = part.splitn(2, '=').collect();
        if kv.len() == 2 {
            match kv[0] {
                "destination" | "target" | "dst" => destination = kv[1].to_string(),
                "tmpfs-size" => options.push(format!("size={}", kv[1])),
                "tmpfs-mode" => {}, // Ignore mode, compose doesn't support it
                "type" => {}, // Skip type
                _ => {}
            }
        }
    }
    
    // Add standard security options
    options.insert(0, "rw".to_string());
    options.insert(1, "noexec".to_string());
    options.insert(2, "nosuid".to_string());
    
    format!("{}:{}", destination, options.join(","))
}

fn convert_mount_to_volume(mount_str: &str) -> String {
    // Parse --mount format: type=bind,source=/path,target=/path,readonly
    let mut source = String::new();
    let mut target = String::new();
    let mut readonly = false;
    
    for part in mount_str.split(',') {
        if let Some(value) = part.strip_prefix("source=") {
            source = value.to_string();
        } else if let Some(value) = part.strip_prefix("target=") {
            target = value.to_string();
        } else if let Some(value) = part.strip_prefix("destination=") {
            target = value.to_string();
        } else if part == "readonly" || part == "ro" {
            readonly = true;
        }
    }
    
    // Convert to short syntax: source:target[:ro]
    if !source.is_empty() && !target.is_empty() {
        if readonly {
            format!("{}:{}:ro", source, target)
        } else {
            format!("{}:{}", source, target)
        }
    } else {
        // If parsing failed, return as is
        mount_str.to_string()
    }
}

fn parse_gpus(map: &mut serde_yaml::Mapping, value: &str) -> Result<(), String> {
    let count_value = if value == "all" {
        Value::String("all".to_string())
    } else {
        Value::Number(value.parse::<i64>()
            .map_err(|_| format!("Invalid GPU count: {}", value))?.into())
    };
    
    let mut device = IndexMap::new();
    device.insert("driver".to_string(), Value::String("nvidia".to_string()));
    device.insert("count".to_string(), count_value);
    device.insert("capabilities".to_string(), Value::Sequence(vec![Value::String("gpu".to_string())]));
    
    let mut devices = IndexMap::new();
    devices.insert("devices".to_string(), Value::Sequence(vec![
        Value::Mapping(device.into_iter().map(|(k, v)| (Value::String(k), v)).collect())
    ]));
    
    let mut reservations = IndexMap::new();
    reservations.insert("reservations".to_string(), Value::Mapping(
        devices.into_iter().map(|(k, v)| (Value::String(k), v)).collect()
    ));
    
    let mut resources = IndexMap::new();
    resources.insert("resources".to_string(), Value::Mapping(
        reservations.into_iter().map(|(k, v)| (Value::String(k), v)).collect()
    ));
    
    set_nested_map(map, &["deploy"], resources);
    
    Ok(())
}
