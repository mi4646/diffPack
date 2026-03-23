use crate::error::{AppError, Result};
use crate::models::SshConfig;
use russh::client::Config;
use russh_sftp::client::SftpSession;
use std::sync::Arc;

/// 封装SSH会话
pub struct SshSession {
    pub session: russh::client::Handle<ClientHandler>,
    pub sftp: Option<SftpSession>,
}

/// 客户端处理器
pub struct ClientHandler;

#[async_trait::async_trait]
impl russh::client::Handler for ClientHandler {
    type Error = russh::Error;

    fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::PublicKey,
    ) -> impl std::future::Future<Output = std::result::Result<bool, Self::Error>> + Send {
        // 暂时信任所有服务器密钥，后续可以添加主机密钥验证功能
        async move { Ok(true) }
    }
}

/// 创建SSH连接
pub async fn create_session(config: &SshConfig) -> Result<SshSession> {
    // 创建客户端配置
    let client_config = Arc::new(Config::default());

    // 连接到服务器
    let addr = (config.host.as_str(), config.port);
    let handler = ClientHandler;
    let session = russh::client::connect(client_config, addr, handler).await?;

    Ok(SshSession {
        session,
        sftp: None,
    })
}

impl SshSession {
    /// 执行命令
    pub async fn execute_command(&mut self, command: &str) -> Result<String> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut output = String::new();
        let mut stderr = String::new();
        let mut exit_status = 0;

        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };

            match msg {
                russh::ChannelMsg::Data { data } => {
                    output.push_str(&String::from_utf8_lossy(&data));
                }
                russh::ChannelMsg::ExtendedData { data, ext: 1 } => {
                    stderr.push_str(&String::from_utf8_lossy(&data));
                }
                russh::ChannelMsg::ExitStatus { exit_status: status } => {
                    exit_status = status;
                }
                russh::ChannelMsg::Eof => break,
                russh::ChannelMsg::Close => break,
                _ => {}
            }
        }

        let _ = channel.close().await;

        if exit_status != 0 && !stderr.is_empty() {
            return Err(AppError::Ssh(format!(
                "Command failed with exit code {}: {}",
                exit_status, stderr
            )));
        }

        Ok(output)
    }

    /// 初始化SFTP会话
    pub async fn init_sftp(&mut self) -> Result<()> {
        let channel = self.session.channel_open_session().await?;
        channel.request_subsystem(true, "sftp").await?;
        let sftp = SftpSession::new(channel.into_stream()).await
            .map_err(|e| AppError::Ssh(e.to_string()))?;
        self.sftp = Some(sftp);
        Ok(())
    }

    /// 获取SFTP客户端
    pub fn sftp(&self) -> Result<&SftpSession> {
        self.sftp.as_ref().ok_or_else(|| AppError::Ssh("SFTP not initialized".to_string()))
    }

    /// 获取可变SFTP客户端
    pub fn sftp_mut(&mut self) -> Result<&mut SftpSession> {
        self.sftp.as_mut().ok_or_else(|| AppError::Ssh("SFTP not initialized".to_string()))
    }
}
