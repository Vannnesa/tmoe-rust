use anyhow::{Result, Context};
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn init_environment() -> Result<()> {
    // Ensure TMPDIR is set
    if env::var("TMPDIR").is_err() {
        env::set_var("TMPDIR", "/tmp");
        fs::create_dir_all("/tmp").ok();
    }

    // Initialize TMOE tool directory
    let home = env::var("HOME").context("HOME not set")?;
    let tmoe_dir = PathBuf::from(&home).join(".tmoe");
    fs::create_dir_all(&tmoe_dir).ok();

    // Set TMOE_TOOL_DIR
    env::set_var("TMOE_TOOL_DIR", &tmoe_dir);

    Ok(())
}

pub fn setup_locale() -> Result<()> {
    // Detect system locale
    let lang = env::var("LANG").unwrap_or_else(|_| "en_US.UTF-8".to_string());

    // Check if locale is available
    let locale_output = std::process::Command::new("locale")
        .arg("-a")
        .output()
        .context("Failed to get available locales")?;

    let available_locales = String::from_utf8_lossy(&locale_output.stdout);

    // If system locale is available, use it
    if available_locales.contains(&lang) {
        env::set_var("LANG", &lang);
    } else if available_locales.contains("en_US.UTF-8") {
        env::set_var("LANG", "en_US.UTF-8");
    }

    // Set UTF-8 encoding
    env::set_var("LC_ALL", "");
    env::set_var("LANGUAGE", env::var("LANG").unwrap_or_else(|_| "en_US.UTF-8".to_string()));

    Ok(())
}

pub fn get_mirror_station() -> String {
    env::var("SOURCE_MIRROR_STATION")
        .unwrap_or_else(|_| "mirrors.bfsu.edu.cn".to_string())
}

pub fn set_mirror_station(station: &str) {
    env::set_var("SOURCE_MIRROR_STATION", station);
}
