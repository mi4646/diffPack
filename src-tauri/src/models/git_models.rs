use serde::{Deserialize, Serialize};

/// Commit 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub timestamp: i64,
}

/// 文件变更类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
}

/// 文件变更信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChange {
    pub path: String,
    pub change_type: FileChangeType,
    pub old_path: Option<String>,
}

/// 差异结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffResult {
    pub commits: Vec<CommitInfo>,
    pub changes: Vec<FileChange>,
    pub total_files: usize,
    pub total_additions: usize,
    pub total_deletions: usize,
}

/// 仓库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoInfo {
    pub path: String,
    pub name: String,
    pub current_branch: String,
    pub branches: Vec<String>,
    pub is_clean: bool,
}

/// Commit 查询选项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitQueryOptions {
    pub branch: Option<String>,
    pub limit: Option<usize>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
