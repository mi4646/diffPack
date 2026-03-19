<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useGitStore, usePackStore, useAppStore } from "@/stores";
import { useRepository, useSelection, useNotifier } from "@/composables";
import { openInExplorer } from "@/services/packService";
import PackOptions from "@/components/pack/PackOptions.vue";
import PackProgress from "@/components/pack/PackProgress.vue";
import PackResult from "@/components/pack/PackResult.vue";
import SearchInput from "@/components/common/SearchInput.vue";
import CommitList from "@/components/common/CommitList.vue";
import FileList from "@/components/common/FileList.vue";
import AppDialog from "@/components/common/AppDialog.vue";
import type { CommitInfo, FileChange } from "@/types";

const gitStore = useGitStore();
const packStore = usePackStore();
const appStore = useAppStore();
const { error } = useNotifier();

// 使用 composables
const { repoPath, hasRepo, repoInfo, commits, selectedCommits, diffResult, openRepo: openRepoFn, loadCommits, analyzeDiff: analyzeDiffFn, reset: resetRepo } = useRepository();

const { selectionMode, selectCommit: selectCommitFn } = useSelection(
  () => gitStore.commits,
  () => gitStore.selectedCommits,
  (hashes) => { gitStore.selectedCommits = hashes; }
);

const repoPathInput = ref("");
const startDate = ref("");
const endDate = ref("");
const commitSearchKeyword = ref("");

// Commit 详情弹框
const showDetailDialog = ref(false);
const detailCommit = ref<CommitInfo | null>(null);

function showCommitDetail(commit: CommitInfo) {
  detailCommit.value = commit;
  showDetailDialog.value = true;
}

function formatDetailMessage(commit: CommitInfo): string {
  return [
    `Hash：${commit.hash}`,
    `作者：${commit.author} <${commit.email}>`,
    `时间：${commit.date}`,
    ``,
    `提交说明：`,
    commit.message,
  ].join("\n");
}

// 日期范围快捷选项
const datePresets = [
  { label: "今天", days: 0 },
  { label: "最近7天", days: 7 },
  { label: "最近30天", days: 30 },
  { label: "最近90天", days: 90 },
];

async function browseRepo() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "选择 Git 仓库目录",
  });
  if (selected && typeof selected === "string") {
    repoPathInput.value = selected;
    await openRepo();
  }
}

async function openRepo() {
  if (!repoPathInput.value) return;
  try {
    await gitStore.openRepo(repoPathInput.value);
    await loadCommits();
  } catch (e) {
    const errStr = e instanceof Error ? e.message : String(e);
    console.error("Failed to open repo:", errStr);
    if (
      errStr.includes("code=NotFound") ||
      errStr.includes("class=Repository") ||
      errStr.includes("could not find repository")
    ) {
      error(
        "未找到 Git 仓库",
        `所选路径不是有效的 Git 仓库，请确认该目录已执行过 git init 或是从远程克隆的仓库。\n\n路径：${repoPathInput.value}`
      );
    } else {
      error("打开仓库失败", errStr);
    }
  }
}

function setDatePreset(days: number) {
  const now = new Date();
  endDate.value = now.toISOString().split("T")[0];
  const start = new Date(now.getTime() - days * 24 * 60 * 60 * 1000);
  startDate.value = start.toISOString().split("T")[0];
}

function selectCommit(commit: CommitInfo, _index: number) {
  if (selectionMode.value === "range" || selectionMode.value === "date") {
    // 范围/时间范围模式：依次选两个端点
    if (gitStore.selectedCommits.length === 0) {
      gitStore.selectedCommits = [commit.hash];
    } else if (gitStore.selectedCommits.length === 1) {
      if (gitStore.selectedCommits[0] === commit.hash) {
        // 点击同一个 commit 取消选择
        gitStore.selectedCommits = [];
      } else {
        gitStore.selectedCommits = [gitStore.selectedCommits[0], commit.hash];
      }
    } else {
      // 已选两个，重新开始
      gitStore.selectedCommits = [commit.hash];
    }
  } else if (selectionMode.value === "list") {
    // 列表模式：多选切换
    const idx = gitStore.selectedCommits.indexOf(commit.hash);
    if (idx === -1) {
      gitStore.selectedCommits = [...gitStore.selectedCommits, commit.hash];
    } else {
      gitStore.selectedCommits = gitStore.selectedCommits.filter(h => h !== commit.hash);
    }
  }
}

// 根据搜索关键词过滤后的 commit 列表
const filteredCommits = computed(() => {
  const kw = commitSearchKeyword.value.trim().toLowerCase();
  if (!kw) return gitStore.commits;
  return gitStore.commits.filter((c) =>
    c.message.toLowerCase().includes(kw) ||
    c.author.toLowerCase().includes(kw) ||
    c.hash.toLowerCase().includes(kw) ||
    c.shortHash.toLowerCase().includes(kw)
  );
});

async function analyzeDiff() {
  try {
    await gitStore.analyzeDiff();
  } catch (e) {
    const errStr = e instanceof Error ? e.message : String(e);
    error("分析差异失败", errStr);
  }
}

async function startPack() {
  const changes = gitStore.getSelectedChanges();
  if (changes.length === 0 || !gitStore.repoPath) return;

  try {
    await packStore.startPack("local", changes, undefined, gitStore.repoPath);
  } catch (e) {
    console.error("Pack failed:", e);
  }
}

async function handlePathClick(path: string) {
  try {
    await openInExplorer(path);
  } catch (e) {
    console.error("Failed to open path:", e);
  }
}

async function handleFilePathClick(filePath: string) {
  if (!gitStore.repoPath) return;
  const fullPath = `${gitStore.repoPath}\\${filePath}`;
  // 获取目录路径
  const lastSep = Math.max(fullPath.lastIndexOf("/"), fullPath.lastIndexOf("\\"));
  const dirPath = lastSep > 0 ? fullPath.substring(0, lastSep) : fullPath;
  try {
    await openInExplorer(dirPath);
  } catch (e) {
    console.error("Failed to open path:", e);
  }
}

watch([startDate, endDate], async () => {
  if (selectionMode.value === "date" && gitStore.hasRepo) {
    try {
      await loadCommits();
    } catch (e) {
      const errStr = e instanceof Error ? e.message : String(e);
      error("加载提交失败", errStr);
    }
  }
});

watch(selectionMode, async () => {
  gitStore.selectedCommits = [];
  if (selectionMode.value === "date" && gitStore.hasRepo) {
    try {
      await loadCommits();
    } catch (e) {
      const errStr = e instanceof Error ? e.message : String(e);
      error("加载提交失败", errStr);
    }
  }
});

// 仓库路径被清空时重置所有状态
watch(repoPathInput, (val) => {
  if (!val) {
    gitStore.reset();
    commitSearchKeyword.value = "";
    selectionMode.value = "range";
    startDate.value = "";
    endDate.value = "";
  }
});
</script>

<template>
  <div class="page-container">
    <!-- 仓库选择 -->
    <section class="card">
      <div class="card-header">
        <h2 class="card-title">选择 Git 仓库</h2>
      </div>
      <div class="repo-input-row">
        <input
          v-model="repoPathInput"
          type="text"
          placeholder="输入仓库路径或点击浏览..."
          class="input-field"
          @keyup.enter="openRepo"
          @dblclick="handlePathClick(repoPathInput)"
        />
        <button class="btn-secondary" @click="browseRepo">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
          </svg>
          浏览
        </button>
        <button class="btn-primary" @click="openRepo" :disabled="!repoPathInput">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
            <polyline points="22 4 12 14.01 9 11.01"/>
          </svg>
          打开
        </button>
      </div>
      <div v-if="gitStore.repoInfo" class="repo-info">
        <span class="repo-name">
          <svg class="info-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
          </svg>
          {{ gitStore.repoInfo.name }}
        </span>
        <span class="badge badge-primary">
          <svg class="badge-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="6" y1="3" x2="6" y2="15"/>
            <circle cx="18" cy="6" r="3"/>
            <circle cx="6" cy="18" r="3"/>
            <path d="M18 9a9 9 0 0 1-9 9"/>
          </svg>
          {{ gitStore.repoInfo.currentBranch }}
        </span>
      </div>
    </section>

    <!-- Commit 选择 -->
    <section v-if="gitStore.hasRepo" class="card">
      <div class="card-header">
        <h2 class="card-title">选择 Commit</h2>
      </div>

      <!-- 选择模式 -->
      <div class="selection-mode">
        <label class="radio-label">
          <input type="radio" v-model="selectionMode" value="range" class="radio-input" />
          <span class="radio-control"></span>
          <span class="radio-text">范围选择</span>
        </label>
        <label class="radio-label">
          <input type="radio" v-model="selectionMode" value="list" class="radio-input" />
          <span class="radio-control"></span>
          <span class="radio-text">列表选择</span>
        </label>
        <label class="radio-label">
          <input type="radio" v-model="selectionMode" value="date" class="radio-input" />
          <span class="radio-control"></span>
          <span class="radio-text">时间范围</span>
        </label>
      </div>

      <!-- 时间范围选择 -->
      <div v-if="selectionMode === 'date'" class="date-range">
        <div class="date-presets">
          <button
            v-for="preset in datePresets"
            :key="preset.days"
            class="btn-ghost btn-sm"
            @click="setDatePreset(preset.days)"
          >
            {{ preset.label }}
          </button>
        </div>
        <div class="date-inputs">
          <label class="form-label">
            开始日期
            <input type="date" v-model="startDate" class="input-field input-sm" />
          </label>
          <label class="form-label">
            结束日期
            <input type="date" v-model="endDate" class="input-field input-sm" />
          </label>
        </div>
      </div>

      <!-- commit 搜索框 -->
      <div class="search-row">
        <div class="search-input-wrapper">
          <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
          </svg>
          <input
            v-model="commitSearchKeyword"
            type="text"
            class="search-input"
            placeholder="搜索 commit message、作者、hash..."
          />
          <button
            v-if="commitSearchKeyword"
            class="search-clear"
            @click="commitSearchKeyword = ''"
            aria-label="清除搜索"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
        <span class="commit-count">{{ filteredCommits.length }} / {{ gitStore.commits.length }} 个</span>
      </div>

      <!-- 操作提示 -->
      <div class="mode-hint">
        <svg class="hint-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <line x1="12" y1="16" x2="12" y2="12"/>
          <line x1="12" y1="8" x2="12.01" y2="8"/>
        </svg>
        <span v-if="selectionMode === 'range' || selectionMode === 'date'">
          点击选择起始 commit，再次点击选择终止 commit
        </span>
        <span v-else-if="selectionMode === 'list'">
          点击可多选，将对第一个和第二个选中的 commit 进行差异分析
        </span>
      </div>

      <!-- Commit 列表 -->
      <div class="commit-list">
        <div
          v-for="(commit, index) in filteredCommits"
          :key="commit.hash"
          class="commit-item"
          :class="{
            selected: gitStore.selectedCommits.includes(commit.hash),
            'selected-first': gitStore.selectedCommits[0] === commit.hash,
            'selected-second': gitStore.selectedCommits[1] === commit.hash,
          }"
          @click="selectCommit(commit, index)"
        >
          <!-- 第一行：hash + 作者 + 时间 -->
          <div class="commit-header">
            <div class="commit-meta">
              <span class="hash-badge" :title="commit.hash">
                {{ appStore.showFullCommitId ? commit.hash : commit.shortHash }}
              </span>
              <span class="commit-author">{{ commit.author }}</span>
              <span class="commit-date">{{ commit.date }}</span>
            </div>
            <button
              class="btn-ghost btn-sm"
              title="查看详情"
              @click.stop="showCommitDetail(commit)"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="16" x2="12" y2="12"/>
                <line x1="12" y1="8" x2="12.01" y2="8"/>
              </svg>
              详情
            </button>
          </div>
          <!-- 第二行：commit 描述 -->
          <div class="commit-body">
            <span class="commit-message">{{ commit.message }}</span>
          </div>
        </div>
      </div>

      <!-- 已选择信息 -->
      <div v-if="gitStore.selectedCommits.length > 0" class="selected-info">
        <span class="selected-count">已选择 {{ gitStore.selectedCommits.length }} 个 commit</span>
        <button
          class="btn-primary"
          @click="analyzeDiff"
          :disabled="gitStore.selectedCommits.length < 2 || gitStore.isLoading"
        >
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
          分析差异
        </button>
      </div>
    </section>

    <!-- 差异预览 -->
    <section v-if="gitStore.hasDiff" class="card">
      <div class="card-header">
        <h2 class="card-title">差异预览</h2>
        <div class="diff-stats">
          <span class="stat-item">
            <span class="stat-value">{{ gitStore.diffResult?.totalFiles }}</span>
            <span class="stat-label">个文件变更</span>
          </span>
          <span class="stat-item text-success">
            <span class="stat-value">+{{ gitStore.diffResult?.totalAdditions }}</span>
          </span>
          <span class="stat-item text-error">
            <span class="stat-value">-{{ gitStore.diffResult?.totalDeletions }}</span>
          </span>
        </div>
      </div>

      <div class="file-list">
        <div
          v-for="change in gitStore.diffResult?.changes"
          :key="change.path"
          class="file-item"
          :class="{
            selected: gitStore.selectedFiles.includes(change.path),
          }"
          @click="gitStore.toggleFileSelection(change.path)"
        >
          <input
            type="checkbox"
            :checked="gitStore.selectedFiles.includes(change.path)"
            @click.stop
            class="checkbox"
          />
          <span
            class="change-badge"
            :class="{
              'change-added': change.changeType === 'added',
              'change-modified': change.changeType === 'modified',
              'change-deleted': change.changeType === 'deleted',
              'change-renamed': change.changeType === 'renamed',
            }"
          >
            {{ change.changeType }}
          </span>
          <span
            class="file-path"
            :title="'双击打开所在目录'"
            @dblclick.stop="handleFilePathClick(change.path)"
          >{{ change.path }}</span>
          <span v-if="change.oldPath" class="old-path">← {{ change.oldPath }}</span>
        </div>
      </div>

      <div class="file-actions">
        <button class="btn-secondary btn-sm" @click="gitStore.selectAllFiles">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
          全选
        </button>
        <button class="btn-secondary btn-sm" @click="gitStore.deselectAllFiles">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
          </svg>
          取消全选
        </button>
      </div>
    </section>

    <!-- 打包选项 -->
    <section v-if="gitStore.hasDiff" class="card">
      <div class="card-header">
        <h2 class="card-title">打包选项</h2>
      </div>
      <PackOptions @pack="startPack" />

      <PackProgress v-if="packStore.progress" :progress="packStore.progress" />

      <PackResult v-if="packStore.currentTask?.result" :result="packStore.currentTask.result" />
    </section>
  </div>

  <!-- commit 详情弹框 -->
  <AppDialog
    v-if="detailCommit"
    :visible="showDetailDialog"
    title="Commit 详情"
    :message="formatDetailMessage(detailCommit)"
    type="info"
    @close="showDetailDialog = false"
  />
</template>

<style scoped>
.page-container {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  max-width: var(--max-content-width);
  margin: 0 auto;
}

/* ========== Repo Section ========== */
.repo-input-row {
  display: flex;
  gap: var(--space-2);
}

.repo-input-row .input-field {
  flex: 1;
}

.repo-info {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-top: var(--space-3);
  padding-top: var(--space-3);
  border-top: 1px solid var(--color-border-light);
}

.repo-name {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
}

.info-icon {
  width: 16px;
  height: 16px;
  color: var(--color-text-muted);
}

.badge-icon {
  width: 12px;
  height: 12px;
  margin-right: 4px;
}

/* ========== Selection Mode ========== */
.selection-mode {
  display: flex;
  gap: var(--space-4);
  margin-bottom: var(--space-4);
}

.radio-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  cursor: pointer;
  font-size: var(--font-size-sm);
}

.radio-input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.radio-control {
  width: 16px;
  height: 16px;
  border: 2px solid var(--color-border);
  border-radius: 50%;
  position: relative;
  transition: all var(--transition-normal);
}

.radio-input:checked + .radio-control {
  border-color: var(--color-primary);
}

.radio-input:checked + .radio-control::after {
  content: '';
  position: absolute;
  top: 3px;
  left: 3px;
  width: 6px;
  height: 6px;
  background: var(--color-primary);
  border-radius: 50%;
}

.radio-text {
  color: var(--color-text-secondary);
}

/* ========== Date Range ========== */
.date-range {
  margin-bottom: var(--space-4);
}

.date-presets {
  display: flex;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.date-inputs {
  display: flex;
  gap: var(--space-4);
}

.date-inputs .form-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

/* ========== Search ========== */
.search-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-3);
}

.search-input-wrapper {
  position: relative;
  flex: 1;
}

.search-icon {
  position: absolute;
  left: var(--space-3);
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 16px;
  color: var(--color-text-muted);
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding-left: 40px;
  padding-right: 32px;
}

.search-clear {
  position: absolute;
  right: var(--space-2);
  top: 50%;
  transform: translateY(-50%);
  width: 20px;
  height: 20px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text-muted);
  border-radius: var(--radius-sm);
}

.search-clear:hover {
  color: var(--color-error);
  background: var(--color-error-bg);
}

.search-clear svg {
  width: 14px;
  height: 14px;
}

.commit-count {
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
  white-space: nowrap;
  min-width: 80px;
  text-align: right;
}

/* ========== Mode Hint ========== */
.mode-hint {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
  margin-bottom: var(--space-3);
  padding: var(--space-2) var(--space-3);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-md);
}

.hint-icon {
  width: 14px;
  height: 14px;
  flex-shrink: 0;
}

/* ========== Commit List ========== */
.commit-list {
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-lg);
}

.commit-item {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  cursor: pointer;
  transition: background-color var(--transition-fast);
}

.commit-item:last-child {
  border-bottom: none;
}

.commit-item:hover {
  background: var(--color-bg-hover);
}

.commit-item.selected {
  background: var(--color-primary-subtle);
}

.commit-item.selected-first {
  background: #d4e4f7;
}

.commit-item.selected-second {
  background: #c0d8ef;
}

/* 第一行：hash + 作者 + 日期 */
.commit-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
}

.commit-meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-wrap: wrap;
  min-width: 0;
}

.commit-author {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
  white-space: nowrap;
}

.commit-date {
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
  white-space: nowrap;
}

/* 第二行：commit 说明 */
.commit-body {
  padding-left: 2px;
}

.commit-message {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
}

/* ========== Selected Info ========== */
.selected-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: var(--space-4);
  padding-top: var(--space-4);
  border-top: 1px solid var(--color-border-light);
}

.selected-count {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
}

/* ========== Diff Stats ========== */
.diff-stats {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.stat-item {
  display: flex;
  align-items: baseline;
  gap: var(--space-1);
}

.stat-value {
  font-weight: var(--font-weight-semibold);
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
}

.stat-label {
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
}

/* ========== File List ========== */
.file-list {
  max-height: 300px;
  overflow-y: auto;
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-lg);
  margin-bottom: var(--space-3);
}

.file-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border-bottom: 1px solid var(--color-border-light);
  cursor: pointer;
}

.file-item:last-child {
  border-bottom: none;
}

.file-item:hover {
  background: var(--color-bg-hover);
}

.file-item.selected {
  background: var(--color-success-bg);
}

.checkbox {
  width: 16px;
  height: 16px;
  cursor: pointer;
  flex-shrink: 0;
}

.change-badge {
  font-size: var(--font-size-xs);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  text-transform: uppercase;
  font-weight: var(--font-weight-medium);
  flex-shrink: 0;
}

.file-path {
  flex: 1;
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
  cursor: pointer;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-path:hover {
  color: var(--color-primary);
  text-decoration: underline;
}

.old-path {
  color: var(--color-text-muted);
  font-size: var(--font-size-xs);
  flex-shrink: 0;
}

/* ========== File Actions ========== */
.file-actions {
  display: flex;
  gap: var(--space-2);
}

/* ========== Button Icons ========== */
.btn-icon {
  width: 14px;
  height: 14px;
  margin-right: 6px;
}

/* ========== Input Field ========== */
.input-field {
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  background: var(--color-bg-surface);
  transition: border-color var(--transition-normal), box-shadow var(--transition-normal);
}

.input-field:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px var(--color-primary-light);
}

.input-sm {
  padding: var(--space-1) var(--space-2);
  font-size: var(--font-size-sm);
}

/* ========== Responsive ========== */
@media (max-width: 768px) {
  .selection-mode {
    flex-wrap: wrap;
  }

  .date-inputs {
    flex-direction: column;
  }

  .commit-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-1);
  }
}
</style>
