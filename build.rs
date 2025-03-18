use std::process::Command;
use std::path::Path;


fn main() {
    let switch_tools_dir = Path::new("switch-tools");

    let autogen_status = Command::new("./autogen.sh")
        .current_dir(switch_tools_dir)
        .status()
        .expect("Couldn't run autogen.sh");

    let conf_status = Command::new("./configure")
        .current_dir(switch_tools_dir)
        .status().
        expect("Couldn't run configure");

    let make_status = Command::new("make")
        .current_dir(switch_tools_dir)
        .status()
        .expect("Couldn't run make");

    if !autogen_status.success() || !conf_status.success() || !make_status.success() {
        panic!("Something went wrong trying to build switch-tools");
    }
}