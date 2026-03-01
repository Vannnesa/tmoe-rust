# 快速启动指南

## 编译和运行

### 编译项目
```bash
cd /home/Vannesa/Rust
source $HOME/.cargo/env
cargo build --release
```

### 运行二进制
```bash
# 显示系统信息
./target/release/tmoe-linux-tools info

# 检查依赖
./target/release/tmoe-linux-tools install-deps

# 列出镜像源
./target/release/tmoe-linux-tools --mirror-list

# 显示帮助
./target/release/tmoe-linux-tools --help

# 启动交互式 TUI (需要终端)
./target/release/tmoe-linux-tools --interactive
# 或简单地运行
./target/release/tmoe-linux-tools
```

## 键盘快捷键

| 按键 | 功能 |
|-----|------|
| **↑** 或 **k** | 上移菜单 |
| **↓** 或 **j** | 下移菜单 |
| **Enter** | 选择当前项目 |
| **ESC** 或 **q** | 返回/退出 |
| **Ctrl-C** | 强制退出 |

## 可用命令

```bash
# 子命令
tmoe docker          # Docker 管理
tmoe aria2           # Aria2 管理
tmoe qemu            # QEMU 虚拟机
tmoe file            # 文件浏览器
tmoe install-gui     # 安装 GUI
tmoe remove-gui      # 移除 GUI
tmoe tuna            # 切换到 BFSU 镜像
tmoe ppa             # PPA 管理
tmoe info            # 系统信息

# 选项
tmoe -i              # 交互模式 (默认)
tmoe -m              # 显示镜像源
tmoe -u              # 更新工具
tmoe -h              # 显示帮助
```

## 项目信息

- **语言**: Rust 1.93.1
- **二进制大小**: 1.7 MB
- **编译时间**: ~6-10 秒 (release)
- **依赖**: 21 个 crates

## 故障排除

### 编译失败
```bash
# 检查 Rust 版本
rustc --version

# 如果需要更新
rustup update
```

### 权限错误
某些命令需要 root 权限:
```bash
sudo ./target/release/tmoe-linux-tools install-gui
```

### 日志调试
```bash
RUST_LOG=debug ./target/release/tmoe-linux-tools info
```

## 下一步

1. 查看 `README.md` 获取完整文档
2. 查看 `COMPLETION_SUMMARY.md` 了解项目进度
3. 查看 `src/` 目录查阅源代码
4. 运行 `cargo doc --open` 生成 API 文档

