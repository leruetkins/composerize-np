use clap::{Parser, Subcommand};
use composerize_np::{composerize, composerize_to_json, convert_file, json_to_yaml, yaml_to_json};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "composerize-np")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Convert docker run commands to docker-compose files and convert between YAML/JSON")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Docker run command to convert (legacy mode, use 'run' subcommand instead)
    #[arg(value_name = "COMMAND", conflicts_with = "command", conflicts_with = "from_file")]
    docker_command: Vec<String>,

    /// Docker Compose format (v2x, v3x, latest)
    #[arg(short, long, default_value = "latest")]
    format: String,

    /// Number of spaces for indentation
    #[arg(short, long, default_value_t = 2)]
    indent: usize,

    /// Save to file (default: docker-compose.yml)
    #[arg(short, long, value_name = "FILE", num_args = 0..=1, default_missing_value = "docker-compose.yml", require_equals = false)]
    output: Option<PathBuf>,

    /// Output format: yaml or json
    #[arg(long, default_value = "yaml")]
    output_format: String,

    /// Read docker command from file
    #[arg(long, value_name = "FILE", conflicts_with = "command", conflicts_with = "docker_command")]
    from_file: Option<PathBuf>,
}



#[derive(Subcommand)]
enum Commands {
    /// Convert docker run command to compose file
    Run {
        /// Docker run command to convert
        #[arg(value_name = "COMMAND")]
        docker_command: Vec<String>,

        /// Docker Compose format (v2x, v3x, latest)
        #[arg(short, long, default_value = "latest")]
        format: String,

        /// Number of spaces for indentation
        #[arg(short, long, default_value_t = 2)]
        indent: usize,

        /// Save to file (default: docker-compose.yml)
        #[arg(short, long, value_name = "FILE", num_args = 0..=1, default_missing_value = "docker-compose.yml", require_equals = false)]
        output: Option<PathBuf>,

        /// Output format: yaml or json
        #[arg(long, default_value = "yaml")]
        output_format: String,

        /// Read docker command from file
        #[arg(long, value_name = "FILE", conflicts_with = "docker_command")]
        from_file: Option<PathBuf>,
    },

    /// Convert YAML to JSON
    YamlToJson {
        /// Input YAML file
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output JSON file (optional, prints to stdout if not specified)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Pretty print JSON
        #[arg(short, long, default_value_t = true)]
        pretty: bool,
    },

    /// Convert JSON to YAML
    JsonToYaml {
        /// Input JSON file
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output YAML file (optional, prints to stdout if not specified)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
    },

    /// Convert between formats (auto-detect)
    Convert {
        /// Input file (YAML or JSON)
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output file
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    // Legacy mode: direct command without subcommand
    if !cli.docker_command.is_empty() || cli.from_file.is_some() {
        let cmd = if let Some(file_path) = cli.from_file {
            match fs::read_to_string(&file_path) {
                Ok(content) => vec![content.trim().to_string()],
                Err(e) => {
                    eprintln!("Error reading file {}: {}", file_path.display(), e);
                    std::process::exit(1);
                }
            }
        } else {
            cli.docker_command
        };
        
        handle_docker_run(cmd, cli.format, cli.indent, cli.output, cli.output_format);
        return;
    }

    // Subcommand mode
    match cli.command {
        Some(Commands::Run {
            docker_command,
            format,
            indent,
            output,
            output_format,
            from_file,
        }) => {
            let cmd = if let Some(file_path) = from_file {
                match fs::read_to_string(&file_path) {
                    Ok(content) => vec![content.trim().to_string()],
                    Err(e) => {
                        eprintln!("Error reading file {}: {}", file_path.display(), e);
                        std::process::exit(1);
                    }
                }
            } else {
                docker_command
            };
            handle_docker_run(cmd, format, indent, output, output_format);
        }
        Some(Commands::YamlToJson {
            input,
            output,
            pretty,
        }) => {
            handle_yaml_to_json(&input, output.as_deref(), pretty);
        }
        Some(Commands::JsonToYaml { input, output }) => {
            handle_json_to_yaml(&input, output.as_deref());
        }
        Some(Commands::Convert { input, output }) => {
            handle_convert(&input, &output);
        }
        None => {
            println!("composerize-np v{} - Convert docker run commands to docker-compose files\n", env!("CARGO_PKG_VERSION"));
            
            println!("QUICK EXAMPLES:");
            println!("  # Convert docker run to YAML");
            println!("  composerize-np \"docker run -p 80:80 nginx\"\n");
            
            println!("  # Save to file (docker-compose.yml)");
            println!("  composerize-np \"docker run -p 80:80 nginx\" -o\n");
            
            println!("  # Convert to JSON");
            println!("  composerize-np \"docker run -p 80:80 nginx\" --output-format json -o\n");
            
            println!("  # Read from file (for long/complex commands)");
            println!("  composerize-np --from-file command.txt -o\n");
            
            println!("  # Convert YAML to JSON");
            println!("  composerize-np yaml-to-json docker-compose.yml -o output.json\n");
            
            println!("  # Convert JSON to YAML");
            println!("  composerize-np json-to-yaml docker-compose.json -o output.yml\n");
            
            println!("USAGE:");
            println!("  composerize-np [OPTIONS] \"<DOCKER_COMMAND>\"");
            println!("  composerize-np <SUBCOMMAND>\n");
            
            println!("OPTIONS:");
            println!("  -o, --output <FILE>        Save to file (default: docker-compose.yml)");
            println!("  --output-format <FORMAT>   Output format: yaml or json [default: yaml]");
            println!("  -f, --format <VERSION>     Compose version: latest, v3x, v2x [default: latest]");
            println!("  -i, --indent <NUM>         Indentation spaces [default: 2]");
            println!("  --from-file <FILE>         Read docker command from file");
            println!("  -h, --help                 Print help\n");
            
            println!("SUBCOMMANDS:");
            println!("  run           Convert docker run command to compose file");
            println!("  yaml-to-json  Convert YAML to JSON");
            println!("  json-to-yaml  Convert JSON to YAML");
            println!("  convert       Auto-detect and convert between formats");
            println!("  help          Print this message or the help of the given subcommand(s)\n");
            
            println!("For detailed help on any subcommand:");
            println!("  composerize-np <SUBCOMMAND> --help\n");
            
            println!("Examples with options:");
            println!("  composerize-np \"docker run nginx\" -f v3x -o compose.yml");
            println!("  composerize-np --from-file cmd.txt --output-format json -o");
            
            std::process::exit(0);
        }
    }
}

fn handle_docker_run(
    docker_command: Vec<String>,
    format: String,
    indent: usize,
    output: Option<PathBuf>,
    output_format: String,
) {
    let command = docker_command.join(" ");

    let existing_compose = if atty::isnt(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_default();
        buffer
    } else {
        String::new()
    };

    let result = if output_format == "json" {
        composerize_to_json(&command, &existing_compose, &format, indent)
    } else {
        composerize(&command, &existing_compose, &format, indent)
    };

    match result {
        Ok(output_content) => {
            if let Some(mut output_path) = output {
                // If path is default name docker-compose.yml but format is JSON,
                // change extension to .json
                if output_path.to_str() == Some("docker-compose.yml") && output_format == "json" {
                    output_path = PathBuf::from("docker-compose.json");
                }

                match fs::File::create(&output_path) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(output_content.as_bytes()) {
                            eprintln!("Error writing to file: {}", e);
                            std::process::exit(1);
                        }
                        println!("Successfully written to {}", output_path.display());
                    }
                    Err(e) => {
                        eprintln!("Error creating file {}: {}", output_path.display(), e);
                        std::process::exit(1);
                    }
                }
            } else {
                println!("{}", output_content);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nTip: Make sure to quote the entire docker command:");
            eprintln!("  composerize-np \"docker run -p 80:80 nginx\"");
            eprintln!("\nFor complex commands with special characters, consider:");
            eprintln!("  1. Using single quotes on Linux/Mac");
            eprintln!("  2. Escaping special characters");
            eprintln!("  3. Saving command to a file and reading it");
            std::process::exit(1);
        }
    }
}

fn handle_yaml_to_json(input: &Path, output: Option<&Path>, pretty: bool) {
    let content = match fs::read_to_string(input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", input.display(), e);
            std::process::exit(1);
        }
    };

    let json = match yaml_to_json(&content, pretty) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Error converting YAML to JSON: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(output_path) = output {
        match fs::write(output_path, json) {
            Ok(_) => println!("Successfully written to {}", output_path.display()),
            Err(e) => {
                eprintln!("Error writing file {}: {}", output_path.display(), e);
                std::process::exit(1);
            }
        }
    } else {
        println!("{}", json);
    }
}

fn handle_json_to_yaml(input: &Path, output: Option<&Path>) {
    let content = match fs::read_to_string(input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", input.display(), e);
            std::process::exit(1);
        }
    };

    let yaml = match json_to_yaml(&content) {
        Ok(y) => y,
        Err(e) => {
            eprintln!("Error converting JSON to YAML: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(output_path) = output {
        match fs::write(output_path, yaml) {
            Ok(_) => println!("Successfully written to {}", output_path.display()),
            Err(e) => {
                eprintln!("Error writing file {}: {}", output_path.display(), e);
                std::process::exit(1);
            }
        }
    } else {
        println!("{}", yaml);
    }
}

fn handle_convert(input: &Path, output: &Path) {
    match convert_file(input, output, output.extension().and_then(|s| s.to_str()).unwrap_or("")) {
        Ok(_) => println!(
            "Successfully converted {} to {}",
            input.display(),
            output.display()
        ),
        Err(e) => {
            eprintln!("Error converting file: {}", e);
            std::process::exit(1);
        }
    }
}


