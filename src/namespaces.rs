use nix::sched::{unshare, CloneFlags};
use nix::unistd::sethostname;

pub fn create_namespaces(container: &str) {
    unshare(
        CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNET
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWIPC,
    )
    .expect("Failed to unshare numspaces");

    // set the host for the new UTS
    sethostname(container).expect("Failed to set hostname");
}
