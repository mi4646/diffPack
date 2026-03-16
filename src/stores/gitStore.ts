import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { CommitInfo, DiffResult, RepoInfo, FileChange, CommitQueryOptions } from "@/types";
import * as gitService from "@/services/gitService";

export const useGitStore = defineStore("git", () => {
    // 状态
    const repoPath = ref<string | null>(null);
    const repoInfo = ref<RepoInfo | null>(null);
    const commits = ref<CommitInfo[]>([]);
    const selectedCommits = ref<string[]>([]);
    const diffResult = ref<DiffResult | null>(null);
    const selectedFiles = ref<string[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);

    // 计算属性
    const hasRepo = computed(() => repoPath.value !== null);
    const hasDiff = computed(() => diffResult.value !== null);
    const selectedFromCommit = computed(() => selectedCommits.value[0] || null);
    const selectedToCommit = computed(() => selectedCommits.value[1] || null);

    // Actions
    async function openRepo(path: string) {
        isLoading.value = true;
        error.value = null;
        try {
            const info = await gitService.openLocalRepo(path);
            repoPath.value = path;
            repoInfo.value = info;
            commits.value = [];
            selectedCommits.value = [];
            diffResult.value = null;
            selectedFiles.value = [];
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e);
            throw e;
        } finally {
            isLoading.value = false;
        }
    }

    async function loadCommits(options?: CommitQueryOptions) {
        if (!repoPath.value) return;
        isLoading.value = true;
        error.value = null;
        try {
            const result = await gitService.getCommits(repoPath.value, options);
            commits.value = result;
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e);
            throw e;
        } finally {
            isLoading.value = false;
        }
    }

    function selectCommitRange(from: string, to: string) {
        selectedCommits.value = [from, to];
    }

    async function analyzeDiff() {
        if (!repoPath.value || selectedCommits.value.length < 2) return;
        isLoading.value = true;
        error.value = null;
        try {
            const result = await gitService.getDiffBetweenCommits(
                repoPath.value,
                selectedCommits.value[0],
                selectedCommits.value[1]
            );
            diffResult.value = result;
            // 默认选中所有文件
            selectedFiles.value = result.changes.map((c) => c.path);
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e);
            throw e;
        } finally {
            isLoading.value = false;
        }
    }

    function toggleFileSelection(path: string) {
        const index = selectedFiles.value.indexOf(path);
        if (index === -1) {
            selectedFiles.value.push(path);
        } else {
            selectedFiles.value.splice(index, 1);
        }
    }

    function selectAllFiles() {
        if (diffResult.value) {
            selectedFiles.value = diffResult.value.changes.map((c) => c.path);
        }
    }

    function deselectAllFiles() {
        selectedFiles.value = [];
    }

    function getSelectedChanges(): FileChange[] {
        if (!diffResult.value) return [];
        return diffResult.value.changes.filter((c) => selectedFiles.value.includes(c.path));
    }

    function reset() {
        repoPath.value = null;
        repoInfo.value = null;
        commits.value = [];
        selectedCommits.value = [];
        diffResult.value = null;
        selectedFiles.value = [];
        error.value = null;
    }

    return {
        // 状态
        repoPath,
        repoInfo,
        commits,
        selectedCommits,
        diffResult,
        selectedFiles,
        isLoading,
        error,
        // 计算属性
        hasRepo,
        hasDiff,
        selectedFromCommit,
        selectedToCommit,
        // Actions
        openRepo,
        loadCommits,
        selectCommitRange,
        analyzeDiff,
        toggleFileSelection,
        selectAllFiles,
        deselectAllFiles,
        getSelectedChanges,
        reset,
    };
});
