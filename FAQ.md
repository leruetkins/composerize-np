# FAQ - Frequently Asked Questions

## General Questions

### What's the difference between `composerize-np "docker run..."` and `composerize-np run "docker run..."`?

**Short answer:** None! Both work identically.

**Details:**

1. **Direct command** (without `run`):
   ```bash
   composerize-np "docker run -p 80:80 nginx"
   ```
   - ✅ Shorter and faster to type
   - ✅ Compatible with original JavaScript version
   - ✅ Convenient for manual terminal use
   - ❌ Less explicit about what the command does

2. **With `run` subcommand**:
   ```bash
   composerize-np run "docker run -p 80:80 nginx"
   ```
   - ✅ Explicitly indicates converting docker run
   - ✅ Better for scripts and automation
   - ✅ Consistent with other subcommands (yaml-to-json, json-to-yaml)
   - ❌ Slightly longer

**Recommendation:**
- For manual work: use direct command
- For scripts/CI/CD: use `run`

### Why doesn't the command `composerize-np "docker run..."` work?

Make sure that:
1. Command is in quotes: `"docker run -p 80:80 nginx"`
2. You're using the latest version (with legacy mode support)
3. If it doesn't work, try with `run`: `composerize-np run "docker run..."`

### Why does the program crash/close on long commands in the command line?

**Short answer:** Use double double quotes `""` in PowerShell or `--from-file` for long commands!

**Quick fix for PowerShell:**
```powershell
# ❌ Doesn't work
composerize-np "docker run --health-cmd='curl || exit 1' nginx"

# ✅ Works - use double double quotes
composerize-np "docker run --health-cmd=""curl || exit 1"" nginx"
```

**Reasons for problems with direct input:**

1. **Command line length limitations:**
   - Windows CMD: ~8191 characters
   - PowerShell: ~32767 characters (but with escaping issues)
   - Your command may exceed these limits

2. **Special character escaping issues:**
   - Quotes: `"curl -f http://localhost || exit 1"`
   - Operators: `||`, `&&`, `|`, `&`, `;`
   - Special characters: `<`, `>`, `` ` ``, `'`, `=`, `,`
   - PowerShell/CMD try to interpret them BEFORE passing to the program

3. **Example problematic command:**
   ```bash
   # ❌ May crash in PowerShell
   composerize-np "docker run --health-cmd='curl http://localhost || exit 1' nginx"
   # PowerShell sees || and tries to process it as an operator
   ```

**Solution: use `--from-file`**

1. **Save command to file** (e.g., `command.txt`):
   
   Single-line format:
   ```
   docker run -d --name app -p 80:80 --health-cmd="curl http://localhost || exit 1" nginx
   ```
   
   Or Bash-style with line breaks:
   ```bash
   docker run -d \
     --name app \
     -p 80:80 \
     --health-cmd="curl http://localhost || exit 1" \
     nginx
   ```

2. **Use `--from-file`**:
   ```bash
   # Simple way (without run)
   composerize-np --from-file command.txt -o
   
   # Or with subcommand
   composerize-np run --from-file command.txt -o
   ```

**Advantages of `--from-file`:**
- ✅ No length limitations
- ✅ No escaping issues
- ✅ All special characters work (`, ", ', $, ||, &&, etc.)
- ✅ Bash-style backslash support (`\`)
- ✅ Multi-line commands
- ✅ Shell doesn't see or interpret the content

**When to use direct input:**
- ✅ Short simple commands: `docker run -p 80:80 nginx`
- ✅ Commands without special characters
- ✅ Quick testing

**When to use `--from-file`:**
- ✅ Long commands (>100 characters)
- ✅ Commands with special characters (`||`, `&&`, quotes)
- ✅ Production commands with many parameters
- ✅ Commands from documentation/scripts

### How to choose between YAML and JSON?

**YAML (default):**
```bash
# Output to console
composerize-np "docker run -p 80:80 nginx"

# Save to file
composerize-np "docker run -p 80:80 nginx" -o docker-compose.yml
```
- ✅ Standard Docker Compose format
- ✅ More human-readable
- ✅ Fewer characters

**JSON:**
```bash
# Output to console
composerize-np "docker run -p 80:80 nginx" --output-format json

# Save to file
composerize-np "docker run -p 80:80 nginx" --output-format json -o compose.json
```
- ✅ Easier to parse programmatically
- ✅ Strict structure
- ✅ Support in most programming languages

### How to save result directly to JSON file?

Use combination of `--output-format json` and `-o`:

```bash
# Save to docker-compose.json (automatic name)
composerize-np "docker run -p 80:80 nginx" --output-format json -o

# Save to custom file
composerize-np "docker run -p 80:80 nginx" --output-format json -o compose.json

# Full example with MySQL
composerize-np "docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=secret mysql:8" --output-format json -o mysql.json
```

**Important:** When using `-o` without filename:
- YAML format → `docker-compose.yml`
- JSON format → `docker-compose.json`

## Format Conversion

### How to convert existing docker-compose.yml to JSON?

```bash
composerize-np yaml-to-json docker-compose.yml -o docker-compose.json
```

### How to convert JSON back to YAML?

```bash
composerize-np json-to-yaml docker-compose.json -o docker-compose.yml
```

### Can I convert automatically by file extension?

Yes! Use the `convert` command:

```bash
# YAML → JSON
composerize-np convert input.yml -o output.json

# JSON → YAML
composerize-np convert input.json -o output.yml
```

## Working with Files

### How to save result to file?

```bash
# To docker-compose.yml (default)
composerize-np "docker run -p 80:80 nginx" -o

# To custom file
composerize-np "docker run -p 80:80 nginx" -o my-compose.yml
```

### How to output to console instead of file?

Simply don't use the `-o` flag:

```bash
composerize-np "docker run -p 80:80 nginx"
```

### Can I read from stdin?

Yes, if stdin is not TTY, the program will read existing compose file for merge:

```bash
cat existing-compose.yml | composerize-np "docker run -p 80:80 nginx"
```

## Docker Compose Formats

### Which Docker Compose versions are supported?

The `-f, --format` parameter determines the Docker Compose version:

- **`latest`** (default) - without version specification (Docker Compose Specification)
- **`v3x`** - Docker Compose v3
- **`v2x`** - Docker Compose v2

```bash
# Without version (modern format)
composerize-np "docker run -p 80:80 nginx"
# Result:
# services:
#   nginx:
#     image: nginx
#     ports:
#       - 80:80

# Version 3
composerize-np "docker run -p 80:80 nginx" -f v3x
# Result:
# version: '3'
# services:
#   nginx:
#     image: nginx
#     ports:
#       - 80:80

# Version 2
composerize-np "docker run -p 80:80 nginx" -f v2x
# Result:
# version: '2'
# services:
#   nginx:
#     image: nginx
#     ports:
#       - 80:80
```

### Which format to choose?

- **`latest`** - for new projects (recommended)
  - ✅ Modern standard
  - ✅ Supported by Docker Compose v2+
  - ✅ No version specification required

- **`v3x`** - for Docker Compose v3 compatibility
  - ✅ Wide support
  - ✅ Stable format
  - ⚠️ Some features deprecated

- **`v2x`** - for old projects on Docker Compose v2
  - ⚠️ Deprecated format
  - ✅ Compatibility with legacy systems

### What is the `-i, --indent` parameter?

Specifies the number of spaces for indentation in YAML file:

```bash
# 2 spaces (default, YAML standard)
composerize-np "docker run nginx" -i 2

# 4 spaces (popular in some projects)
composerize-np "docker run nginx" -i 4
```

**Note:** In the current version, YAML always uses 2 spaces (serde_yaml standard). The `-i` parameter is kept for compatibility with the original version and future improvements.

**For JSON:** The `-i` parameter affects JSON output formatting:
```bash
# Compact JSON (no indentation)
composerize-np "docker run nginx" --output-format json -i 0

# JSON with indentation (default 2)
composerize-np "docker run nginx" --output-format json -i 2
```

## Flag Support

### Which Docker flags are supported?

**All** major Docker flags (80+) are supported:
- Ports, volumes, environment
- Memory, CPU, GPU
- Networks, healthcheck, logging
- Security, capabilities
- And much more

See full list in [README.md](README.md#supported-flags)

### What to do if a flag is not supported?

1. Check that you're using correct syntax
2. Look at the list of supported flags in README
3. Create an issue on GitHub with command example

## Performance

### How much faster is the Rust version?

- ⚡ Startup: ~10-50x faster (no Node.js overhead)
- 💾 Memory: ~10x less (~5 MB vs ~50 MB)
- 📦 Size: ~30x smaller (~3 MB vs ~100 MB with Node.js)

### Can I use it in CI/CD?

Yes! The Rust version is ideal for CI/CD:
- Fast startup
- Single executable file
- No dependencies
- Cross-platform

## Development

### How to build from source?

```bash
git clone https://github.com/yourusername/composerize-np
cd composerize-np
cargo build --release
```

### How to run tests?

```bash
cargo test
```

### How to add support for a new flag?

1. Add mapping in `src/mappings.rs`
2. Add test in `src/lib.rs` or `tests/integration_tests.rs`
3. Run `cargo test`

## Comparison with Original

### How is it different from the JavaScript version?

| Feature | JavaScript | Rust |
|---------|-----------|------|
| Basic flags | ✅ | ✅ |
| All flags | ✅ | ✅ |
| YAML output | ✅ | ✅ |
| JSON output | ❌ | ✅ |
| YAML ↔ JSON | ❌ | ✅ |
| Speed | 🐌 | ⚡ |
| Size | ~100 MB | ~3 MB |
| Dependencies | Node.js | None |

### Can I replace the JavaScript version?

Yes! The Rust version is fully compatible and has additional features:
- ✅ All flags supported
- ✅ Compatible CLI (direct command works)
- ✅ Additionally: JSON output and YAML↔JSON conversion
- ✅ Faster and lighter

## Troubleshooting

### Error "No image specified"

Make sure the command contains an image name:

```bash
# ❌ Wrong
composerize-np "docker run -d -p 80:80"

# ✅ Correct
composerize-np "docker run -d -p 80:80 nginx"
```

### Error "Unknown format"

Use one of the supported formats:
- `latest` (default)
- `v3x`
- `v2x`

```bash
composerize-np "docker run nginx" -f v3x
```

### Flag is ignored

Some flags (e.g., `--rm`, `-d`) are ignored as they have no equivalent in Docker Compose:
- `--rm` - containers in Compose are always removed on stop
- `-d` - containers in Compose always run in background

## Networks and Volumes

### Why do networks and volumes sections appear in the compose file?

`composerize-np` automatically creates `networks:` and `volumes:` sections for used resources to make the file fully valid and ready to use.

**Example:**
```bash
composerize-np "docker run --network ml-net -v data:/data nginx"
```

Result:
```yaml
services:
  nginx:
    networks:
      ml-net: {}
    volumes:
    - data:/data
    image: nginx
networks:
  ml-net:
    external: true
volumes:
  data: null
```

### What does `external: true` mean for networks?

This means the network must be created beforehand with:
```bash
docker network create ml-net
```

If you want Docker Compose to create the network automatically, remove `external: true` or replace with:
```yaml
networks:
  ml-net:
    driver: bridge
```

### Why don't bind mounts appear in the volumes section?

Bind mounts (paths starting with `/`, `.`, `~`) don't require declaration in the `volumes:` section as they reference existing paths in the host filesystem.

**Named volumes** (e.g., `data:/data`) require declaration so Docker knows to create the volume.

```bash
# Bind mount - will NOT appear in volumes section
composerize-np "docker run -v /host/data:/data nginx"

# Named volume - will appear in volumes section
composerize-np "docker run -v data:/data nginx"
```

### How to disable automatic creation of networks/volumes sections?

In the current version, this behavior is enabled by default to create valid compose files. If you need to disable it, you can:

1. Manually remove sections from generated file
2. Use standard networks (`bridge`, `host`, `none`) - they won't appear in networks section

### Can I use multiple networks?

Yes! Just specify multiple `--network` flags:

```bash
composerize-np "docker run --network net1 --network net2 nginx"
```

Result:
```yaml
services:
  nginx:
    networks:
      net1: {}
      net2: {}
    image: nginx
networks:
  net1:
    external: true
  net2:
    external: true
```

## Additional Help

Didn't find an answer to your question?

- 📖 Read [README.md](README.md)
- 🐛 Create an issue on GitHub
- 💬 Ask a question in Discussions
