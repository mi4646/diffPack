use crate::error::{AppError, Result};
use crate::models::{CommitInfo, ConnectionStatus, DiffResult, FileChange, FileChangeType, SshConfig};
use crate::ssh::auth::authenticate;
use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref SSH_SESSIONS: Arc<Mutex<HashMap<String, ssh2::Session>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// 生成会话 ID
fn generate_session_id() -> String {
    format!("sess_{}", chrono::Utc::now().timestamp_millis())
}

/// 测试 SSH 连接
pub fn test_connection(config: &SshConfig) -> Result<ConnectionStatus> {
    let tcp = TcpStream::connect((config.host.as_str(), config.port))
        .map_err(|e| AppError::Ssh(format!("Connection failed: {}", e)))?;

    let mut sess = ssh2::Session::new()
        .map_err(|e| AppError::Ssh(format!("Session creation failed: {}", e)))?;

    sess.set_tcp_stream(tcp);
    sess.handshake()
        .map_err(|e| AppError::Ssh(format!("Handshake failed: {}", e)))?;

    authenticate(&sess, &config.username, &config.auth_method)?;

    let server_info = format!("{}:{}", config.host, config.port);

    Ok(ConnectionStatus {
        connected: true,
        server_info: Some(server_info),
        error: None,
    })
}

/// 建立 SSH 连接
pub fn connect(config: &SshConfig) -> Result<String> {
    let tcp = TcpStream::connect((config.host.as_str(), config.port))
        .map_err(|e| AppError::Ssh(format!("Connection failed: {}", e)))?;

    let mut sess = ssh2::Session::new()
        .map_err(|e| AppError::Ssh(format!("Session creation failed: {}", e)))?;

    sess.set_tcp_stream(tcp);
    sess.handshake()
        .map_err(|e| AppError::Ssh(format!("Handshake failed: {}", e)))?;

    authenticate(&sess, &config.username, &config.auth_method)?;

    let session_id = generate_session_id();

    let mut sessions = SSH_SESSIONS.lock()
        .map_err(|e| AppError::Ssh(format!("Lock error: {}", e)))?;
    sessions.insert(session_id.clone(), sess);

    Ok(session_id)
}

/// 断开 SSH 连接
pub fn disconnect(session_id: &str) -> Result<()> {
    let mut sessions = SSH_SESSIONS.lock()
        .map_err(|e| AppError::Ssh(format!("Lock error: {}", e)))?;

    if let Some(sess) = sessions.remove(session_id) {
        let _ = sess.disconnect(None, "Closing", Some(""));
    }

    Ok(())
}

/// 执行远程命令
fn execute_remote_command(session: &ssh2::Session, command: &str) -> Result<String> {
    let mut channel = session.channel_session()
        .map_err(|e| AppError::Ssh(format!("Channel error: {}", e)))?;

    channel.exec(command)
        .map_err(|e| AppError::Ssh(format!("Command execution failed: {}", e)))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| AppError::Ssh(format!("Read output failed: {}", e)))?;

    channel.wait_close()
        .map_err(|e| AppError::Ssh(format!("Close channel failed: {}", e)))?;

    Ok(output)
}

/// 获取远程仓库的 Commits
pub fn get_remote_commits(session_id: &str, repo_path: &str, limit: usize) -> Result<Vec<CommitInfo>> {
    let sessions = SSH_SESSIONS.lock()
        .map_err(|e| AppError::Ssh(format!("Lock error: {}", e)))?;
    let session = sessions.get(session_id)
        .ok_or_else(|| AppError::SessionNotFound(session_id.to_string()))?;

    let command = format!(
        "cd {} && git log --pretty=format:'%H|%h|%s|%an|%ae|%at' -n {}",
        repo_path, limit
    );

    let output = execute_remote_command(session, &command)?;

    let commits: Vec<CommitInfo> = output.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(6, '|').collect();
            if parts.len() >= 6 {
                let timestamp = parts[5].parse().unwrap_or(0);
                let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
                    .unwrap_or_else(|| chrono::Utc::now());

                Some(CommitInfo {
                    hash: parts[0].to_string(),
                    short_hash: parts[1].to_string(),
                    message: parts[2].to_string(),
                    author: parts[3].to_string(),
                    email: parts[4].to_string(),
                    date: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                    timestamp,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(commits)
}

/// 获取远程差异
pub fn get_remote_diff(session_id: &str, repo_path: &str, from: &str, to: &str) -> Result<DiffResult> {
    let sessions = SSH_SESSIONS.lock()
        .map_err(|e| AppError::Ssh(format!("Lock error: {}", e)))?;
    let session = sessions.get(session_id)
        .ok_or_else(|| AppError::SessionNotFound(session_id.to_string()))?;

    // 获取变更文件列表
    let command = format!(
        "cd {} && git diff --name-status {} {}",
        repo_path, from, to
    );

    let output = execute_remote_command(session, &command)?;

    let mut changes = Vec::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let change_type = match parts[0] {
                "A" => FileChangeType::Added,
                "D" => FileChangeType::Deleted,
                "M" => FileChangeType::Modified,
                "R" => FileChangeType::Renamed,
                "C" => FileChangeType::Copied,
                _ => FileChangeType::Modified,
            };

            let (path, old_path) = if change_type == FileChangeType::Renamed && parts.len() >= 3 {
                (parts[2].to_string(), Some(parts[1].to_string()))
            } else {
                (parts[1].to_string(), None)
            };

            changes.push(FileChange {
                path,
                change_type,
                old_path,
            });
        }
    }

    // 获取统计信息
    let stat_command = format!(
        "cd {} && git diff --shortstat {} {}",
        repo_path, from, to
    );

    let stat_output = execute_remote_command(session, &stat_command)?;
    let (total_additions, total_deletions) = parse_shortstat(&stat_output);
    let total_files = changes.len();

    Ok(DiffResult {
        commits: vec![],
        changes,
        total_files,
        total_additions,
        total_deletions,
    })
}

/// 解析 shortstat 输出
fn parse_shortstat(output: &str) -> (usize, usize) {
    let mut additions = 0;
    let mut deletions = 0;

    // 格式: "X files changed, Y insertions(+), Z deletions(-)"
    for part in output.split(',') {
        let part = part.trim();
        if part.contains("insertion") {
            additions = part.split_whitespace()
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        } else if part.contains("deletion") {
            deletions = part.split_whitespace()
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        }
    }

    (additions, deletions)
}
