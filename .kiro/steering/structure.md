# 项目结构

## 根目录

```
rmail/
├── config.toml         # 邮件配置文件（不在 git 中）
├── .gitignore          # Git 忽略规则
├── Cargo.toml          # 项目清单和依赖
├── Cargo.lock          # 依赖锁定文件
├── README.md           # 项目文档（中文）
├── src/                # 源代码目录
│   ├── config.rs       # 配置管理模块
│   ├── lib.rs          # 库根文件
│   ├── main.rs         # 应用程序入口
│   ├── mail_receiver.rs # 邮件接收模块
│   └── mail_sender.rs  # 邮件发送模块
└── target/             # 构建产物（不在 git 中）
```

## 源代码组织

### 核心文件

- **`src/main.rs`**: 应用程序入口点，包含交互式 CLI 菜单
- **`src/lib.rs`**: 库根文件，导出公共 API
- **`src/config.rs`**: 配置管理模块，处理 TOML 配置文件
- **`src/mail_receiver.rs`**: IMAP 邮件接收功能
- **`src/mail_sender.rs`**: SMTP 邮件发送功能

### 模块结构

- **库设计**: 代码既作为库又作为二进制文件
- **公共 API**: 主要类型通过 `lib.rs` 导出
- **关注点分离**: 发送和接收功能分离到不同模块

## 命名约定

- **结构体**: PascalCase（例如：`MailReceiver`、`ParsedEmail`）
- **函数**: snake_case（例如：`send_text_email`、`fetch_recent_emails`）
- **变量**: snake_case 并使用描述性名称
- **常量**: 环境变量使用 SCREAMING_SNAKE_CASE

## 代码组织模式

- **构造器模式**: 使用 `new()` 方法进行结构体初始化
- **建造者模式**: 用于邮件消息构建
- **错误上下文**: 使用中文的描述性错误消息
- **配置加载**: 在构造器中从 TOML 文件加载配置
- **配置验证**: 启动时验证配置的完整性和有效性

## 语言和交流标准

- **主要语言**: 所有项目交流使用中文
- **文档**: README、注释和技术文档使用中文
- **用户界面**: CLI 提示和消息使用中文
- **代码审查**: 使用中文进行审查和讨论
- **错误处理**: 面向用户的错误消息使用中文并提供上下文

## 文件约定

- **配置**: 敏感数据使用 `config.toml`（从 git 中排除）
- **文档**: 面向用户的内容使用中文
- **注释**: 代码文档使用中文
- **错误消息**: 用户反馈使用中文
