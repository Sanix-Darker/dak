use std::{fs::OpenOptions, io::Write};

const DAK_CGROUP_NAME: &str = "dak.cgroup";

// TODO: pass a process id/name to attach to the cgroups.
pub fn attach_dak_cgroup(){
    let path = format!("/sys/fs/cgroup/{}", DAK_CGROUP_NAME);

    println!("Attach specs to cgroups {:?}...", path);

    let memory_limit_path = format!("{}/memory.limit_in_bytes", path);
    let mut memory_limit = OpenOptions::new().write(true).open(memory_limit_path).unwrap();
    memory_limit.write_all(b"1000").unwrap();
}
