pub mod checks;
pub mod env;

pub use checks::{
    check_linux_distro, check_architecture, check_current_user, check_is_root,
    SystemInfo, LinuxDistro, collect_system_info,
};
pub use env::{init_environment, setup_locale};
