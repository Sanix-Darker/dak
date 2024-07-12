use crate::cgroups;

pub fn start_container(image: &str){
    // cgroups
    // since dak cgroup is already created (manualy)no need to create it again
    cgroups::attach_dak_cgroup()
    // networking
    // filesystem
    // namespaces
}
