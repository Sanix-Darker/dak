use std::{fs::OpenOptions, io::Write};

const DAK_CGROUP_PATH: &str = "/sys/fs/cgroup/dak.cgroup";

/// To set memory max
/// Usage:
/// cgroups::set_memory_max(b"100")
pub fn set_memory_max(memory_in_mb: &[u8]) {
    let memory_limit_path = format!("{}/memory.max", DAK_CGROUP_PATH);

    println!(
        "Set memory max to {:?}... in {}",
        memory_in_mb, memory_limit_path,
    );

    let mut memory_limit = OpenOptions::new()
        .write(true)
        .open(memory_limit_path)
        .unwrap();
    memory_limit.write_all(memory_in_mb).unwrap();
}

// TODO: set cpu max
