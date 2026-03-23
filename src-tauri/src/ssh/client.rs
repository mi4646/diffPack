use crate::error::{AppError, Result};
use crate::models::{CommitInfo, ConnectionStatus, DiffResult, FileChange, FileChangeType, SshConfig};
use crate::ssh::auth::authenticate;
use crate::ssh::session::{create_session, SshSession};
use shell_escape::escape;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref SSH_SESSIONS: Arc<Mutex<HashMap<String, SshSession>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// 生成会话 ID
fn generate_session_id() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let random_suffix: String = (0..8)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    format!("sess_{}_{}", chrono::Utc::now().timestamp_millis(), random_suffix)
}

/// 测试 SSH 连接
pub async fn test_connection(config: &SshConfig) -> Result<ConnectionStatus> {
    let mut sess = create_session(config).await?;
    authenticate(&mut sess.session, &config.username, &config.auth_method).await?;

    let server_info = format!("{}:{}", config.host, config.port);

    Ok(ConnectionStatus {
        connected: true,
        server_info: Some(server_info),
        error: None,
    })
}

/// 建立 SSH 连接
pub async fn connect(config: &SshConfig) -> Result<String> {
    let mut sess = create_session(config).await?;
    authenticate(&mut sess.session, &config.username, &config.auth_method).await?;

    let session_id = generate_session_id();

    let mut sessions = SSH_SESSIONS.lock().await;
    sessions.insert(session_id.clone(), sess);

    Ok(session_id)
}

/// 断开 SSH 连接
pub async fn disconnect(session_id: &str) -> Result<()> {
    let mut sessions = SSH_SESSIONS.lock().await;

    if let Some(mut sess) = sessions.remove(session_id) {
        // 关闭SFTP会话
        if let Some(sftp) = sess.sftp.take() {
            let _ = sftp.close().await;
        }
        // 关闭SSH会话
        use russh::Disconnect;
        let _ = sess.session.disconnect(Disconnect::ByApplication, "Closing", "en").await;
    }

    Ok(())
}

/// 执行远程命令
async fn execute_remote_command(session: &mut SshSession, command: &str) -> Result<String> {
    session.execute_command(command).await
}

/// 获取远程仓库的 Commits
pub async fn get_remote_commits(session_id: &str, repo_path: &str, limit: usize) -> Result<Vec<CommitInfo>> {
    let mut sessions = SSH_SESSIONS.lock().await;
    let session = sessions.get_mut(session_id)
        .ok_or_else(|| AppError::SessionNotFound(session_id.to_string()))?;

    // 转义 shell 元字符防止注入
    let repo_path_escaped = escape(repo_path.into());
    let command = format!(
        "cd {} && git log --pretty=format:'%H|%h|%s|%an|%ae|%at' -n {}",
        repo_path_escaped, limit
    );

    let output = execute_remote_command(session, &command).await?;

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
pub async fn get_remote_diff(session_id: &str, repo_path: &str, from: &str, to: &str) -> Result<DiffResult> {
    let mut sessions = SSH_SESSIONS.lock().await;
    let session = sessions.get_mut(session_id)
        .ok_or_else(|| AppError::SessionNotFound(session_id.to_string()))?;

    // 获取变更文件列表
    let repo_path_escaped = escape(repo_path.into());
    let from_escaped = escape(from.into());
    let to_escaped = escape(to.into());
    let command = format!(
        "cd {} && git diff --name-status {} {}",
        repo_path_escaped, from_escaped, to_escaped
    );

    let output = execute_remote_command(session, &command).await?;

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
    let repo_path_escaped = escape(repo_path.into());
    let from_escaped = escape(from.into());
    let to_escaped = escape(to.into());
    let stat_command = format!(
        "cd {} && git diff --shortstat {} {}",
        repo_path_escaped, from_escaped, to_escaped
    );

    let stat_output = execute_remote_command(session, &stat_command).await?;
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
