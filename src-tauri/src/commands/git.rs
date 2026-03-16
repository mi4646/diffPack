use crate::git::{get_commits as git_get_commits, get_commits_by_date, get_diff, get_repo_info, open_repo};
use crate::models::{CommitQueryOptions, DiffResult, RepoInfo};
use tauri::command;

/// 打开本地 Git 仓库
#[command]
pub async fn open_local_repo(path: String) -> Result<RepoInfo, String> {
    let repo = open_repo(&path).map_err(|e| e.to_string())?;
    let info = get_repo_info(&repo).map_err(|e| e.to_string())?;
    Ok(info)
}

/// 获取 Commit 列表
#[command]
pub async fn get_commits(
    repo_path: String,
    options: CommitQueryOptions,
) -> Result<Vec<crate::models::CommitInfo>, String> {
    let repo = open_repo(&repo_path).map_err(|e| e.to_string())?;
    let commits = git_get_commits(&repo, &options).map_err(|e| e.to_string())?;
    Ok(commits)
}

/// 获取两个 Commit 之间的差异
#[command]
pub async fn get_diff_between_commits(
    repo_path: String,
    from_commit: String,
    to_commit: String,
) -> Result<DiffResult, String> {
    let repo = open_repo(&repo_path).map_err(|e| e.to_string())?;
    let diff = get_diff(&repo, &from_commit, &to_commit).map_err(|e| e.to_string())?;
    Ok(diff)
}

/// 按日期范围获取 Commits
#[command]
pub async fn get_commits_by_date_range(
    repo_path: String,
    start_date: String,
    end_date: String,
) -> Result<Vec<crate::models::CommitInfo>, String> {
    let repo = open_repo(&repo_path).map_err(|e| e.to_string())?;
    let commits = get_commits_by_date(&repo, &start_date, &end_date).map_err(|e| e.to_string())?;
    Ok(commits)
}
