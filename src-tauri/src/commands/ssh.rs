use crate::models::{ConnectionStatus, DiffResult, SshConfig, SshConfigEntry};
use crate::ssh::{connect, disconnect, get_remote_commits as ssh_get_remote_commits, get_remote_diff as ssh_get_remote_diff, parse_ssh_config_file, test_connection};
use crate::ssh::get_default_ssh_config_path;
use tauri::command;

/// 解析 SSH Config 文件
#[command]
pub async fn parse_ssh_config() -> Result<Vec<SshConfigEntry>, String> {
    let config_path = get_default_ssh_config_path();
    let entries = parse_ssh_config_file(&config_path).map_err(|e| e.to_string())?;
    Ok(entries)
}

/// 测试 SSH 连接
#[command]
pub async fn test_ssh_connection(config: SshConfig) -> Result<ConnectionStatus, String> {
    let status = test_connection(&config).await.map_err(|e| e.to_string())?;
    Ok(status)
}

/// 建立 SSH 连接
#[command]
pub async fn connect_ssh(config: SshConfig) -> Result<String, String> {
    let session_id = connect(&config).await.map_err(|e| e.to_string())?;
    Ok(session_id)
}

/// 断开 SSH 连接
#[command]
pub async fn disconnect_ssh(session_id: String) -> Result<(), String> {
    disconnect(&session_id).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取远程 Commits
#[command]
pub async fn get_remote_commits(
    session_id: String,
    repo_path: String,
    limit: Option<usize>,
) -> Result<Vec<crate::models::CommitInfo>, String> {
    let commits = ssh_get_remote_commits(&session_id, &repo_path, limit.unwrap_or(100)).await
        .map_err(|e| e.to_string())?;
    Ok(commits)
}

/// 获取远程差异
#[command]
pub async fn get_remote_diff(
    session_id: String,
    repo_path: String,
    from_commit: String,
    to_commit: String,
) -> Result<DiffResult, String> {
    let diff = ssh_get_remote_diff(&session_id, &repo_path, &from_commit, &to_commit).await
        .map_err(|e| e.to_string())?;
    Ok(diff)
}

/// 打包远程变更（占位实现）
#[command]
pub async fn pack_remote_changes(
    _session_id: String,
    _repo_path: String,
    _changes: Vec<crate::models::FileChange>,
    _options: crate::models::PackOptions,
    _app_handle: tauri::AppHandle,
) -> Result<crate::models::PackResult, String> {
    // TODO: 实现远程文件下载和打包
    Err("Remote pack not implemented yet".to_string())
}
