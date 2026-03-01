use anyhow::{Result, Context};
use std::path::PathBuf;

pub struct GitManager {
    repo_path: PathBuf,
}

impl GitManager {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    pub fn clone(url: &str, dest: &str) -> Result<()> {
        let output = std::process::Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(dest)
            .output()
            .context("Failed to execute git clone")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git clone failed: {}", error);
        }

        Ok(())
    }

    pub fn pull_with_rebase(&self) -> Result<()> {
        let output = std::process::Command::new("git")
            .current_dir(&self.repo_path)
            .arg("pull")
            .arg("--rebase")
            .arg("--stat")
            .arg("origin")
            .arg("master")
            .arg("--allow-unrelated-histories")
            .output()
            .context("Failed to execute git pull")?;

        if !output.status.success() {
            // Try to skip rebase
            let _skip = std::process::Command::new("git")
                .current_dir(&self.repo_path)
                .arg("rebase")
                .arg("--skip")
                .output();
        }

        Ok(())
    }

    pub fn reset_hard(&self) -> Result<()> {
        std::process::Command::new("git")
            .current_dir(&self.repo_path)
            .arg("reset")
            .arg("--hard")
            .output()
            .context("Failed to execute git reset")?;

        Ok(())
    }

    pub fn fetch_all(&self) -> Result<()> {
        std::process::Command::new("git")
            .current_dir(&self.repo_path)
            .arg("fetch")
            .arg("--all")
            .output()
            .context("Failed to execute git fetch")?;

        Ok(())
    }
}
