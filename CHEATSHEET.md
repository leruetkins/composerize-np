# composerize-np Cheat Sheet

## Quick Commands

### Converting docker run

```bash
# Output YAML to console
composerize-np "docker run -p 80:80 nginx"

# Output JSON to console
composerize-np "docker run -p 80:80 nginx" --output-format json

# Save to YAML file (docker-compose.yml)
composerize-np "docker run -p 80:80 nginx" -o

# Save to JSON file (docker-compose.json)
composerize-np "docker run -p 80:80 nginx" --output-format json -o

# Read command from file (for long commands)
composerize-np --from-file command.txt -o
# Or with subcommand
composerize-np run --from-file command.txt -o

# Custom file names
composerize-np "docker run -p 80:80 nginx" -o my-compose.yml
composerize-np "docker run -p 80:80 nginx" --output-format json -o my-compose.json
```

### Format Conversion

```bash
# YAML → JSON
composerize-np yaml-to-json input.yml -o output.json

# JSON → YAML
composerize-np json-to-yaml input.json -o output.yml

# Automatic
composerize-np convert input.yml -o output.json
```

## Parameters

| Flag | Description | Values | Example |
|------|-------------|--------|---------|
| `-o, --output` | Save to file | File path or empty for default | `-o compose.yml` |
| `--output-format` | Output format | `yaml` (default) or `json` | `--output-format json` |
| `-f, --format` | Docker Compose version | `latest` (default), `v3x`, `v2x` | `-f v3x` |
| `-i, --indent` | Number of spaces for indentation | Number (default 2) | `-i 4` |
| `--from-file` | Read command from file | File path | `--from-file cmd.txt` |

### Parameter Details

#### `-f, --format` - Docker Compose Version

Specifies which Docker Compose version to use in the output file:

- **`latest`** (default) - Modern format without version specification
  ```yaml
  services:
    nginx:
      image: nginx
  ```

- **`v3x`** - Docker Compose version 3
  ```yaml
  version: '3'
  services:
    nginx:
      image: nginx
  ```

- **`v2x`** - Docker Compose version 2
  ```yaml
  version: '2'
  services:
    nginx:
      image: nginx
  ```

#### `-i, --indent` - Indentation

Number of spaces for indentation in YAML file:

```bash
# 2 spaces (default)
composerize-np "docker run nginx" -i 2
# Result:
# services:
#   nginx:
#     image: nginx

# 4 spaces
composerize-np "docker run nginx" -i 4
# Result:
# services:
#     nginx:
#         image: nginx
```

## Command Examples

### Simple

```bash
# Nginx
composerize-np "docker run -p 80:80 nginx"

# Redis
composerize-np "docker run -p 6379:6379 redis"

# MySQL
composerize-np "docker run -p 3306:3306 -e MYSQL_ROOT_PASSWORD=secret mysql"
```

### With volumes

```bash
composerize-np "docker run -v /data:/app -v /logs:/var/log nginx"
```

### With environment

```bash
composerize-np "docker run -e NODE_ENV=production -e PORT=3000 node"
```

### Complete Example

```bash
# Direct command (short commands)
composerize-np "docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=secret postgres:15" -o

# From file (long commands)
# Create command.txt file:
cat > command.txt << 'EOF'
docker run -d \
  --name postgres \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=secret \
  -v pgdata:/var/lib/postgresql/data \
  --restart always \
  --memory 1g \
  postgres:15
EOF

# Convert
composerize-np run --from-file command.txt -o postgres.yml
```

### JSON Output

```bash
# To console
composerize-np "docker run -p 80:80 nginx" --output-format json

# To file
composerize-np "docker run -p 80:80 nginx" --output-format json -o compose.json

# With v3x format
composerize-np "docker run -p 80:80 nginx" -f v3x --output-format json -o compose.json
```

## Common Scenarios

### 1. Quick Conversion for Testing

```bash
composerize-np "docker run -p 80:80 nginx"
```

### 2. Creating Compose File for Project

```bash
composerize-np "docker run -d -p 3000:3000 -e NODE_ENV=production myapp" -o docker-compose.yml
```

### 3. Conversion for API (JSON)

```bash
composerize-np "docker run -p 80:80 nginx" --output-format json -o compose.json
```

### 4. Migrating from docker run to compose

```bash
# Step 1: Convert command
composerize-np "docker run -d -p 80:80 nginx" -o

# Step 2: Run via compose
docker-compose up -d
```

### 5. Working with Existing Files

```bash
# Convert YAML to JSON for editing
composerize-np yaml-to-json docker-compose.yml -o compose.json

# Edit JSON...

# Convert back
composerize-np json-to-yaml compose.json -o docker-compose.yml
```

## Shortcuts (Aliases)

Add to `.bashrc` or `.zshrc`:

```bash
# Short alias
alias dcr='composerize-np'

# With auto-save
alias dcr-save='composerize-np -o'

# JSON output
alias dcr-json='composerize-np --output-format json'

# Usage
dcr "docker run -p 80:80 nginx"
dcr-save "docker run -p 80:80 nginx"
dcr-json "docker run -p 80:80 nginx"
```

## Troubleshooting

### Problem: Command doesn't work

```bash
# ❌ Without quotes
composerize-np docker run -p 80:80 nginx

# ✅ With quotes
composerize-np "docker run -p 80:80 nginx"
```

### Problem: Need JSON file

```bash
# ❌ Format only, no save
composerize-np "docker run -p 80:80 nginx" --output-format json

# ✅ With save
composerize-np "docker run -p 80:80 nginx" --output-format json -o compose.json
```

### Problem: Wrong format

```bash
# ❌ Invalid format
composerize-np "docker run nginx" -f v4

# ✅ Valid formats
composerize-np "docker run nginx" -f v2x
composerize-np "docker run nginx" -f v3x
composerize-np "docker run nginx" -f latest
```

## Useful Combinations

### Conversion + View

```bash
# Linux/Mac
composerize-np "docker run -p 80:80 nginx" -o && cat docker-compose.yml

# Windows PowerShell
composerize-np "docker run -p 80:80 nginx" -o; Get-Content docker-compose.yml
```

## PowerShell: Working with Special Characters

### Commands with || or && (healthcheck, etc.)

```powershell
# ❌ DOESN'T WORK - PowerShell fails on ||
composerize-np "docker run --health-cmd='curl http://localhost || exit 1' nginx"

# ✅ WORKS - Use double double quotes ""
composerize-np "docker run --health-cmd=""curl http://localhost || exit 1"" nginx"

# ✅ OR use --from-file (for very long commands)
echo 'docker run --health-cmd="curl || exit 1" nginx' > command.txt
composerize-np --from-file command.txt -o
```

### Full Command with Many Special Characters

```powershell
# For complex production commands - always use --from-file
composerize-np --from-file my-complex-command.txt -o docker-compose.yml
```

### Conversion + Run

```bash
composerize-np "docker run -p 80:80 nginx" -o && docker-compose up -d
```

### Conversion + git

```bash
composerize-np "docker run -p 80:80 nginx" -o && git add docker-compose.yml && git commit -m "Add compose file"
```

## Help

```bash
# General help
composerize-np --help

# Command help
composerize-np run --help
composerize-np yaml-to-json --help
composerize-np json-to-yaml --help
composerize-np convert --help
```

## Links

- [README.md](README.md) - Full documentation
- [FAQ.md](FAQ.md) - Frequently Asked Questions
