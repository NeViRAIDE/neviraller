use std::process::Command;

pub fn detect_package_managers() -> Vec<String> {
    let managers = vec![
        "apt", "yum", "dnf", "pacman", "zypper", "brew", "yay", "paru",
    ];
    let mut available_managers = Vec::new();

    for manager in managers {
        let output = Command::new("which")
            .arg(manager)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            available_managers.push(manager.to_string());
        }
    }

    available_managers
}
