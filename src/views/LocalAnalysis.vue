<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useGitStore, usePackStore, useAppStore } from "@/stores";
import { openInExplorer } from "@/services/packService";
import PackOptions from "@/components/pack/PackOptions.vue";
import PackProgress from "@/components/pack/PackProgress.vue";
import PackResult from "@/components/pack/PackResult.vue";
import AppDialog from "@/components/common/AppDialog.vue";
import type { CommitInfo } from "@/types";

const gitStore = useGitStore();
const packStore = usePackStore();
const appStore = useAppStore();

const repoPathInput = ref("");
const selectionMode = ref<"range" | "list" | "date">("range");
const startDate = ref("");
const endDate = ref("");
// commit 搜索关键词
const commitSearchKeyword = ref("");

// 弹框状态
const showDialog = ref(false);
const dialogTitle = ref("");
const dialogMessage = ref("");
const dialogType = ref<"success" | "error" | "warning" | "info">("error");

function showError(title: string, message: string) {
  dialogTitle.value = title;
  dialogMessage.value = message;
  dialogType.value = "error";
  showDialog.value = true;
}

// commit 详情弹框
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
      showError(
        "未找到 Git 仓库",
        `所选路径不是有效的 Git 仓库，请确认该目录已执行过 git init 或是从远程克隆的仓库。\n\n路径：${repoPathInput.value}`
      );
    } else {
      showError("打开仓库失败", errStr);
    }
  }
}

async function loadCommits() {
  const options: { startDate?: string; endDate?: string; limit?: number } = {};
  if (selectionMode.value === "date" && startDate.value) {
    options.startDate = startDate.value;
    if (endDate.value) {
      options.endDate = endDate.value;
    }
  }
  await gitStore.loadCommits(options);
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
    showError("分析差异失败", errStr);
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
      showError("加载提交失败", errStr);
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
      showError("加载提交失败", errStr);
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
  <div class="local-analysis">
    <!-- 仓库选择 -->
    <section class="card repo-section">
      <h3>选择 Git 仓库</h3>
      <div class="repo-input-row">
        <input
          v-model="repoPathInput"
          type="text"
          placeholder="输入仓库路径或点击浏览..."
          class="path-input"
          @keyup.enter="openRepo"
          @dblclick="handlePathClick(repoPathInput)"
        />
        <button @click="browseRepo">浏览</button>
        <button @click="openRepo" :disabled="!repoPathInput">打开</button>
      </div>
      <div v-if="gitStore.repoInfo" class="repo-info">
        <span class="repo-name">{{ gitStore.repoInfo.name }}</span>
        <span class="branch-badge">{{ gitStore.repoInfo.currentBranch }}</span>
      </div>

    </section>

    <!-- Commit 选择 -->
    <section v-if="gitStore.hasRepo" class="card commit-section">
      <h3>选择 Commit</h3>
      <div class="selection-mode">
        <label>
          <input type="radio" v-model="selectionMode" value="range" />
          范围选择
        </label>
        <label>
          <input type="radio" v-model="selectionMode" value="list" />
          列表选择
        </label>
        <label>
          <input type="radio" v-model="selectionMode" value="date" />
          时间范围
        </label>
      </div>

      <!-- 时间范围选择 -->
      <div v-if="selectionMode === 'date'" class="date-range">
        <div class="date-presets">
          <button
            v-for="preset in datePresets"
            :key="preset.days"
            class="preset-btn"
            @click="setDatePreset(preset.days)"
          >
            {{ preset.label }}
          </button>
        </div>
        <div class="date-inputs">
          <label>
            开始日期：
            <input type="date" v-model="startDate" />
          </label>
          <label>
            结束日期：
            <input type="date" v-model="endDate" />
          </label>
        </div>
      </div>

      <!-- commit 搜索框 -->
      <div class="commit-search-row">
        <input
          v-model="commitSearchKeyword"
          type="text"
          class="commit-search-input"
          placeholder="搜索 commit message、作者、hash..."
          clearable
        />
        <span v-if="commitSearchKeyword" class="search-clear" @click="commitSearchKeyword = ''">&#x2715;</span>
        <span class="commit-count">{{ filteredCommits.length }} / {{ gitStore.commits.length }} 个</span>
      </div>

      <!-- Commit 列表 -->
      <div class="commit-mode-hint">
        <span v-if="selectionMode === 'range' || selectionMode === 'date'">
          点击选择起始 commit，再次点击选择终止 commit
        </span>
        <span v-else-if="selectionMode === 'list'">
          点击可多选，将对第一个和第二个选中的 commit 进行差异分析
        </span>
      </div>
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
          <!-- 第一行：hash + 作者 + 时间 + 详情 -->
          <div class="commit-header">
            <div class="commit-header-left">
              <span class="commit-hash-badge" :title="commit.hash">
                {{ appStore.showFullCommitId ? commit.hash : commit.shortHash }}
              </span>
              <span class="commit-author">{{ commit.author }}</span>
              <span class="commit-date">{{ commit.date }}</span>
            </div>
            <button
              class="detail-btn"
              title="查看详情"
              @click.stop="showCommitDetail(commit)"
            >详情</button>
          </div>
          <!-- 第二行：commit 描述 -->
          <div class="commit-body">
            <span class="commit-message">{{ commit.message }}</span>
          </div>
        </div>
      </div>

      <div class="selected-info" v-if="gitStore.selectedCommits.length > 0">
        已选择 {{ gitStore.selectedCommits.length }} 个 commit
        <button
          @click="analyzeDiff"
          :disabled="gitStore.selectedCommits.length < 2 || gitStore.isLoading"
        >
          分析差异
        </button>
      </div>
    </section>

    <!-- 差异预览 -->
    <section v-if="gitStore.hasDiff" class="card diff-section">
      <h3>差异预览</h3>
      <div class="diff-stats">
        <span>共 {{ gitStore.diffResult?.totalFiles }} 个文件变更</span>
        <span class="text-success">+{{ gitStore.diffResult?.totalAdditions }}</span>
        <span class="text-error">-{{ gitStore.diffResult?.totalDeletions }}</span>
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
          />
          <span
            class="change-type"
            :class="{
              added: change.changeType === 'added',
              modified: change.changeType === 'modified',
              deleted: change.changeType === 'deleted',
              renamed: change.changeType === 'renamed',
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
        <button @click="gitStore.selectAllFiles">全选</button>
        <button @click="gitStore.deselectAllFiles">取消全选</button>
      </div>
    </section>

    <!-- 打包选项 -->
    <section v-if="gitStore.hasDiff" class="card pack-section">
      <h3>打包选项</h3>
      <PackOptions @pack="startPack" />
      
      <PackProgress v-if="packStore.progress" :progress="packStore.progress" />
      
      <PackResult v-if="packStore.currentTask?.result" :result="packStore.currentTask.result" />
    </section>
  </div>

  <AppDialog
    :visible="showDialog"
    :title="dialogTitle"
    :message="dialogMessage"
    :type="dialogType"
    @close="showDialog = false"
  />

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
.local-analysis {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.repo-input-row {
  display: flex;
  gap: 8px;
}

.repo-input-row input {
  flex: 1;
}

.repo-input-row .path-input {
  cursor: pointer;
}

.repo-input-row .path-input:hover {
  background-color: var(--bg-color);
}

.repo-info {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
}

.repo-name {
  font-weight: 500;
}

.branch-badge {
  background: var(--primary-color);
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
}

.error-message {
  color: var(--error-color);
  margin-top: 8px;
}

.selection-mode {
  display: flex;
  gap: 16px;
  margin-bottom: 16px;
}

.selection-mode label {
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
}

.date-range {
  margin-bottom: 16px;
}

.date-presets {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.preset-btn {
  padding: 4px 12px;
  font-size: 12px;
  background: var(--bg-color);
  color: var(--text-color);
}

.preset-btn:hover {
  background: var(--primary-color);
  color: white;
}

.date-inputs {
  display: flex;
  gap: 16px;
}

.date-inputs label {
  display: flex;
  align-items: center;
  gap: 8px;
}

.commit-mode-hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 8px;
  padding: 4px 0;
}

.commit-search-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  position: relative;
}

.commit-search-input {
  flex: 1;
  padding-right: 28px;
}

.search-clear {
  position: absolute;
  right: 70px;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 13px;
  line-height: 1;
  user-select: none;
}

.search-clear:hover {
  color: var(--error-color);
}

.commit-count {
  white-space: nowrap;
  font-size: 12px;
  color: var(--text-secondary);
  min-width: 60px;
  text-align: right;
}

.commit-list {
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.commit-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
  transition: background-color 0.2s;
}

.commit-item:last-child {
  border-bottom: none;
}

.commit-item:hover {
  background: var(--bg-color);
}

.commit-item.selected {
  background: #e6f7ff;
}

.commit-item.selected-first {
  background: #bae7ff;
}

.commit-item.selected-second {
  background: #91d5ff;
}

/* 第一行：hash + 作者 + 日期 + 详情按钮 */
.commit-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.commit-header-left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  min-width: 0;
}

.commit-hash-badge {
  font-family: monospace;
  font-size: 12px;
  font-weight: 600;
  color: var(--primary-color);
  background: rgba(24, 144, 255, 0.08);
  border: 1px solid rgba(24, 144, 255, 0.25);
  border-radius: 4px;
  padding: 1px 6px;
  white-space: nowrap;
  flex-shrink: 0;
  cursor: default;
}

.commit-author {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-color);
  white-space: nowrap;
}

.commit-date {
  font-size: 12px;
  color: var(--text-secondary);
  white-space: nowrap;
}

.detail-btn {
  padding: 1px 8px;
  font-size: 11px;
  height: auto;
  line-height: 1.6;
  background: transparent;
  color: var(--primary-color);
  border: 1px solid rgba(24, 144, 255, 0.35);
  border-radius: 3px;
  flex-shrink: 0;
  cursor: pointer;
  transition: all 0.15s;
}

.detail-btn:hover {
  background: rgba(24, 144, 255, 0.08);
}

/* 第二行：commit 说明 */
.commit-body {
  padding-left: 2px;
}

.commit-message {
  font-size: 12px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
}

.selected-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-color);
}

.diff-stats {
  display: flex;
  gap: 16px;
  margin-bottom: 12px;
}

.file-list {
  max-height: 300px;
  overflow-y: auto;
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
}

.file-item:last-child {
  border-bottom: none;
}

.file-item:hover {
  background: var(--bg-color);
}

.file-item.selected {
  background: #f6ffed;
}

.change-type {
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 3px;
  text-transform: uppercase;
  font-weight: 500;
}

.change-type.added {
  background: #d9f7be;
  color: #389e0d;
}

.change-type.modified {
  background: #fff1b8;
  color: #d48806;
}

.change-type.deleted {
  background: #ffccc7;
  color: #cf1322;
}

.change-type.renamed {
  background: #e6f7ff;
  color: #0958d9;
}

.file-path {
  flex: 1;
  font-family: monospace;
  font-size: 13px;
  cursor: pointer;
}

.file-path:hover {
  text-decoration: underline;
  color: var(--primary-color);
}

.old-path {
  color: var(--text-secondary);
  font-size: 12px;
}

.file-actions {
  display: flex;
  gap: 8px;
  margin-top: 12px;
}

.file-actions button {
  background: var(--bg-color);
  color: var(--text-color);
}

.file-actions button:hover {
  background: var(--primary-color);
  color: white;
}


</style>
