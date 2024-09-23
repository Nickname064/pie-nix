use std::process::{Command, Stdio};

/// Returns true if the `nix` command can be run correctly
pub fn require_nix() -> bool {

    let child = Command::new("which")
        .args(vec!["nix"])
        .stdout(Stdio::null())
        .spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => status.success(),
            _ => false,
        },
        _ => false,
    }
}

pub fn install_package(package: &str) -> bool {
    let child = Command::new("nix")
        .args(vec!["profile", "install", package])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => status.success(),
            Err(_) => false,
        },
        Err(e) => false,
    }
}

pub fn remove_package(package: &str) -> bool {
    let child = Command::new("nix")
        .args(vec!["profile", "remove", package])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => status.success(),
            Err(_) => false,
        },
        Err(e) => false,
    }
}
