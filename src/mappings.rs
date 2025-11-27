use indexmap::IndexMap;
use serde_yaml::Value;

#[derive(Debug, Clone)]
pub enum ArgType {
    Array,
    Ulimits,
    Switch,
    Value,
    IntValue,
    FloatValue,
    DeviceBlockIOConfigRate,
    DeviceBlockIOConfigWeight,
    Networks,
    MapArray,
    Map,
    Envs,
    Gpus,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    pub arg_type: ArgType,
    pub path: String,
}

impl Mapping {
    pub fn new(arg_type: ArgType, path: &str) -> Self {
        Self {
            arg_type,
            path: path.to_string(),
        }
    }
}

pub fn get_mappings() -> IndexMap<String, Mapping> {
    let mut mappings = IndexMap::new();

    // Main mappings
    mappings.insert("add-host".to_string(), Mapping::new(ArgType::Array, "extra_hosts"));
    mappings.insert("blkio-weight".to_string(), Mapping::new(ArgType::IntValue, "blkio_config/weight"));
    mappings.insert("blkio-weight-device".to_string(), Mapping::new(ArgType::DeviceBlockIOConfigWeight, "blkio_config/weight_device"));
    mappings.insert("cap-add".to_string(), Mapping::new(ArgType::Array, "cap_add"));
    mappings.insert("cap-drop".to_string(), Mapping::new(ArgType::Array, "cap_drop"));
    mappings.insert("cgroup-parent".to_string(), Mapping::new(ArgType::Value, "cgroup_parent"));
    mappings.insert("cgroupns".to_string(), Mapping::new(ArgType::Value, "cgroup"));
    mappings.insert("cpu-period".to_string(), Mapping::new(ArgType::Value, "cpu_period"));
    mappings.insert("cpu-quota".to_string(), Mapping::new(ArgType::Value, "cpu_quota"));
    mappings.insert("cpu-rt-period".to_string(), Mapping::new(ArgType::Value, "cpu_rt_period"));
    mappings.insert("cpu-rt-runtime".to_string(), Mapping::new(ArgType::Value, "cpu_rt_runtime"));
    mappings.insert("cpu-shares".to_string(), Mapping::new(ArgType::IntValue, "cpu_shares"));
    mappings.insert("cpus".to_string(), Mapping::new(ArgType::FloatValue, "deploy/resources/limits/cpus"));
    mappings.insert("detached".to_string(), Mapping::new(ArgType::Switch, ""));
    mappings.insert("device-cgroup-rule".to_string(), Mapping::new(ArgType::Array, "device_cgroup_rules"));
    mappings.insert("device-read-bps".to_string(), Mapping::new(ArgType::DeviceBlockIOConfigRate, "blkio_config/device_read_bps"));
    mappings.insert("device-read-iops".to_string(), Mapping::new(ArgType::DeviceBlockIOConfigRate, "blkio_config/device_read_iops"));
    mappings.insert("device-write-bps".to_string(), Mapping::new(ArgType::DeviceBlockIOConfigRate, "blkio_config/device_write_bps"));
    mappings.insert("device-write-iops".to_string(), Mapping::new(ArgType::DeviceBlockIOConfigRate, "blkio_config/device_write_iops"));
    mappings.insert("device".to_string(), Mapping::new(ArgType::Array, "devices"));
    mappings.insert("dns-opt".to_string(), Mapping::new(ArgType::Array, "dns_opt"));
    mappings.insert("dns-search".to_string(), Mapping::new(ArgType::Array, "dns_search"));
    mappings.insert("dns".to_string(), Mapping::new(ArgType::Array, "dns"));
    mappings.insert("domainname".to_string(), Mapping::new(ArgType::Value, "domainname"));
    mappings.insert("entrypoint".to_string(), Mapping::new(ArgType::Array, "entrypoint"));
    mappings.insert("env-file".to_string(), Mapping::new(ArgType::Array, "env_file"));
    mappings.insert("env".to_string(), Mapping::new(ArgType::Envs, "environment"));
    mappings.insert("expose".to_string(), Mapping::new(ArgType::Array, "expose"));
    mappings.insert("gpus".to_string(), Mapping::new(ArgType::Gpus, "deploy"));
    mappings.insert("group-add".to_string(), Mapping::new(ArgType::Array, "group_add"));
    mappings.insert("health-cmd".to_string(), Mapping::new(ArgType::Value, "healthcheck/test"));
    mappings.insert("health-interval".to_string(), Mapping::new(ArgType::Value, "healthcheck/interval"));
    mappings.insert("health-retries".to_string(), Mapping::new(ArgType::IntValue, "healthcheck/retries"));
    mappings.insert("health-start-period".to_string(), Mapping::new(ArgType::Value, "healthcheck/start_period"));
    mappings.insert("health-timeout".to_string(), Mapping::new(ArgType::Value, "healthcheck/timeout"));
    mappings.insert("hostname".to_string(), Mapping::new(ArgType::Value, "hostname"));
    mappings.insert("init".to_string(), Mapping::new(ArgType::Switch, "init"));
    mappings.insert("interactive".to_string(), Mapping::new(ArgType::Switch, "stdin_open"));
    mappings.insert("ip6".to_string(), Mapping::new(ArgType::Value, "networks/¤network¤/ipv6_address"));
    mappings.insert("ip".to_string(), Mapping::new(ArgType::Value, "networks/¤network¤/ipv4_address"));
    mappings.insert("ipc".to_string(), Mapping::new(ArgType::Value, "ipc"));
    mappings.insert("isolation".to_string(), Mapping::new(ArgType::Value, "isolation"));
    mappings.insert("label".to_string(), Mapping::new(ArgType::Array, "labels"));
    mappings.insert("link-local-ip".to_string(), Mapping::new(ArgType::Array, "networks/¤network¤/link_local_ips"));
    mappings.insert("link".to_string(), Mapping::new(ArgType::Array, "links"));
    mappings.insert("log-driver".to_string(), Mapping::new(ArgType::Value, "logging/driver"));
    mappings.insert("log-opt".to_string(), Mapping::new(ArgType::Map, "logging/options"));
    mappings.insert("mac-address".to_string(), Mapping::new(ArgType::Value, "mac_address"));
    mappings.insert("memory-reservation".to_string(), Mapping::new(ArgType::Value, "deploy/resources/reservations/memory"));
    mappings.insert("memory-swap".to_string(), Mapping::new(ArgType::Value, "memswap_limit"));
    mappings.insert("memory-swappiness".to_string(), Mapping::new(ArgType::Value, "mem_swappiness"));
    mappings.insert("memory".to_string(), Mapping::new(ArgType::Value, "deploy/resources/limits/memory"));
    mappings.insert("mount".to_string(), Mapping::new(ArgType::MapArray, "volumes"));
    mappings.insert("name".to_string(), Mapping::new(ArgType::Value, "container_name"));
    mappings.insert("net".to_string(), Mapping::new(ArgType::Networks, "network_mode"));
    mappings.insert("network-alias".to_string(), Mapping::new(ArgType::Array, "networks/¤network¤/aliases"));
    mappings.insert("network".to_string(), Mapping::new(ArgType::Networks, "network_mode"));
    mappings.insert("no-healthcheck".to_string(), Mapping::new(ArgType::Switch, "healthcheck/disable"));
    mappings.insert("oom-kill-disable".to_string(), Mapping::new(ArgType::Switch, "oom_kill_disable"));
    mappings.insert("oom-score-adj".to_string(), Mapping::new(ArgType::Value, "oom_score_adj"));
    mappings.insert("pid".to_string(), Mapping::new(ArgType::Value, "pid"));
    mappings.insert("pids-limit".to_string(), Mapping::new(ArgType::IntValue, "deploy/resources/limits/pids"));
    mappings.insert("platform".to_string(), Mapping::new(ArgType::Value, "platform"));
    mappings.insert("privileged".to_string(), Mapping::new(ArgType::Switch, "privileged"));
    mappings.insert("publish".to_string(), Mapping::new(ArgType::Array, "ports"));
    mappings.insert("pull".to_string(), Mapping::new(ArgType::Value, "pull_policy"));
    mappings.insert("read-only".to_string(), Mapping::new(ArgType::Switch, "read_only"));
    mappings.insert("restart".to_string(), Mapping::new(ArgType::Value, "restart"));
    mappings.insert("rm".to_string(), Mapping::new(ArgType::Switch, ""));
    mappings.insert("runtime".to_string(), Mapping::new(ArgType::Value, "runtime"));
    mappings.insert("security-opt".to_string(), Mapping::new(ArgType::Array, "security_opt"));
    mappings.insert("shm-size".to_string(), Mapping::new(ArgType::Value, "shm_size"));
    mappings.insert("stop-signal".to_string(), Mapping::new(ArgType::Value, "stop_signal"));
    mappings.insert("stop-timeout".to_string(), Mapping::new(ArgType::Value, "stop_grace_period"));
    mappings.insert("storage-opt".to_string(), Mapping::new(ArgType::Map, "storage_opt"));
    mappings.insert("sysctl".to_string(), Mapping::new(ArgType::Array, "sysctls"));
    mappings.insert("tmpfs".to_string(), Mapping::new(ArgType::Array, "tmpfs"));
    mappings.insert("tty".to_string(), Mapping::new(ArgType::Switch, "tty"));
    mappings.insert("ulimit".to_string(), Mapping::new(ArgType::Ulimits, "ulimits"));
    mappings.insert("user".to_string(), Mapping::new(ArgType::Value, "user"));
    mappings.insert("userns".to_string(), Mapping::new(ArgType::Value, "userns_mode"));
    mappings.insert("uts".to_string(), Mapping::new(ArgType::Value, "uts"));
    mappings.insert("volume".to_string(), Mapping::new(ArgType::Array, "volumes"));
    mappings.insert("volumes-from".to_string(), Mapping::new(ArgType::Array, "volumes_from"));
    mappings.insert("workdir".to_string(), Mapping::new(ArgType::Value, "working_dir"));

    // Short flags
    mappings.insert("v".to_string(), mappings.get("volume").unwrap().clone());
    mappings.insert("p".to_string(), mappings.get("publish").unwrap().clone());
    mappings.insert("e".to_string(), mappings.get("env").unwrap().clone());
    mappings.insert("l".to_string(), mappings.get("label").unwrap().clone());
    mappings.insert("h".to_string(), mappings.get("hostname").unwrap().clone());
    mappings.insert("u".to_string(), mappings.get("user").unwrap().clone());
    mappings.insert("w".to_string(), mappings.get("workdir").unwrap().clone());
    mappings.insert("c".to_string(), mappings.get("cpu-shares").unwrap().clone());
    mappings.insert("t".to_string(), mappings.get("tty").unwrap().clone());
    mappings.insert("i".to_string(), mappings.get("interactive").unwrap().clone());
    mappings.insert("m".to_string(), mappings.get("memory").unwrap().clone());
    mappings.insert("d".to_string(), mappings.get("detached").unwrap().clone());

    mappings
}

pub fn strip_quotes(val: &str) -> String {
    let trimmed = val.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn parse_key_value_list(input: &str, list_sep: char, entry_sep: char) -> IndexMap<String, Value> {
    let mut result = IndexMap::new();
    
    for item in input.split(list_sep) {
        let parts: Vec<&str> = item.splitn(2, entry_sep).collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim();
            
            if let Ok(num) = value.parse::<i64>() {
                result.insert(key, Value::Number(num.into()));
            } else {
                result.insert(key, Value::String(value.to_string()));
            }
        }
    }
    
    result
}

pub fn is_boolean_flag(flag: &str) -> bool {
    let mappings = get_mappings();
    if let Some(mapping) = mappings.get(flag) {
        matches!(mapping.arg_type, ArgType::Switch)
    } else {
        false
    }
}
