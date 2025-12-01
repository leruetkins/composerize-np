# composerize-np

Rust version of [composerize](https://github.com/magicmark/composerize) - a powerful tool for:
- 🐳 Converting `docker run` commands to `docker-compose.yaml` or JSON files
- 🔄 Converting between YAML ↔ JSON formats
- ⚡ Fast operation, single executable, no dependencies

## Quick Start

```bash
# Simplest way - just pass the docker command
composerize-np "docker run -p 80:80 nginx"

# Save to YAML file (docker-compose.yml)
composerize-np "docker run -p 80:80 nginx" -o

# ⚠️ For long/complex commands - use a file!
# This solves escaping and length limitation issues
composerize-np --from-file my-docker-command.txt -o

# Save to JSON file (docker-compose.json)
composerize-np "docker run -p 80:80 nginx" --output-format json -o

# Convert YAML to JSON
composerize-np yaml-to-json docker-compose.yml -o docker-compose.json
```

> **⚠️ IMPORTANT for PowerShell:** If the command contains `||`, `&&`, `|` or other special characters, use **double double quotes** `""` around the problematic value:
> ```powershell
> # ❌ DOESN'T WORK in PowerShell (fails on ||)
> composerize-np "docker run --health-cmd='curl || exit 1' nginx"
> 
> # ✅ WORKS: Use double double quotes ""
> composerize-np "docker run --health-cmd=""curl || exit 1"" nginx"
> 
> # ✅ OR use --from-file (RECOMMENDED for very long commands)
> # 1. Save command to command.txt file
> # 2. Run:
> composerize-np --from-file command.txt -o
> ```

## Installation

### As a CLI tool

```bash
# Install from crates.io (after publication)
cargo install composerize-np

# Or build from source
cd composerize-np
cargo build --release
```

The executable will be in `target/release/composerize-np` (or `composerize-np.exe` on Windows).

### As a library

Add to your `Cargo.toml`:

```toml
[dependencies]
composerize-np = "0.1"
```

**Quick example:**

```rust
use composerize_np::composerize;

fn main() {
    let docker_command = "docker run -d -p 80:80 nginx";
    
    match composerize(docker_command, "", "latest", 2) {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

See [EXAMPLES.md](EXAMPLES.md) for more examples and API documentation.

## Usage

### 1. Converting docker run to compose

There are two ways to use it:

#### Option A: Simple way (direct command)
**Fastest and most convenient for manual use**

Just pass the docker command directly:

```bash
# YAML format (default)
composerize-np "docker run -p 80:80 nginx"

# JSON format (output to console)
composerize-np "docker run -p 80:80 nginx" --output-format json

# Save YAML to file (docker-compose.yml by default)
composerize-np "docker run -p 80:80 nginx" -o

# Save YAML to custom file
composerize-np "docker run -p 80:80 nginx" -o my-compose.yml

# Save JSON to file (docker-compose.json by default)
composerize-np "docker run -p 80:80 nginx" --output-format json -o

# Save JSON to custom file
composerize-np "docker run -p 80:80 nginx" --output-format json -o compose.json

# With formatting parameters
composerize-np "docker run -p 80:80 nginx" -f v3x -i 4 -o
```

**When to use:**
- ✅ Quick conversion in terminal
- ✅ Compatibility with original composerize version
- ✅ Simple one-line commands

#### Option B: With `run` subcommand
**Recommended for scripts and automation**

Use explicit `run` subcommand:

```bash
# YAML format
composerize-np run "docker run -p 80:80 nginx"

# JSON format
composerize-np run "docker run -p 80:80 nginx" --output-format json

# With parameters
composerize-np run "docker run -p 80:80 nginx" -f v3x -i 4 -o compose.yaml
```

**When to use:**
- ✅ In scripts and CI/CD
- ✅ When explicit command is needed for readability
- ✅ When using other subcommands (yaml-to-json, json-to-yaml)

**Both options work identically!** Choose whichever is more convenient for you.

| Criteria | Direct command | With `run` |
|----------|----------------|------------|
| Command length | Shorter | Longer |
| Readability in scripts | Medium | High |
| Compatibility with original | ✅ Yes | ❌ No |
| Intent clarity | Medium | High |
| Recommended for | Manual use | Scripts, CI/CD |

### 2. Converting YAML to JSON

```bash
# Output to console
composerize-np yaml-to-json docker-compose.yml

# Save to file
composerize-np yaml-to-json docker-compose.yml -o docker-compose.json

# Without pretty-print
composerize-np yaml-to-json docker-compose.yml -o output.json --pretty false
```

### 3. Converting JSON to YAML

```bash
# Output to console
composerize-np json-to-yaml docker-compose.json

# Save to file
composerize-np json-to-yaml docker-compose.json -o docker-compose.yml
```

### 4. Automatic conversion

The `convert` command automatically determines format by file extension:

```bash
# YAML → JSON
composerize-np convert docker-compose.yml -o docker-compose.json

# JSON → YAML
composerize-np convert docker-compose.json -o docker-compose.yml
```

### Formatting parameters

```bash
# -f, --format: Docker Compose version (latest, v3x, v2x)
composerize-np "docker run nginx" -f v3x    # Adds version: '3'
composerize-np "docker run nginx" -f v2x    # Adds version: '2'
composerize-np "docker run nginx" -f latest # No version (default)

# -i, --indent: Number of spaces for indentation (default 2)
composerize-np "docker run nginx" -i 4      # 4 spaces instead of 2

# Combination of parameters
composerize-np "docker run -p 80:80 nginx" -f v3x -i 4 -o compose.yml
```

### Help

```bash
# General help
composerize-np --help

# Command help
composerize-np run --help
composerize-np yaml-to-json --help
composerize-np json-to-yaml --help
composerize-np convert --help
```

## Examples

### Simple Nginx web server

```bash
# Option 1: Direct command (shorter)
composerize-np "docker run -d -p 80:80 nginx"

# Option 2: With subcommand (more explicit)
composerize-np run "docker run -d -p 80:80 nginx"
```

Result:
```yaml
services:
  nginx:
    image: nginx
    ports:
      - 80:80
```

### MySQL with environment variables (JSON)

```bash
composerize-np run "docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=secret mysql:8" --output-format json
```

Result:
```json
{
  "services": {
    "mysql": {
      "image": "mysql:8",
      "ports": ["3306:3306"],
      "environment": ["MYSQL_ROOT_PASSWORD=secret"]
    }
  }
}
```

### Converting existing compose file

```bash
# Have docker-compose.yml, need JSON
composerize-np yaml-to-json docker-compose.yml -o docker-compose.json

# Edit JSON...

# Convert back to YAML
composerize-np json-to-yaml docker-compose.json -o docker-compose.yml
```

## Supported Flags

Full support for all major Docker flags:

### Network and ports
- `-p, --publish` - ports
- `--expose` - expose ports
- `--network, --net` - network
- `--network-alias` - network aliases
- `--ip` - IPv4 address
- `--ip6` - IPv6 address
- `--link` - container links
- `--add-host` - additional hosts
- `--dns` - DNS servers
- `--dns-search` - DNS search domains
- `--dns-opt` - DNS options
- `--mac-address` - MAC address

### Volumes and filesystem
- `-v, --volume` - volumes
- `--mount` - mount
- `--volumes-from` - volumes from other containers
- `--tmpfs` - tmpfs mount
- `--read-only` - read-only
- `--workdir, -w` - working directory

### Environment and configuration
- `-e, --env` - environment variables
- `--env-file` - environment file
- `--name` - container name
- `--hostname, -h` - hostname
- `--domainname` - domain name
- `--user, -u` - user
- `--label, -l` - labels
- `--entrypoint` - entrypoint
- `--platform` - platform

### Resources
- `--memory, -m` - memory limit
- `--memory-swap` - swap limit
- `--memory-reservation` - memory reservation
- `--memory-swappiness` - swappiness
- `--cpus` - CPU count
- `--cpu-shares, -c` - CPU shares
- `--cpu-period` - CPU period
- `--cpu-quota` - CPU quota
- `--cpu-rt-period` - CPU RT period
- `--cpu-rt-runtime` - CPU RT runtime
- `--pids-limit` - process limit
- `--ulimit` - ulimits
- `--device` - devices
- `--gpus` - GPU

### Security and privileges
- `--privileged` - privileged mode
- `--cap-add` - add capabilities
- `--cap-drop` - drop capabilities
- `--security-opt` - security options
- `--userns` - user namespace
- `--group-add` - additional groups

### Behavior and policies
- `--restart` - restart policy
- `-d, --detach` - run in background
- `-t, --tty` - allocate pseudo-TTY
- `-i, --interactive` - interactive mode
- `--init` - use init
- `--rm` - remove after stop
- `--pull` - pull policy
- `--stop-signal` - stop signal
- `--stop-timeout` - stop timeout

### Logging
- `--log-driver` - logging driver
- `--log-opt` - logging options

### Healthcheck
- `--health-cmd` - health check command
- `--health-interval` - check interval
- `--health-retries` - retry count
- `--health-timeout` - check timeout
- `--health-start-period` - start period
- `--no-healthcheck` - disable healthcheck

### Other
- `--cgroup-parent` - parent cgroup
- `--cgroupns` - cgroup namespace
- `--ipc` - IPC mode
- `--pid` - PID mode
- `--uts` - UTS namespace
- `--isolation` - isolation
- `--runtime` - runtime
- `--shm-size` - /dev/shm size
- `--sysctl` - sysctl parameters
- `--storage-opt` - storage options
- `--blkio-weight` - Block IO weight
- `--device-read-bps` - device read limit
- `--device-write-bps` - device write limit
- `--device-read-iops` - read IOPS limit
- `--device-write-iops` - write IOPS limit
- `--oom-kill-disable` - disable OOM killer
- `--oom-score-adj` - OOM score adjustment

## Automatic Networks and Volumes Sections

`composerize-np` automatically creates `networks:` and `volumes:` sections at the root of the compose file for used resources:

### Networks
- Automatically adds `networks:` section for custom networks
- Marks them as `external: true` (assumes network is created beforehand)
- Ignores standard networks: `default`, `bridge`, `host`, `none`

```bash
composerize-np "docker run --network ml-net nginx"
```

Result:
```yaml
services:
  nginx:
    networks:
      ml-net: {}
    image: nginx
networks:
  ml-net:
    external: true
```

### Volumes
- Automatically adds `volumes:` section for named volumes
- Does **NOT** add bind mounts (paths starting with `/`, `.`, `~`)
- Named volumes are declared as `null` (created by Docker automatically)

```bash
composerize-np "docker run -v data:/data -v cache:/cache -v /host:/host nginx"
```

Result:
```yaml
services:
  nginx:
    volumes:
    - data:/data
    - cache:/cache
    - /host:/host
    image: nginx
volumes:
  data: null
  cache: null
```

### Full example with GPU, networks and volumes

```bash
composerize-np "docker run -d \
  --name ml-gpu \
  --network ml-net \
  --gpus all \
  -v ml-models:/models \
  -v ml-cache:/cache \
  --tmpfs /tmp:rw,size=256m \
  --health-cmd='curl -f http://localhost:5000/health || exit 1' \
  --health-interval=20s \
  -p 5000:5000 \
  tensorflow/tensorflow:latest-gpu"
```

Result - fully valid compose file:
```yaml
services:
  tensorflow:
    container_name: ml-gpu
    networks:
      ml-net: {}
    deploy:
      resources:
        reservations:
          devices:
          - driver: nvidia
            count: all
            capabilities:
            - gpu
    volumes:
    - ml-models:/models
    - ml-cache:/cache
    tmpfs:
    - /tmp:rw,size=256m
    healthcheck:
      test:
      - CMD-SHELL
      - curl -f http://localhost:5000/health || exit 1
      interval: 20s
    ports:
    - 5000:5000
    image: tensorflow/tensorflow:latest-gpu
networks:
  ml-net:
    external: true
volumes:
  ml-models: null
  ml-cache: null
```

This file fully complies with Docker Compose specification and is ready to use!

## Testing

The project has full test coverage:
- 41 unit tests (including networks/volumes tests)
- 18 integration tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

See [TESTING.md](TESTING.md) for detailed information.

## Development

```bash
# Clone repository
git clone https://github.com/yourusername/composerize-np
cd composerize-np

# Build
cargo build

# Run tests
cargo test

# Build release version
cargo build --release

# Install locally
cargo install --path .
```

## Documentation

### For CLI Users
- 📋 [CHEATSHEET.md](CHEATSHEET.md) - Quick cheat sheet with command examples (including PowerShell tips)
- ❓ [FAQ.md](FAQ.md) - Frequently Asked Questions

### For Library Users
- 📚 [LIBRARY_USAGE.md](LIBRARY_USAGE.md) - Complete guide for using as a library
- 💡 [EXAMPLES.md](EXAMPLES.md) - Detailed examples with explanations
- 🔧 [examples/](examples/) - Runnable code examples (`cargo run --example basic_usage`)
- 📖 [API Documentation](https://docs.rs/composerize-np) - Full API reference (after publication)

## FAQ

Have questions? See [FAQ.md](FAQ.md) with answers to common questions:
- What's the difference between direct command and `run`?
- How to choose between YAML and JSON?
- How to convert between formats?
- Which flags are supported?
- And much more!

## License

MIT
