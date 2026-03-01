use anyhow::Result;

pub struct Aria2Manager;

impl Aria2Manager {
    pub fn get_status() -> Result<String> {
        let output = std::process::Command::new("aria2c")
            .arg("--version")
            .output()?;

        if output.status.success() {
            Ok("Aria2 is installed".to_string())
        } else {
            Ok("Aria2 is not installed".to_string())
        }
    }

    pub fn start_daemon() -> Result<()> {
        std::process::Command::new("aria2c")
            .arg("--enable-rpc")
            .arg("--rpc-listen-all=true")
            .spawn()?;
        Ok(())
    }

    pub fn download(url: &str, output: Option<&str>) -> Result<()> {
        let mut cmd = std::process::Command::new("aria2c");
        cmd.arg(url);
        
        if let Some(path) = output {
            cmd.arg("-o").arg(path);
        }
        
        cmd.output()?;
        Ok(())
    }

    pub fn check_installed() -> bool {
        which::which("aria2c").is_ok()
    }

    pub fn get_version() -> Result<String> {
        let output = std::process::Command::new("aria2c")
            .arg("--version")
            .output()?;

        let version = String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("unknown")
            .to_string();

        Ok(version)
    }
}
