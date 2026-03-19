import { ref, computed } from "vue";
import { useGitStore } from "@/stores/gitStore";
import { useNotifier } from "./useNotifier";
import { useLoading } from "./useLoading";

export interface LoadCommitsOptions {
  startDate?: string;
  endDate?: string;
  limit?: number;
}

export function useRepository() {
  const gitStore = useGitStore();
  const { error } = useNotifier();
  const { startLoading, stopLoading, setLoadingText } = useLoading();

  // 状态
  const repoPath = ref(gitStore.repoPath || "");

  // 计算属性
  const hasRepo = computed(() => gitStore.hasRepo);
  const repoInfo = computed(() => gitStore.repoInfo);
  const commits = computed(() => gitStore.commits);
  const selectedCommits = computed(() => gitStore.selectedCommits);
  const diffResult = computed(() => gitStore.diffResult);

  // 打开仓库
  async function openRepo(path: string): Promise<boolean> {
    if (!path) return false;

    startLoading("正在打开仓库...");
    try {
      await gitStore.openRepo(path);
      repoPath.value = path;
      return true;
    } catch (e) {
      const errStr = e instanceof Error ? e.message : String(e);
      handleOpenError(errStr, path);
      return false;
    } finally {
      stopLoading();
    }
  }

  // 处理打开仓库错误
  function handleOpenError(errStr: string, path: string) {
    if (
      errStr.includes("code=NotFound") ||
      errStr.includes("class=Repository") ||
      errStr.includes("could not find repository")
    ) {
      error(
        "未找到 Git 仓库",
        `所选路径不是有效的 Git 仓库，请确认该目录已执行过 git init 或是从远程克隆的仓库。\n\n路径：${path}`
      );
    } else {
      error("打开仓库失败", errStr);
    }
  }

  // 加载提交
  async function loadCommits(options: LoadCommitsOptions = {}): Promise<void> {
    if (!hasRepo.value) return;

    startLoading("正在加载提交...");
    try {
      await gitStore.loadCommits(options);
    } catch (e) {
      const errStr = e instanceof Error ? e.message : String(e);
      error("加载提交失败", errStr);
      throw e;
    } finally {
      stopLoading();
    }
  }

  // 分析差异
  async function analyzeDiff(): Promise<void> {
    if (!hasRepo.value || selectedCommits.value.length === 0) return;

    startLoading("正在分析差异...");
    try {
      await gitStore.analyzeDiff();
    } catch (e) {
      const errStr = e instanceof Error ? e.message : String(e);
      error("分析差异失败", errStr);
      throw e;
    } finally {
      stopLoading();
    }
  }

  // 获取选中的文件变更
  function getSelectedChanges() {
    return gitStore.getSelectedChanges();
  }

  // 重置状态
  function reset() {
    repoPath.value = "";
    gitStore.reset();
  }

  return {
    // 状态
    repoPath,
    // 计算属性
    hasRepo,
    repoInfo,
    commits,
    selectedCommits,
    diffResult,
    // 方法
    openRepo,
    loadCommits,
    analyzeDiff,
    getSelectedChanges,
    reset,
    // 加载状态
    startLoading,
    stopLoading,
    setLoadingText,
  };
}
