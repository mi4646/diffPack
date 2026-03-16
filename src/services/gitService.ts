import { invoke } from "@tauri-apps/api/core";
import type { RepoInfo, CommitInfo, DiffResult, CommitQueryOptions } from "@/types";

export async function openLocalRepo(path: string): Promise<RepoInfo> {
    return invoke("open_local_repo", { path });
}

export async function getCommits(
    repoPath: string,
    options?: CommitQueryOptions
): Promise<CommitInfo[]> {
    return invoke("get_commits", { repoPath, options: options || {} });
}

export async function getDiffBetweenCommits(
    repoPath: string,
    fromCommit: string,
    toCommit: string
): Promise<DiffResult> {
    return invoke("get_diff_between_commits", { repoPath, fromCommit, toCommit });
}

export async function getCommitsByDateRange(
    repoPath: string,
    startDate: string,
    endDate: string
): Promise<CommitInfo[]> {
    return invoke("get_commits_by_date_range", { repoPath, startDate, endDate });
}
