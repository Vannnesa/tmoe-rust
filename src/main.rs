use clap::{Parser, Subcommand};
use tmoe_linux_tools::{
    system::{self, SystemInfo},
    utils::ColoredOutput,
    APP_VERSION,
};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "tmoe")]
#[command(version = APP_VERSION)]
#[command(about = "TMOE Linux Tools - System Management Utility", long_about = None)]
#[command(disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Interactive mode
    #[arg(short = 'i', long)]
    interactive: bool,

    /// Show mirror sources
    #[arg(short = 'm', long = "mirror-list")]
    mirror_list: bool,

    /// Update tools
    #[arg(short = 'u', long = "upgrade")]
    upgrade: bool,

    /// Help text
    #[arg(short = 'h', long)]
    help_text: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Docker management
    Docker,

    /// Aria2 download manager
    Aria2,

    /// QEMU virtual machine
    Qemu,

    /// File browser
    File,

    /// Install GUI
    #[command(name = "install-gui")]
    InstallGui,

    /// Remove GUI
    #[command(name = "remove-gui")]
    RemoveGui,

    /// Switch to BFSU mirror
    Tuna,

    /// Add PPA source
    Ppa,

    /// Check dependencies
    #[command(name = "install-deps")]
    InstallDeps,

    /// System information
    Info,
}

fn main() -> Result<()> {
    // Initialize environment
    system::env::init_environment()?;
    system::env::setup_locale()?;

    let cli = Cli::parse();

    // Collect system info
    let sys_info = system::collect_system_info()?;

    if cli.help_text {
        print_help();
        return Ok(());
    }

    // Handle subcommands
    if let Some(command) = cli.command {
        handle_command(command, &sys_info)?;
    } else if cli.interactive || (!cli.mirror_list && !cli.upgrade && !cli.help_text) {
        // Start interactive TUI
        run_interactive_mode(&sys_info)?;
    } else if cli.mirror_list {
        handle_mirror_list(&sys_info)?;
    } else if cli.upgrade {
        handle_upgrade(&sys_info)?;
    }

    Ok(())
}

fn handle_command(cmd: Commands, sys_info: &SystemInfo) -> Result<()> {
    match cmd {
        Commands::Docker => {
            ColoredOutput::info("Docker management not yet implemented");
        }
        Commands::Aria2 => {
            ColoredOutput::info("Aria2 management not yet implemented");
        }
        Commands::Qemu => {
            ColoredOutput::info("QEMU management not yet implemented");
        }
        Commands::File => {
            ColoredOutput::info("File browser not yet implemented");
        }
        Commands::InstallGui => {
            ColoredOutput::info("GUI installation not yet implemented");
        }
        Commands::RemoveGui => {
            ColoredOutput::info("GUI removal not yet implemented");
        }
        Commands::Tuna => {
            ColoredOutput::info("BFSU mirror switch not yet implemented");
        }
        Commands::Ppa => {
            ColoredOutput::info("PPA management not yet implemented");
        }
        Commands::InstallDeps => {
            check_dependencies(sys_info)?;
        }
        Commands::Info => {
            print_system_info(sys_info);
        }
    }

    Ok(())
}

fn run_interactive_mode(sys_info: &SystemInfo) -> Result<()> {
    ColoredOutput::header("TMOE Linux Tools v0.1.0");
    ColoredOutput::info("Interactive mode");
    println!("System: {}", sys_info.distro);
    println!("Architecture: {}", sys_info.architecture);
    println!("User: {} ({})", sys_info.username, sys_info.group);
    println!("Root: {}", if sys_info.is_root { "Yes" } else { "No" });
    println!();
    ColoredOutput::info("Interactive TUI not yet implemented - use subcommands");

    Ok(())
}

fn handle_mirror_list(_sys_info: &SystemInfo) -> Result<()> {
    ColoredOutput::header("Available Mirror Sources");
    println!("  1. BFSU (Beijing)");
    println!("  2. Tsinghua University");
    println!("  3. Aliyun");
    println!("  4. NetEase");
    Ok(())
}

fn handle_upgrade(sys_info: &SystemInfo) -> Result<()> {
    if !sys_info.is_root {
        ColoredOutput::warning("Update requires root privileges");
        return Ok(());
    }

    ColoredOutput::info("Checking for updates...");
    ColoredOutput::info("Update not yet implemented");

    Ok(())
}

fn check_dependencies(_sys_info: &SystemInfo) -> Result<()> {
    ColoredOutput::header("Checking Dependencies");

    let required = vec!["git", "curl", "wget"];
    let mut missing = Vec::new();

    for cmd in required {
        if which::which(cmd).is_ok() {
            ColoredOutput::success(&format!("✓ {} found", cmd));
        } else {
            ColoredOutput::warning(&format!("✗ {} not found", cmd));
            missing.push(cmd);
        }
    }

    if !missing.is_empty() {
        println!();
        ColoredOutput::warning(&format!(
            "Missing: {}",
            missing.join(", ")
        ));
    }

    Ok(())
}

fn print_system_info(sys_info: &SystemInfo) {
    ColoredOutput::header("System Information");
    println!("Distribution: {}", sys_info.distro);
    println!("Architecture: {}", sys_info.architecture);
    println!("Username: {}", sys_info.username);
    println!("Group: {}", sys_info.group);
    println!("Root Access: {}", if sys_info.is_root { "Yes" } else { "No" });
}

fn print_help() {
    ColoredOutput::header("TMOE Linux Tools Help");
    println!("Usage: tmoe [OPTIONS] [COMMAND]");
    println!();
    println!("Commands:");
    println!("  docker          Docker management");
    println!("  aria2           Aria2 download manager");
    println!("  qemu            QEMU virtual machines");
    println!("  file            File browser");
    println!("  install-gui     Install graphical desktop");
    println!("  remove-gui      Remove graphical desktop");
    println!("  tuna            Switch to BFSU mirror");
    println!("  ppa             Add PPA sources (Debian/Ubuntu)");
    println!("  info            Show system information");
    println!();
    println!("Options:");
    println!("  -i, --interactive   Interactive mode (default)");
    println!("  -m, --mirror-list   Show available mirrors");
    println!("  -u, --upgrade       Update tools");
    println!("  -h, --help          Show this help message");
}
