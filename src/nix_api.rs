use std::process::Command;

/// Returns true if the `nix` command can be run correctly
pub fn require_nix() -> bool {
    let child = Command::new("nix").spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => status.success(),
            _ => false,
        },
        _ => false,
    }
}

pub fn install_package(pkg: &str) -> bool {
    let child = Command::new("nix").args(vec!["profile", "install", pkg]).spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => status.success(),
            Err(_) => false,
        },
        Err(e) => false,
    }
}

pub fn remove_package(packages: &str) -> bool {
    let child = Command::new("nix profile remove").arg(packages).spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => status.success(),
            Err(_) => false,
        },
        Err(e) => false,
    }
}
