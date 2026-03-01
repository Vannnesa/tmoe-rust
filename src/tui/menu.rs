use super::app::Language;

pub struct MenuItem {
    pub label: String,
    pub description: String,
    pub action: String,
}

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub title: String,
}

impl Menu {
    pub fn main_menu(lang: Language) -> Self {
        let items = match lang {
            Language::English => vec![
                MenuItem {
                    label: "Install GUI".to_string(),
                    description: "Install graphical desktop environment".to_string(),
                    action: "install-gui".to_string(),
                },
                MenuItem {
                    label: "Docker Management".to_string(),
                    description: "Manage Docker containers and images".to_string(),
                    action: "docker".to_string(),
                },
                MenuItem {
                    label: "Aria2 Manager".to_string(),
                    description: "Manage Aria2 download tool".to_string(),
                    action: "aria2".to_string(),
                },
                MenuItem {
                    label: "QEMU Manager".to_string(),
                    description: "Manage QEMU virtual machines".to_string(),
                    action: "qemu".to_string(),
                },
                MenuItem {
                    label: "Mirror Sources".to_string(),
                    description: "Switch package mirror sources".to_string(),
                    action: "mirror".to_string(),
                },
                MenuItem {
                    label: "Update Tools".to_string(),
                    description: "Update tmoe-linux-tools".to_string(),
                    action: "update".to_string(),
                },
                MenuItem {
                    label: "Exit".to_string(),
                    description: "Exit the application".to_string(),
                    action: "exit".to_string(),
                },
            ],
            Language::Chinese => vec![
                MenuItem {
                    label: "安装 GUI".to_string(),
                    description: "安装图形桌面环境".to_string(),
                    action: "install-gui".to_string(),
                },
                MenuItem {
                    label: "Docker 管理".to_string(),
                    description: "管理 Docker 容器和镜像".to_string(),
                    action: "docker".to_string(),
                },
                MenuItem {
                    label: "Aria2 管理器".to_string(),
                    description: "管理 Aria2 下载工具".to_string(),
                    action: "aria2".to_string(),
                },
                MenuItem {
                    label: "QEMU 管理器".to_string(),
                    description: "管理 QEMU 虚拟机".to_string(),
                    action: "qemu".to_string(),
                },
                MenuItem {
                    label: "镜像源".to_string(),
                    description: "切换软件包镜像源".to_string(),
                    action: "mirror".to_string(),
                },
                MenuItem {
                    label: "更新工具".to_string(),
                    description: "更新 tmoe-linux-tools".to_string(),
                    action: "update".to_string(),
                },
                MenuItem {
                    label: "退出".to_string(),
                    description: "退出应用程序".to_string(),
                    action: "exit".to_string(),
                },
            ],
        };

        Self {
            items,
            title: match lang {
                Language::English => "TMOE Linux Tools".to_string(),
                Language::Chinese => "TMOE Linux 工具".to_string(),
            },
        }
    }

    pub fn get_selected(&self, index: usize) -> Option<&MenuItem> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
