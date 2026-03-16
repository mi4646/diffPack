// 打包相关类型定义

export type PackFormat = "zip" | "tarGz";

export interface PackOptions {
    format: PackFormat;
    outputPath: string;
    includeDeleted: boolean;
    preserveStructure: boolean;
    baseDir?: string;
}

export interface PackProgress {
    currentFile: string;
    processed: number;
    total: number;
    percentage: number;
}

export interface PackResult {
    success: boolean;
    outputPath: string;
    fileCount: number;
    totalSize: number;
    durationMs: number;
    error?: string;
}

export interface PackTask {
    id: string;
    source: "local" | "remote";
    changes: import("./git").FileChange[];
    options: PackOptions;
    status: "pending" | "running" | "completed" | "failed";
    progress?: PackProgress;
    result?: PackResult;
}
