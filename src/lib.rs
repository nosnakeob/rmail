#[macro_use]
extern crate anyhow;

pub use crate::config::Config;
pub use crate::mail_receiver::{MailReceiver, ParsedEmail};

pub mod config;
pub mod mail_receiver;

mod tests {
    use super::*;
    use anyhow::Result;
    use async_imap::{Client, Session};
    use async_native_tls::TlsConnector;
    use futures::{StreamExt, TryStreamExt};
    use mail_parser::{MessageParser, mailbox};
    use tokio::net::TcpStream;
    use tokio_util::compat::TokioAsyncReadCompatExt;

    #[tokio::test]
    async fn t_get_5_mails() -> Result<()> {
        println!("=== RMail 邮件获取演示 ===\n");

        // 加载配置
        println!("正在加载配置...");
        let config = match Config::load() {
            Ok(config) => {
                println!("配置加载成功");
                // 检查是否使用示例配置
                if config.email.username.contains("example.com") {
                    eprintln!("检测到示例配置！");
                    eprintln!("请复制 config.example.toml 为 config.toml 并填入真实的邮箱信息");
                    eprintln!("参考 README.md 了解如何配置不同邮箱服务商");
                    return Err(anyhow::anyhow!("请配置真实的邮箱信息"));
                }
                config
            }
            Err(e) => {
                eprintln!("配置加载失败: {}", e);
                eprintln!("请确保 config.toml 文件存在并配置正确");
                eprintln!("可以复制 config.example.toml 为 config.toml 开始配置");
                return Err(e);
            }
        };

        // 创建邮件接收器
        let mail_receiver = MailReceiver::new(config);

        // 获取最近的 5 封邮件
        println!("\n开始获取最近的 5 封邮件...");
        match mail_receiver.fetch_recent_emails(5).await {
            Ok(emails) => {
                if emails.is_empty() {
                    println!("没有找到邮件");
                } else {
                    println!("\n成功获取到 {} 封邮件:\n", emails.len());

                    for (index, email) in emails.iter().enumerate() {
                        println!("=== 邮件 {} ===", index + 1);
                        dbg!(email);
                    }
                }
            }
            Err(e) => {
                eprintln!("获取邮件失败: {}", e);
                eprintln!("请检查网络连接和邮箱配置");
                return Err(e);
            }
        }

        println!("程序执行完成");
        Ok(())
    }

    #[tokio::test]
    async fn t_get_box() -> Result<()> {
        let config = Config::load()?;
        // dbg!(&config);

        let imap_addr = (config.imap.server.as_str(), config.imap.port);
        let tcp_stream = TcpStream::connect(imap_addr).await?;
        let tls_connector = TlsConnector::new();
        let tls_stream = tls_connector
            .connect(&config.imap.server, tcp_stream.compat())
            .await?;
        let client = Client::new(tls_stream);

        let mut imap_session = client
            .login(&config.email.username, &config.email.password)
            .await
            .map_err(|e| anyhow!("登录失败: {:?}", e.0))?;

        // 邮件箱
        let mailboxes: Vec<_> = imap_session
            .list(None, Some("*"))
            .await?
            .map_ok(|m| m.name().to_string())
            .try_collect::<Vec<_>>()
            .await?;

        dbg!(&mailboxes);

        // 选择收件箱
        imap_session.select("INBOX").await?;

        let mut messages: Vec<_> = imap_session.search("ALL").await?.into_iter().collect();
        messages.sort();

        let mut msg_stream = imap_session
            .fetch(messages[0].to_string(), "RFC822")
            .await?;

        let msg = msg_stream.next().await.unwrap()?;

        let parsed = MessageParser::new()
            .parse(msg.body().unwrap())
            .unwrap();
        dbg!(parsed.from());

        // // // 登出
        // imap_session.logout().await?;

        Ok(())
    }
}
