use std::error::Error;
use std::process::Command;
use std::str::from_utf8;

use std::fmt;

#[derive(Debug)]
struct NeovimVersionError {
    message: String,
}

impl NeovimVersionError {
    fn new(msg: &str) -> NeovimVersionError {
        NeovimVersionError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for NeovimVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for NeovimVersionError {}

pub async fn check_neovim_version(new_version_line: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("nvim").arg("--version").output()?;

    let current_version_output = from_utf8(&output.stdout)?;

    let current_version_line = current_version_output.lines().next().ok_or_else(|| {
        Box::new(NeovimVersionError::new(
            "Failed to get the current Neovim version.",
        )) as Box<dyn Error>
    })?;

    let current_version = current_version_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| {
            Box::new(NeovimVersionError::new(
                "Failed to parse the current Neovim version.",
            )) as Box<dyn Error>
        })?;

    let new_version = new_version_line.lines().next().ok_or_else(|| {
        Box::new(NeovimVersionError::new(
            "Failed to retrieve new version information.",
        )) as Box<dyn Error>
    })?;

    if current_version == new_version {
        Err(Box::new(NeovimVersionError::new("Already up to date.")) as Box<dyn Error>)
    } else {
        Ok(format!(
            "Current installed Neovim version: {}\nThere is a newer version of Neovim Nightly available: {}",
            current_version, new_version
        ))
    }
}
