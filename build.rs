use std::path::Path;
use std::process::Command;

fn main() {
    let switch_tools_dir = Path::new("switch-tools");

    let aclocal_status = Command::new("aclocal")
        .current_dir(switch_tools_dir)
        .status()
        .expect("Couldn't run aclocal");

    let autoconf_status = Command::new("autoconf")
        .current_dir(switch_tools_dir)
        .status()
        .expect("Couldn't run autoconf");

    let automake_status = Command::new("automake")
        .current_dir(switch_tools_dir)
        .arg("--add-missing")
        .arg("-c")
        .status()
        .expect("Couldn't run automake");

    let conf_status = Command::new("./configure")
        .current_dir(switch_tools_dir)
        .status()
        .expect("Couldn't run configure");

    let make_status = Command::new("make")
        .current_dir(switch_tools_dir)
        .status()
        .expect("Couldn't run make");

    assert!(
        aclocal_status.success(),
        "Something went wrong while building switch-tools"
    );
    assert!(
        autoconf_status.success(),
        "Something went wrong while building switch-tools"
    );
    assert!(
        automake_status.success(),
        "Something went wrong while building switch-tools"
    );
    assert!(
        conf_status.success(),
        "Something went wrong while building switch-tools"
    );
    assert!(
        make_status.success(),
        "Something went wrong while building switch-tools"
    );
}
