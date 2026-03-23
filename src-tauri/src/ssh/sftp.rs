use crate::error::{AppError, Result};
use crate::models::SftpFileEntry;
use crate::ssh::client::SSH_SESSIONS;

/// 初始化SFTP会话
pub async fn init_sftp(session_id: &str) -> Result<()> {
    let mut sessions = SSH_SESSIONS.lock().await;

    let session = sessions.get_mut(session_id)
        .ok_or_else(|| AppError::SessionNotFound(session_id.to_string()))?;

    session.init_sftp().await
}

/// 列出远程目录（暂未完全实现）
pub async fn list_directory(_session_id: &str, _path: &str) -> Result<Vec<SftpFileEntry>> {
    Err(AppError::Ssh("SFTP功能开发中，请稍候...".to_string()))
}

/// 下载文件（暂未完全实现）
pub async fn download_file(_session_id: &str, _remote_path: &str, _local_path: &str) -> Result<()> {
    Err(AppError::Ssh("SFTP功能开发中，请稍候...".to_string()))
}

/// 上传文件（暂未完全实现）
pub async fn upload_file(_session_id: &str, _local_path: &str, _remote_path: &str) -> Result<()> {
    Err(AppError::Ssh("SFTP功能开发中，请稍候...".to_string()))
}

/// 删除文件（暂未完全实现）
pub async fn delete_file(_session_id: &str, _remote_path: &str) -> Result<()> {
    Err(AppError::Ssh("SFTP功能开发中，请稍候...".to_string()))
}

/// 删除目录（暂未完全实现）
pub async fn delete_directory(_session_id: &str, _remote_path: &str, _recursive: bool) -> Result<()> {
    Err(AppError::Ssh("SFTP功能开发中，请稍候...".to_string()))
}

/// 创建目录（暂未完全实现）
pub async fn create_directory(_session_id: &str, _remote_path: &str, _mode: u32) -> Result<()> {
    Err(AppError::Ssh("SFTP功能开发中，请稍候...".to_string()))
}

/// 重命名文件/目录（暂未完全实现）
pub async fn rename(_session_id: &str, _old_path: &str, _new_path: &str) -> Result<()> {
    Err(AppError::Ssh("SFTP功能开发中，请稍候...".to_string()))
}

/// 获取文件元数据（暂未完全实现）
pub async fn get_file_metadata(_session_id: &str, _remote_path: &str) -> Result<SftpFileEntry> {
    Err(AppError::Ssh("SFTP功能开发中，请稍候...".to_string()))
}
