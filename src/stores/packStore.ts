import { defineStore } from "pinia";
import { ref } from "vue";
import type { PackOptions, PackProgress, PackResult, PackTask } from "@/types";
import * as packService from "@/services/packService";

export const usePackStore = defineStore("pack", () => {
    // 状态
    const currentTask = ref<PackTask | null>(null);
    const progress = ref<PackProgress | null>(null);
    const isPacking = ref(false);
    const options = ref<PackOptions>({
        format: "zip",
        outputPath: "",
        includeDeleted: false,
        preserveStructure: true,
    });
    const history = ref<PackResult[]>([]);

    // Actions
    async function startPack(
        source: "local" | "remote",
        changes: import("@/types").FileChange[],
        sessionId?: string,
        repoPath?: string
    ) {
        isPacking.value = true;
        progress.value = null;

        const taskId = Date.now().toString();
        currentTask.value = {
            id: taskId,
            source,
            changes,
            options: options.value,
            status: "running",
        };

        try {
            let result: PackResult;
            if (source === "local" && repoPath) {
                result = await packService.packLocalChanges(
                    repoPath,
                    changes,
                    options.value,
                    (p) => {
                        progress.value = p;
                        if (currentTask.value) {
                            currentTask.value.progress = p;
                        }
                    }
                );
            } else if (source === "remote" && sessionId && repoPath) {
                result = await packService.packRemoteChanges(
                    sessionId,
                    repoPath,
                    changes,
                    options.value,
                    (p) => {
                        progress.value = p;
                        if (currentTask.value) {
                            currentTask.value.progress = p;
                        }
                    }
                );
            } else {
                throw new Error("Invalid pack parameters");
            }

            if (currentTask.value) {
                currentTask.value.status = "completed";
                currentTask.value.result = result;
            }
            history.value.unshift(result);
            // 打包完成后稍延清除进度，让 100% 状态展示片刻再消失
            setTimeout(() => { progress.value = null; }, 600);
            return result;
        } catch (e) {
            if (currentTask.value) {
                currentTask.value.status = "failed";
                currentTask.value.result = {
                    success: false,
                    outputPath: "",
                    fileCount: 0,
                    totalSize: 0,
                    durationMs: 0,
                    error: e instanceof Error ? e.message : String(e),
                };
            }
            throw e;
        } finally {
            isPacking.value = false;
        }
    }

    function updateOptions(newOptions: Partial<PackOptions>) {
        options.value = { ...options.value, ...newOptions };
    }

    async function selectOutputDirectory() {
        const path = await packService.selectOutputDirectory();
        if (path) {
            options.value.outputPath = path;
        }
        return path;
    }

    function loadHistory() {
        const stored = localStorage.getItem("packHistory");
        if (stored) {
            try {
                history.value = JSON.parse(stored);
            } catch {
                history.value = [];
            }
        }
    }

    function saveHistory() {
        localStorage.setItem("packHistory", JSON.stringify(history.value.slice(0, 50)));
    }

    function clearHistory() {
        history.value = [];
        localStorage.removeItem("packHistory");
    }

    function reset() {
        currentTask.value = null;
        progress.value = null;
        isPacking.value = false;
    }

    return {
        // 状态
        currentTask,
        progress,
        isPacking,
        options,
        history,
        // Actions
        startPack,
        updateOptions,
        selectOutputDirectory,
        loadHistory,
        saveHistory,
        clearHistory,
        reset,
    };
});
