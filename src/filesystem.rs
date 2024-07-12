use nix::mount::{mount, MsFlags};
use std::ffi::CString;

pub fn mount_overlayfs() {
    let lowerdir = "./lower";
    let upperdir = "./upper";
    let workdir = "./work";
    let mergedir = "./merge";

    std::fs::create_dir_all(lowerdir).expect("Failed to create lowerdir");
    std::fs::create_dir_all(upperdir).expect("Failed to create upperdir");
    std::fs::create_dir_all(workdir).expect("Failed to create workdir");
    std::fs::create_dir_all(mergedir).expect("Failed to create mergedir");

    let options = format!(
        "lowerdir={},upperdir={},workdir={}",
        lowerdir, upperdir, workdir,
    );

    mount(
        Some("overlay"),
        mergedir,
        Some("overlay"),
        MsFlags::empty(),
        Some(&*CString::new(options).unwrap()),
    )
    .expect("Failed to mount overlay filesystem");
}
