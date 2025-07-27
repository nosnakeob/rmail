# RMail - Rust 邮件获取演示

一个使用 async-imap 和 TOML 配置的简洁邮件获取演示程序。

## 功能特性

- 使用 async-imap 异步获取邮件
- 支持 TLS/SSL 安全连接
- TOML 配置文件管理
- 邮件内容解析和显示
- 中文界面和错误提示

## 快速开始

### 1. 配置邮箱

复制 `config.example.toml` 为 `config.toml` 并填入你的邮箱信息：

```toml
[email]
username = "your_email@example.com"
password = "your_password"

[imap]
server = "imap.example.com"
port = 993
use_tls = true
```

### 2. 运行程序

```bash
cargo run
```

## 常见邮箱配置

### Gmail
```toml
[email]
username = "your_email@gmail.com"
password = "your_app_password"  # 需要使用应用专用密码

[imap]
server = "imap.gmail.com"
port = 993
use_tls = true
```

### QQ邮箱
```toml
[email]
username = "your_email@qq.com"
password = "your_authorization_code"  # 需要使用授权码

[imap]
server = "imap.qq.com"
port = 993
use_tls = true
```

### Outlook/Hotmail
```toml
[email]
username = "your_email@outlook.com"
password = "your_password"

[imap]
server = "outlook.office365.com"
port = 993
use_tls = true
```

## 注意事项

1. 确保邮箱服务商已启用 IMAP 访问
2. 某些邮箱（如 Gmail、QQ邮箱）需要使用应用专用密码或授权码
3. 配置文件包含敏感信息，请勿提交到版本控制系统

## 依赖项

- `async-imap` - IMAP 客户端
- `async-native-tls` - TLS 支持
- `tokio` - 异步运行时
- `tokio-util` - Tokio 兼容性工具
- `mail-parser` - 邮件解析
- `anyhow` - 错误处理
- `serde` - 序列化支持
- `toml` - TOML 配置解析

## 构建命令

```bash
# 检查代码
cargo check

# 编译项目
cargo build

# 运行程序
cargo run

# 发布构建
cargo build --release
```
