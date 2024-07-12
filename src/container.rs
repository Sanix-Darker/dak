use crate::{cgroups, filesystem, namespaces};

pub fn start_container(image: &str) {
    println!("Starting container {:?}", image);
    // > namespaces
    namespaces::create_namespaces(image);

    // > cgroups
    // FIXME: Since dak cgroup is already created
    // (manualy)no need to create it again
    // we just set the memory max
    cgroups::set_memory_max(b"100");

    // > filesystem
    filesystem::mount_overlayfs();

    // > networking
}
