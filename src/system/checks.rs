use anyhow::{Result, Context};
use std::fs;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub distro: LinuxDistro,
    pub architecture: String,
    pub username: String,
    pub group: String,
    pub is_root: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinuxDistro {
    Ubuntu,
    Debian,
    ArchLinux,
    Fedora,
    CentOS,
    Alpine,
    Unknown(String),
}

impl std::fmt::Display for LinuxDistro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinuxDistro::Ubuntu => write!(f, "Ubuntu"),
            LinuxDistro::Debian => write!(f, "Debian"),
            LinuxDistro::ArchLinux => write!(f, "ArchLinux"),
            LinuxDistro::Fedora => write!(f, "Fedora"),
            LinuxDistro::CentOS => write!(f, "CentOS"),
            LinuxDistro::Alpine => write!(f, "Alpine"),
            LinuxDistro::Unknown(name) => write!(f, "{}", name),
        }
    }
}

pub fn check_linux_distro() -> Result<LinuxDistro> {
    // Try /etc/os-release first (systemd standard)
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        if content.contains("ubuntu") || content.contains("Ubuntu") {
            return Ok(LinuxDistro::Ubuntu);
        }
        if content.contains("debian") && !content.contains("ubuntu") {
            return Ok(LinuxDistro::Debian);
        }
        if content.contains("arch") || content.contains("Arch") {
            return Ok(LinuxDistro::ArchLinux);
        }
        if content.contains("fedora") || content.contains("Fedora") {
            return Ok(LinuxDistro::Fedora);
        }
        if content.contains("centos") || content.contains("CentOS") {
            return Ok(LinuxDistro::CentOS);
        }
        if content.contains("alpine") || content.contains("Alpine") {
            return Ok(LinuxDistro::Alpine);
        }

        // Extract NAME field
        if let Some(line) = content.lines().find(|l| l.starts_with("NAME=")) {
            let name = line.strip_prefix("NAME=").unwrap_or("")
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
            return Ok(LinuxDistro::Unknown(name));
        }
    }

    // Fallback: check /etc/issue
    if let Ok(content) = fs::read_to_string("/etc/issue") {
        if content.contains("Ubuntu") {
            return Ok(LinuxDistro::Ubuntu);
        }
        if content.contains("Debian") {
            return Ok(LinuxDistro::Debian);
        }
        if content.contains("Arch") {
            return Ok(LinuxDistro::ArchLinux);
        }
    }

    Ok(LinuxDistro::Unknown("Linux".to_string()))
}

pub fn check_architecture() -> Result<String> {
    let output = std::process::Command::new("uname")
        .arg("-m")
        .output()
        .context("Failed to detect architecture")?;

    let arch = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    Ok(match arch.as_str() {
        "aarch64" => "ARM64/aarch64".to_string(),
        "x86_64" => "x86_64/amd64".to_string(),
        "i686" => "i686/i386".to_string(),
        "armv7l" => "ARMv7".to_string(),
        _ => arch,
    })
}

pub fn check_current_user() -> Result<(String, String)> {
    let output = std::process::Command::new("id")
        .arg("-un")
        .output()
        .context("Failed to get username")?;

    let username = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    let group_output = std::process::Command::new("id")
        .arg("-gn")
        .output()
        .context("Failed to get group")?;

    let group = String::from_utf8_lossy(&group_output.stdout)
        .trim()
        .to_string();

    Ok((username, group))
}

pub fn check_is_root() -> bool {
    std::process::Command::new("id")
        .arg("-u")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .eq("0")
        })
        .unwrap_or(false)
}

pub fn collect_system_info() -> Result<SystemInfo> {
    let distro = check_linux_distro()?;
    let architecture = check_architecture()?;
    let (username, group) = check_current_user()?;
    let is_root = check_is_root();

    Ok(SystemInfo {
        distro,
        architecture,
        username,
        group,
        is_root,
    })
}
