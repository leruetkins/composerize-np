use composerize_np::composerize;

#[test]
fn test_full_stack_example() {
    let input = "docker run -d --name postgres-db -p 5432:5432 -e POSTGRES_PASSWORD=secret -e POSTGRES_USER=admin -e POSTGRES_DB=myapp -v pgdata:/var/lib/postgresql/data --restart unless-stopped --memory 1g --cpus 2 postgres:15-alpine";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("postgres"));
    assert!(yaml.contains("5432:5432"));
    assert!(yaml.contains("POSTGRES_PASSWORD=secret"));
    assert!(yaml.contains("POSTGRES_USER=admin"));
    assert!(yaml.contains("POSTGRES_DB=myapp"));
    assert!(yaml.contains("pgdata:/var/lib/postgresql/data"));
    assert!(yaml.contains("container_name: postgres-db"));
    assert!(yaml.contains("restart: unless-stopped"));
    assert!(yaml.contains("memory: 1g"));
    assert!(yaml.contains("cpus: 2"));
}

#[test]
fn test_web_server_with_volumes() {
    let input = "docker run -d -p 80:80 -p 443:443 -v /etc/nginx:/etc/nginx:ro -v /var/www:/usr/share/nginx/html --name webserver nginx:alpine";
    
    let result = composerize(input, "", "v3x", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("version: '3'") || yaml.contains("version: \"3\""));
    assert!(yaml.contains("80:80"));
    assert!(yaml.contains("443:443"));
    assert!(yaml.contains("/etc/nginx:/etc/nginx:ro"));
    assert!(yaml.contains("/var/www:/usr/share/nginx/html"));
    assert!(yaml.contains("container_name: webserver"));
}

#[test]
fn test_redis_with_network() {
    let input = "docker run -d --name redis-cache -p 6379:6379 --network backend --restart always redis:alpine";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("redis"));
    assert!(yaml.contains("6379:6379"));
    assert!(yaml.contains("container_name: redis-cache"));
    assert!(yaml.contains("restart: always"));
}

#[test]
fn test_mysql_with_healthcheck() {
    let input = "docker run -d --name mysql-db -p 3306:3306 -e MYSQL_ROOT_PASSWORD=rootpass -e MYSQL_DATABASE=testdb --health-cmd 'mysqladmin ping -h localhost' --health-interval 10s --health-timeout 5s --health-retries 3 mysql:8";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("mysql"));
    assert!(yaml.contains("3306:3306"));
    assert!(yaml.contains("MYSQL_ROOT_PASSWORD=rootpass"));
    assert!(yaml.contains("MYSQL_DATABASE=testdb"));
    assert!(yaml.contains("healthcheck:"));
    assert!(yaml.contains("interval: 10s"));
    assert!(yaml.contains("timeout: 5s"));
    assert!(yaml.contains("retries: 3"));
}

#[test]
fn test_node_app_with_command() {
    let input = "docker run -d -p 3000:3000 -e NODE_ENV=production -v /app:/usr/src/app --workdir /usr/src/app node:18 npm start";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("node"));
    assert!(yaml.contains("3000:3000"));
    assert!(yaml.contains("NODE_ENV=production"));
    assert!(yaml.contains("/app:/usr/src/app"));
    assert!(yaml.contains("working_dir: /usr/src/app"));
    assert!(yaml.contains("command: npm start"));
}

#[test]
fn test_privileged_container_with_devices() {
    let input = "docker run -d --privileged --device /dev/sda:/dev/sda --cap-add SYS_ADMIN ubuntu";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("privileged: true"));
    assert!(yaml.contains("devices:"));
    assert!(yaml.contains("/dev/sda:/dev/sda"));
    assert!(yaml.contains("cap_add:"));
    assert!(yaml.contains("SYS_ADMIN"));
}

#[test]
fn test_container_with_labels() {
    let input = "docker run -d -l traefik.enable=true -l traefik.http.routers.app.rule=Host(`example.com`) nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("labels:"));
    assert!(yaml.contains("traefik.enable=true"));
    assert!(yaml.contains("traefik.http.routers.app.rule=Host(`example.com`)"));
}

#[test]
fn test_container_with_logging() {
    let input = "docker run -d --log-driver json-file --log-opt max-size=10m --log-opt max-file=3 nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("logging:"));
    assert!(yaml.contains("driver: json-file"));
    assert!(yaml.contains("options:"));
    assert!(yaml.contains("max-size: 10m"));
    assert!(yaml.contains("max-file: 3"));
}

#[test]
fn test_interactive_shell() {
    let input = "docker run -it --rm ubuntu bash";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("stdin_open: true"));
    assert!(yaml.contains("tty: true"));
    assert!(yaml.contains("command: bash"));
}

#[test]
fn test_container_with_dns() {
    let input = "docker run -d --dns 8.8.8.8 --dns 8.8.4.4 --dns-search example.com nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("dns:"));
    assert!(yaml.contains("8.8.8.8"));
    assert!(yaml.contains("8.8.4.4"));
    assert!(yaml.contains("dns_search:"));
    assert!(yaml.contains("example.com"));
}

#[test]
fn test_container_with_ulimits() {
    let input = "docker run -d --ulimit nofile=1024:2048 --ulimit nproc=512 nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("ulimits:"));
    assert!(yaml.contains("nofile:"));
    assert!(yaml.contains("soft: 1024"));
    assert!(yaml.contains("hard: 2048"));
    assert!(yaml.contains("nproc: 512"));
}

#[test]
fn test_container_with_hostname_and_user() {
    let input = "docker run -d --hostname myapp --user 1000:1000 nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("hostname: myapp"));
    assert!(yaml.contains("user: 1000:1000"));
}

#[test]
fn test_podman_command() {
    let input = "podman run -d -p 8080:80 nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("nginx"));
    assert!(yaml.contains("8080:80"));
}

#[test]
fn test_docker_create_command() {
    let input = "docker create -p 80:80 nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("nginx"));
    assert!(yaml.contains("80:80"));
}

#[test]
fn test_multiple_environment_files() {
    let input = "docker run -d --env-file .env --env-file .env.prod nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("env_file:"));
    assert!(yaml.contains(".env"));
    assert!(yaml.contains(".env.prod"));
}

#[test]
fn test_short_flags_combined() {
    let input = "docker run -dit -p 80:80 -v /data:/app nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("stdin_open: true"));
    assert!(yaml.contains("tty: true"));
    assert!(yaml.contains("80:80"));
    assert!(yaml.contains("/data:/app"));
}

#[test]
fn test_memory_and_cpu_limits() {
    let input = "docker run -d --memory 2g --memory-swap 4g --cpus 4 --cpu-shares 1024 nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("memory: 2g"));
    assert!(yaml.contains("memswap_limit: 4g"));
    assert!(yaml.contains("cpus: 4"));
    assert!(yaml.contains("cpu_shares: 1024"));
}

#[test]
fn test_security_options() {
    let input = "docker run -d --security-opt no-new-privileges --cap-drop ALL --cap-add NET_BIND_SERVICE nginx";
    
    let result = composerize(input, "", "latest", 2);
    assert!(result.is_ok());
    
    let yaml = result.unwrap();
    assert!(yaml.contains("security_opt:"));
    assert!(yaml.contains("no-new-privileges"));
    assert!(yaml.contains("cap_drop:"));
    assert!(yaml.contains("ALL"));
    assert!(yaml.contains("cap_add:"));
    assert!(yaml.contains("NET_BIND_SERVICE"));
}
