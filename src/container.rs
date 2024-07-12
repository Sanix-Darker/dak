use crate::{cgroups, filesystem};

pub fn start_container(image: &str) {
    println!("Starting container {:?}", image);
    // > cgroups
    // since dak cgroup is already created (manualy)no need to create it again
    // we just set the memory max
    cgroups::set_memory_max(b"100");

    // > filesystem
    filesystem::mount_overlayfs();

    // networking
    // namespaces
}
