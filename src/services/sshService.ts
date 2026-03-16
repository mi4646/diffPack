import { invoke } from "@tauri-apps/api/core";
import type { SshConfig, SshConfigEntry, ConnectionStatus, CommitInfo, DiffResult, CommitQueryOptions } from "@/types";

export async function parseSshConfig(): Promise<SshConfigEntry[]> {
    return invoke("parse_ssh_config");
}

export async function testSshConnection(config: SshConfig): Promise<ConnectionStatus> {
    return invoke("test_ssh_connection", { config });
}

export async function connectSsh(config: SshConfig): Promise<string> {
    return invoke("connect_ssh", { config });
}

export async function disconnectSsh(sessionId: string): Promise<void> {
    return invoke("disconnect_ssh", { sessionId });
}

export async function getRemoteCommits(
    sessionId: string,
    repoPath: string,
    options?: CommitQueryOptions
): Promise<CommitInfo[]> {
    return invoke("get_remote_commits", { sessionId, repoPath, options: options || {} });
}

export async function getRemoteDiff(
    sessionId: string,
    repoPath: string,
    fromCommit: string,
    toCommit: string
): Promise<DiffResult> {
    return invoke("get_remote_diff", { sessionId, repoPath, fromCommit, toCommit });
}
