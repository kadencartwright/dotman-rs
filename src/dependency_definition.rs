use serde_derive::Deserialize;
use std::process::Command;
use which::which;

#[derive(Debug, Deserialize)]
pub struct DependencyDefinition {
    name: String,
    install_commands: Vec<String>,
}

impl DependencyDefinition {
    pub fn new(name: String, install_commands: Vec<String>, install_check: String) -> Self {
        DependencyDefinition {
            name,
            install_commands,
        }
    }
}

impl DependencyDefinition {
    pub fn install(&self) {
        if self.check_already_installed() {
            return;
        }
        for command in self.install_commands.iter() {
            Command::new("sh").arg(command).output();
        }
    }
    /// returns true if the install check passes
    pub fn check_already_installed(&self) -> bool {
        which(self.name).is_ok()
    }
}
