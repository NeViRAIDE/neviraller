#[derive(Debug, Clone)]
struct Dependency {
    name: String,
    command: String,
    package_manager: String,
    install_command: String,
    description: String,
    git_repo: Option<String>,
    optional: bool,
}

impl Dependency {
    fn new(
        name: &str,
        command: &str,
        package_manager: &str,
        install_command: &str,
        description: &str,
        git_repo: Option<String>,
        optional: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            package_manager: package_manager.to_string(),
            install_command: install_command.to_string(),
            description: description.to_string(),
            git_repo,
            optional,
        }
    }

    fn is_installed(&self) -> bool {
        let output = std::process::Command::new("which")
            .arg(&self.command)
            .output();

        match output {
            Ok(output) => !output.stdout.is_empty(),
            Err(_) => false,
        }
    }

    fn install(&self) -> Result<(), String> {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(&self.install_command)
            .status();

        if status.map_or(false, |s| s.success()) {
            Ok(())
        } else {
            Err(format!("Failed to install {}", self.name))
        }
    }
}
