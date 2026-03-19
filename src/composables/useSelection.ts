import { ref, computed, watch } from "vue";
import type { CommitInfo, FileChange } from "@/types";

export type SelectionMode = "range" | "list" | "date";

export function useSelection(
  commits: () => CommitInfo[],
  selectedCommits: () => string[],
  setSelectedCommits: (hashes: string[]) => void
) {
  // 状态
  const selectionMode = ref<SelectionMode>("range");

  // 计算属性
  const hasRange = computed(() => selectedCommits().length === 2);
  const canAnalyze = computed(() => {
    const selected = selectedCommits();
    return selected.length === 2 || (selectionMode.value === "list" && selected.length > 0);
  });

  // 选择 commit
  function selectCommit(commit: CommitInfo) {
    const current = [...selectedCommits()];

    if (selectionMode.value === "range" || selectionMode.value === "date") {
      // 范围模式：依次选两个端点
      if (current.length === 0) {
        setSelectedCommits([commit.hash]);
      } else if (current.length === 1) {
        if (current[0] === commit.hash) {
          // 点击同一个 commit 取消选择
          setSelectedCommits([]);
        } else {
          setSelectedCommits([current[0], commit.hash]);
        }
      } else {
        // 已选两个，重新开始
        setSelectedCommits([commit.hash]);
      }
    } else if (selectionMode.value === "list") {
      // 列表模式：多选切换
      const idx = current.indexOf(commit.hash);
      if (idx === -1) {
        setSelectedCommits([...current, commit.hash]);
      } else {
        setSelectedCommits(current.filter(h => h !== commit.hash));
      }
    }
  }

  // 切换文件选中
  function toggleFileSelection(
    path: string,
    selectedFiles: () => string[],
    setSelectedFiles: (paths: string[]) => void
  ) {
    const current = selectedFiles();
    const idx = current.indexOf(path);
    if (idx === -1) {
      setSelectedFiles([...current, path]);
    } else {
      setSelectedFiles(current.filter(p => p !== path));
    }
  }

  // 全选文件
  function selectAllFiles(
    files: FileChange[],
    setSelectedFiles: (paths: string[]) => void
  ) {
    setSelectedFiles(files.map(f => f.path));
  }

  // 取消全选文件
  function deselectAllFiles(setSelectedFiles: (paths: string[]) => void) {
    setSelectedFiles([]);
  }

  // 切换选择模式时清空选择
  watch(selectionMode, () => {
    setSelectedCommits([]);
  });

  // 重置选择
  function resetSelection() {
    setSelectedCommits([]);
  }

  return {
    // 状态
    selectionMode,
    // 计算属性
    hasRange,
    canAnalyze,
    // 方法
    selectCommit,
    toggleFileSelection,
    selectAllFiles,
    deselectAllFiles,
    resetSelection,
  };
}

// 文件选择 composable - 独立版本
export function useFileSelection() {
  const selectedFiles = ref<string[]>([]);

  function toggleFile(path: string) {
    const idx = selectedFiles.value.indexOf(path);
    if (idx === -1) {
      selectedFiles.value = [...selectedFiles.value, path];
    } else {
      selectedFiles.value = selectedFiles.value.filter(p => p !== path);
    }
  }

  function selectAll(files: FileChange[]) {
    selectedFiles.value = files.map(f => f.path);
  }

  function deselectAll() {
    selectedFiles.value = [];
  }

  function isSelected(path: string) {
    return selectedFiles.value.includes(path);
  }

  return {
    selectedFiles,
    toggleFile,
    selectAll,
    deselectAll,
    isSelected,
  };
}
