import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { FileChange, PackOptions, PackResult, PackProgress } from "@/types";

export async function packLocalChanges(
    repoPath: string,
    changes: FileChange[],
    options: PackOptions,
    onProgress?: (progress: PackProgress) => void
): Promise<PackResult> {
    // 监听进度事件
    let unlisten: (() => void) | null = null;
    if (onProgress) {
        unlisten = await listen<PackProgress>("pack-progress", (event) => {
            onProgress(event.payload);
        });
    }

    try {
        const result = await invoke<PackResult>("pack_local_changes", {
            repoPath,
            changes,
            options,
        });
        return result;
    } finally {
        if (unlisten) {
            unlisten();
        }
    }
}

export async function packRemoteChanges(
    sessionId: string,
    repoPath: string,
    changes: FileChange[],
    options: PackOptions,
    onProgress?: (progress: PackProgress) => void
): Promise<PackResult> {
    let unlisten: (() => void) | null = null;
    if (onProgress) {
        unlisten = await listen<PackProgress>("pack-progress", (event) => {
            onProgress(event.payload);
        });
    }

    try {
        const result = await invoke<PackResult>("pack_remote_changes", {
            sessionId,
            repoPath,
            changes,
            options,
        });
        return result;
    } finally {
        if (unlisten) {
            unlisten();
        }
    }
}

export async function selectOutputDirectory(): Promise<string | null> {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({
        directory: true,
        multiple: false,
        title: "选择输出目录",
    });
    return typeof selected === "string" ? selected : null;
}

export async function openInExplorer(path: string): Promise<void> {
    return invoke("open_in_explorer", { path });
}
