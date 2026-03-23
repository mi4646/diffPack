use serde::{Deserialize, Serialize};

/// SSH 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_method: AuthMethod,
}

/// 认证方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum AuthMethod {
    Password { password: String },
    KeyFile { key_path: String, passphrase: Option<String> },
    SshAgent,
}

/// SSH Config 条目
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshConfigEntry {
    pub host: String,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub identity_file: Option<String>,
}

/// 连接状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatus {
    pub connected: bool,
    pub server_info: Option<String>,
    pub error: Option<String>,
}

/// SFTP 文件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SftpFileType {
    File,
    Directory,
    Symlink,
    Other,
}

/// SFTP 文件条目
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SftpFileEntry {
    pub name: String,
    pub path: String,
    pub file_type: SftpFileType,
    pub size: u64,
    pub modified_time: i64,
    pub permissions: u32,
}

/// SFTP 文件传输进度
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SftpTransferProgress {
    pub total: u64,
    pub transferred: u64,
    pub percentage: f64,
}
