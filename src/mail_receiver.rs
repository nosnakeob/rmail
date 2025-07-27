use anyhow::Result;
use async_imap::{Client, Session, types::Fetch};
use async_native_tls::TlsConnector;
use futures::TryStreamExt;
use mail_parser::MessageParser;
use std::{convert::TryFrom, fmt};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::config::Config;

/// 邮件接收器
pub struct MailReceiver {
    config: Config,
}

/// 解析后的邮件信息
#[derive(Debug)]
pub struct ParsedEmail {
    pub subject: String,
    pub from: String,
    pub date: String,
    pub body: String,
}

impl fmt::Display for ParsedEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 处理邮件正文显示，限制长度并添加省略号
        let display_body = if self.body.len() > 200 {
            format!("{}...", self.body.chars().take(200).collect::<String>())
        } else {
            self.body.clone()
        };

        write!(
            f,
            "主题: {}\n发件人: {}\n日期: {}\n内容预览: {}",
            self.subject, self.from, self.date, display_body
        )
    }
}

impl TryFrom<&Fetch> for ParsedEmail {
    type Error = anyhow::Error;

    fn try_from(fetch: &Fetch) -> Result<Self, Self::Error> {
        let body = fetch.body().ok_or_else(|| anyhow!("邮件没有正文内容"))?;

        let parsed = MessageParser::default()
            .parse(body)
            .ok_or_else(|| anyhow!("无法解析邮件内容"))?;

        let subject = parsed.subject().unwrap_or("(无主题)").to_string();

        let from = parsed
            .from()
            .and_then(|f| f.first())
            .map(|addr| {
                format!(
                    "{} <{}>",
                    addr.name().unwrap_or(""),
                    addr.address().unwrap_or("")
                )
            })
            .unwrap_or_else(|| "(未知发件人)".to_string());

        let date = parsed
            .date()
            .map(|d| d.to_rfc822())
            .unwrap_or_else(|| "(未知日期)".to_string());

        // 获取邮件正文（不在这里限制长度，交给 Display trait 处理）
        let body = if let Some(text_body) = parsed.body_text(0) {
            text_body.to_string()
        } else if let Some(html_body) = parsed.body_html(0) {
            format!("[HTML内容] {}", html_body)
        } else {
            "(无内容)".to_string()
        };

        Ok(ParsedEmail {
            subject,
            from,
            date,
            body,
        })
    }
}

impl MailReceiver {
    /// 创建新的邮件接收器
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取最近的邮件
    pub async fn fetch_recent_emails(&self, count: usize) -> Result<Vec<ParsedEmail>> {
        println!("正在连接到 IMAP 服务器 (TLS)...");

        let imap_addr = (self.config.imap.server.as_str(), self.config.imap.port);
        let tcp_stream = TcpStream::connect(imap_addr).await?;
        let tls_connector = TlsConnector::new();
        let tls_stream = tls_connector
            .connect(&self.config.imap.server, tcp_stream.compat())
            .await?;
        let client = Client::new(tls_stream);

        self.process_emails(client, count).await
    }

    /// 处理邮件获取的通用逻辑
    async fn process_emails<T>(&self, client: Client<T>, count: usize) -> Result<Vec<ParsedEmail>>
    where
        T: futures::AsyncRead + futures::AsyncWrite + Unpin + std::fmt::Debug + Send,
    {
        println!("正在登录...");

        // 登录
        let mut imap_session = client
            .login(&self.config.email.username, &self.config.email.password)
            .await
            .map_err(|e| anyhow!("登录失败: {:?}", e.0))?;

        println!("正在选择收件箱...");

        // 选择收件箱
        imap_session.select("INBOX").await?;

        println!("正在获取邮件列表...");

        // 搜索最近的邮件
        let messages_set = imap_session.search("ALL").await?;
        let mut messages: Vec<u32> = messages_set.into_iter().collect();
        messages.sort(); // 按邮件 ID 排序

        if messages.is_empty() {
            println!("收件箱中没有邮件");
            imap_session.logout().await?;
            return Ok(vec![]);
        }

        // 获取最后几封邮件
        let start_index = if messages.len() > count {
            messages.len() - count
        } else {
            0
        };

        let recent_messages = &messages[start_index..];
        let mut parsed_emails = Vec::new();

        println!("正在解析 {} 封邮件...", recent_messages.len());

        for &msg_id in recent_messages {
            match self.fetch_and_parse_email(&mut imap_session, msg_id).await {
                Ok(email) => parsed_emails.push(email),
                Err(e) => eprintln!("解析邮件 {} 失败: {}", msg_id, e),
            }
        }

        // 登出
        imap_session.logout().await?;

        Ok(parsed_emails)
    }

    /// 获取并解析单封邮件
    async fn fetch_and_parse_email<T>(
        &self,
        session: &mut Session<T>,
        msg_id: u32,
    ) -> Result<ParsedEmail>
    where
        T: futures::AsyncRead + futures::AsyncWrite + Unpin + std::fmt::Debug + Send,
    {
        // 获取邮件内容
        let message_stream = session.fetch(&msg_id.to_string(), "RFC822").await?;
        let messages: Vec<_> = message_stream.try_collect().await?;

        if let Some(fetch) = messages.first() {
            return ParsedEmail::try_from(fetch)
                .map_err(|e| anyhow!("解析邮件 {} 失败: {}", msg_id, e));
        }

        bail!("无法获取邮件内容");
    }
}
