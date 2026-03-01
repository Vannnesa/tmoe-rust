use anyhow::Result;

pub struct DockerManager;

impl DockerManager {
    pub fn list_containers() -> Result<Vec<String>> {
        let output = std::process::Command::new("docker")
            .arg("ps")
            .arg("-a")
            .arg("--format")
            .arg("{{.Names}}")
            .output()?;

        let containers = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();

        Ok(containers)
    }

    pub fn start(container: &str) -> Result<()> {
        std::process::Command::new("docker")
            .arg("start")
            .arg(container)
            .output()?;
        Ok(())
    }

    pub fn stop(container: &str) -> Result<()> {
        std::process::Command::new("docker")
            .arg("stop")
            .arg(container)
            .output()?;
        Ok(())
    }

    pub fn remove(container: &str) -> Result<()> {
        std::process::Command::new("docker")
            .arg("rm")
            .arg(container)
            .output()?;
        Ok(())
    }

    pub fn list_images() -> Result<Vec<String>> {
        let output = std::process::Command::new("docker")
            .arg("images")
            .arg("--format")
            .arg("{{.Repository}}:{{.Tag}}")
            .output()?;

        let images = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();

        Ok(images)
    }

    pub fn pull(image: &str) -> Result<()> {
        std::process::Command::new("docker")
            .arg("pull")
            .arg(image)
            .output()?;
        Ok(())
    }

    pub fn check_daemon() -> bool {
        std::process::Command::new("docker")
            .arg("ps")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}
