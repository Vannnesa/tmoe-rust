use anyhow::Result;

pub struct QemuManager;

impl QemuManager {
    pub fn list_vms() -> Result<Vec<String>> {
        // This would typically query libvirt or qemu directly
        // For now, return empty list
        Ok(Vec::new())
    }

    pub fn start(vm_name: &str) -> Result<()> {
        std::process::Command::new("qemu-system-x86_64")
            .arg("-name")
            .arg(vm_name)
            .spawn()?;
        Ok(())
    }

    pub fn stop(vm_name: &str) -> Result<()> {
        std::process::Command::new("pkill")
            .arg("-f")
            .arg(format!("qemu.*{}", vm_name))
            .output()?;
        Ok(())
    }

    pub fn check_installed() -> bool {
        which::which("qemu-system-x86_64").is_ok()
    }

    pub fn create_vm(name: &str, size: &str) -> Result<()> {
        std::process::Command::new("qemu-img")
            .arg("create")
            .arg("-f")
            .arg("qcow2")
            .arg(name)
            .arg(size)
            .output()?;
        Ok(())
    }
}
