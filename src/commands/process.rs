use anyhow::{Result, Context};
use std::path::Path;
use std::process::{Command, Output};

pub struct ProcessExecutor;

impl ProcessExecutor {
    pub fn execute_command(program: &str, args: &[&str]) -> Result<Output> {
        Command::new(program)
            .args(args)
            .output()
            .context(format!("Failed to execute: {}", program))
    }

    pub fn execute_command_in_dir(program: &str, args: &[&str], dir: &Path) -> Result<Output> {
        Command::new(program)
            .current_dir(dir)
            .args(args)
            .output()
            .context(format!("Failed to execute: {}", program))
    }

    pub fn check_command_exists(program: &str) -> bool {
        which::which(program).is_ok()
    }

    pub fn get_command_output(program: &str, args: &[&str]) -> Result<String> {
        let output = Self::execute_command(program, args)?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string())
    }
}
