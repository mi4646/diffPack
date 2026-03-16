// Git 相关类型定义

export interface CommitInfo {
    hash: string;
    shortHash: string;
    message: string;
    author: string;
    email: string;
    date: string;
    timestamp: number;
}

export type FileChangeType = "added" | "modified" | "deleted" | "renamed" | "copied";

export interface FileChange {
    path: string;
    changeType: FileChangeType;
    oldPath?: string;
}

export interface DiffResult {
    commits: CommitInfo[];
    changes: FileChange[];
    totalFiles: number;
    totalAdditions: number;
    totalDeletions: number;
}

export interface RepoInfo {
    path: string;
    name: string;
    currentBranch: string;
    branches: string[];
    isClean: boolean;
}

export interface CommitQueryOptions {
    branch?: string;
    limit?: number;
    startDate?: string;
    endDate?: string;
}
