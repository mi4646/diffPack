use crate::error::{AppError, Result};
use crate::models::{SftpFileEntry, SftpFileType};
use crate::ssh::client::SSH_SESSIONS;

/// 初始化SFTP会话
pub async fn init_sftp(session_id: &str) -> Result<()> {
    let mut sessions = SSH_SESSIONS.lock().await;

    let session = sessions.get_mut(session_id)
        .ok_or_else(|| AppError::SessionNotFound(session_id.to_string()))?;

    session.init_sftp().await
}

/// 列出远程目录
pub async fn list_directory(session_id: &str, path: &str) -> Result<Vec<SftpFileEntry>> {
    let mut sessions = SSH_SESSIONS.lock().await;

    let session = sessions.get_mut(session_id)
        .ok_or_else(|| AppError::SessionNotFound(session_id.to_string()))?;

    // 如果SFTP未初始化，先初始化
    if session.sftp().is_err() {
        session.init_sftp().await?;
    }

    let sftp = session.sftp_mut()?;

    // 规范化路径
    let normalized_path = if path.is_empty() || path == "." {
        ".".to_string()
    } else {
        path.to_string()
    };

    // 读取目录内容
    let dir_entries = sftp.read_dir(normalized_path.as_str())
        .await
        .map_err(|e| AppError::Ssh(format!("Failed to read directory '{}': {}", normalized_path, e)))?;

    let mut entries = Vec::new();

    for entry in dir_entries {
        let file_name = entry.file_name().to_string();

        // 跳过.和..目录
        if file_name == "." || file_name == ".." {
            continue;
        }

        let full_path = if normalized_path == "/" {
            format!("/{}", file_name)
        } else if normalized_path.ends_with('/') {
            format!("{}{}", normalized_path, file_name)
        } else {
            format!("{}/{}", normalized_path, file_name)
        };

        // 获取文件属性
        let metadata = entry.metadata();

        // 确定文件类型
        let file_type = if metadata.is_dir() {
            SftpFileType::Directory
        } else if metadata.is_regular() {
            SftpFileType::File
        } else if metadata.is_symlink() {
            SftpFileType::Symlink
        } else {
            SftpFileType::Other
        };

        entries.push(SftpFileEntry {
            name: file_name,
            path: full_path,
            file_type,
            size: metadata.size.unwrap_or(0),
            modified_time: metadata.mtime.unwrap_or(0) as i64,
            permissions: metadata.permissions.unwrap_or(0),
        });
    }

    // 按目录优先，然后按名称排序
    entries.sort_by(|a, b| {
        match (&a.file_type, &b.file_type) {
            (SftpFileType::Directory, SftpFileType::Directory) => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            (SftpFileType::Directory, _) => std::cmp::Ordering::Less,
            (_, SftpFileType::Directory) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(entries)
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
