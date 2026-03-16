use crate::error::{AppError, Result};
use crate::models::{CommitInfo, CommitQueryOptions, RepoInfo};
use chrono::{DateTime, Utc};
use git2::{Repository, Sort};
use std::path::Path;

/// 打开 Git 仓库
pub fn open_repo(path: &str) -> Result<Repository> {
    let repo_path = Path::new(path);
    if !repo_path.exists() {
        return Err(AppError::RepoNotFound(path.to_string()));
    }

    Repository::open(repo_path).map_err(AppError::from)
}

/// 获取仓库信息
pub fn get_repo_info(repo: &Repository) -> Result<RepoInfo> {
    let path = repo.path().parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| repo.path().to_string_lossy().to_string());

    let name = Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let head = repo.head().ok();
    let current_branch = head
        .and_then(|h| h.shorthand().map(|s| s.to_string()))
        .unwrap_or_else(|| "HEAD".to_string());

    let mut branches = Vec::new();
    for branch in repo.branches(None)? {
        let (branch, _type) = branch?;
        if let Some(name) = branch.name()? {
            branches.push(name.to_string());
        }
    }

    let status = repo.statuses(None)?;
    let is_clean = status.is_empty();

    Ok(RepoInfo {
        path,
        name,
        current_branch,
        branches,
        is_clean,
    })
}

/// 获取 Commit 列表
pub fn get_commits(repo: &Repository, options: &CommitQueryOptions) -> Result<Vec<CommitInfo>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TIME)?;

    // 设置起始点
    if let Some(branch) = &options.branch {
        let ref_name = format!("refs/heads/{}", branch);
        let oid = repo.refname_to_id(&ref_name)?;
        revwalk.push(oid)?;
    } else {
        revwalk.push_head()?;
    }

    let limit = options.limit.unwrap_or(100);

    // 解析日期范围
    let start_timestamp = options.start_date.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", d))
            .ok()
            .map(|dt| dt.timestamp())
    });

    let end_timestamp = options.end_date.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(&format!("{}T23:59:59Z", d))
            .ok()
            .map(|dt| dt.timestamp())
    });

    let mut commits = Vec::new();
    for (i, oid_result) in revwalk.enumerate() {
        if i >= limit {
            break;
        }

        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

        let time = commit.time();
        let timestamp = time.seconds();

        // 日期过滤
        if let Some(start) = start_timestamp {
            if timestamp < start {
                continue;
            }
        }
        if let Some(end) = end_timestamp {
            if timestamp > end {
                continue;
            }
        }

        let datetime: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0)
            .unwrap_or_else(|| Utc::now());

        let hash = commit.id().to_string();
        let short_hash = hash.chars().take(7).collect();

        commits.push(CommitInfo {
            hash,
            short_hash,
            message: commit.message().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("unknown").to_string(),
            email: commit.author().email().unwrap_or("").to_string(),
            date: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            timestamp,
        });
    }

    Ok(commits)
}

/// 按日期范围获取 Commits
pub fn get_commits_by_date(
    repo: &Repository,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<CommitInfo>> {
    let options = CommitQueryOptions {
        branch: None,
        limit: Some(1000),
        start_date: Some(start_date.to_string()),
        end_date: Some(end_date.to_string()),
    };
    get_commits(repo, &options)
}
