use std::process::Command;

pub fn install_packages(packages: &Vec<String>) -> bool {
    let child = Command::new("nix profile install").args(packages).spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => status.success(),
            Err(_) => false,
        },
        Err(e) => false,
    }
}

pub fn remove_packages(packages: &Vec<String>) -> bool {
    let child = Command::new("nix profile remove").args(packages).spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => status.success(),
            Err(_) => false,
        },
        Err(e) => false,
    }
}
