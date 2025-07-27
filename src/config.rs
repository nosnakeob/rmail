use anyhow::Result;
use serde::Deserialize;
use std::fs;

/// 邮箱配置结构
#[derive(Debug, Deserialize)]
pub struct Config {
    pub email: EmailConfig,
    pub imap: ImapConfig,
}

/// 邮箱账户配置
#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub username: String,
    pub password: String,
}

/// IMAP 服务器配置
#[derive(Debug, Deserialize)]
pub struct ImapConfig {
    pub server: String,
    pub port: u16,
}

impl Config {
    /// 从 config.toml 文件加载配置
    pub fn load() -> Result<Self> {
        let config_content = fs::read_to_string("config.toml")
            .map_err(|_| anyhow!("无法读取 config.toml 文件，请确保文件存在"))?;

        toml::from_str(&config_content).map_err(|e| anyhow!("配置文件格式错误: {}", e))
    }
}
