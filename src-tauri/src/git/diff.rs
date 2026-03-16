use crate::error::{AppError, Result};
use crate::models::{CommitInfo, DiffResult, FileChange, FileChangeType};
use git2::{Delta, Repository};

/// 获取两个 Commit 之间的差异
pub fn get_diff(repo: &Repository, from_commit: &str, to_commit: &str) -> Result<DiffResult> {
    let from = repo.revparse_single(from_commit)?;
    let to = repo.revparse_single(to_commit)?;

    let from_tree = from.peel_to_tree()?;
    let to_tree = to.peel_to_tree()?;

    let diff = repo.diff_tree_to_tree(Some(&from_tree), Some(&to_tree), None)?;

    let mut changes = Vec::new();
    let mut total_additions = 0;
    let mut total_deletions = 0;

    diff.foreach(
        &mut |delta, _| {
            let change_type = match delta.status() {
                Delta::Added => FileChangeType::Added,
                Delta::Deleted => FileChangeType::Deleted,
                Delta::Modified => FileChangeType::Modified,
                Delta::Renamed => FileChangeType::Renamed,
                Delta::Copied => FileChangeType::Copied,
                _ => FileChangeType::Modified,
            };

            let path = delta.new_file().path()
                .map(|p| p.to_string_lossy().to_string())
                .or_else(|| delta.old_file().path().map(|p| p.to_string_lossy().to_string()))
                .unwrap_or_default();

            let old_path = if delta.status() == Delta::Renamed {
                delta.old_file().path().map(|p| p.to_string_lossy().to_string())
            } else {
                None
            };

            changes.push(FileChange {
                path,
                change_type,
                old_path,
            });

            true
        },
        None,
        None,
        None,
    )?;

    // 计算统计信息
    let stats = diff.stats()?;
    total_additions = stats.insertions();
    total_deletions = stats.deletions();

    // 获取涉及的 commits 信息
    let commits = get_commits_between(repo, from_commit, to_commit)?;

    let total_files = changes.len();

    Ok(DiffResult {
        commits,
        changes,
        total_files,
        total_additions,
        total_deletions,
    })
}

/// 获取两个 Commit 之间的所有 commits
fn get_commits_between(repo: &Repository, from: &str, to: &str) -> Result<Vec<CommitInfo>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let to_oid = repo.revparse_single(to)?.id();
    revwalk.push(to_oid)?;

    let from_oid = repo.revparse_single(from)?.id();

    let mut commits = Vec::new();
    for oid_result in revwalk {
        let oid = oid_result?;
        if oid == from_oid {
            break;
        }

        let commit = repo.find_commit(oid)?;
        let time = commit.time();
        let timestamp = time.seconds();
        let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
            .unwrap_or_else(|| chrono::Utc::now());

        let hash = commit.id().to_string();
        let short_hash: String = hash.chars().take(7).collect();

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

/// 获取变更文件的绝对路径列表
pub fn get_changed_file_paths(
    repo: &Repository,
    changes: &[FileChange],
) -> Result<Vec<std::path::PathBuf>> {
    let repo_path = repo.path().parent()
        .ok_or_else(|| AppError::InvalidPath("Invalid repo path".to_string()))?;

    let mut paths = Vec::new();
    for change in changes {
        if change.change_type != FileChangeType::Deleted {
            let full_path = repo_path.join(&change.path);
            if full_path.exists() {
                paths.push(full_path);
            }
        }
    }

    Ok(paths)
}
