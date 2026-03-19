<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useSshStore, useGitStore, usePackStore } from "@/stores";
import { useNotifier, useLoading } from "@/composables";
import SshConfigForm from "@/components/ssh/SshConfigForm.vue";
import SshConnectionStatus from "@/components/ssh/SshConnectionStatus.vue";
import PackOptions from "@/components/pack/PackOptions.vue";
import PackProgress from "@/components/pack/PackProgress.vue";
import PackResult from "@/components/pack/PackResult.vue";
import LoadingOverlay from "@/components/common/LoadingOverlay.vue";

const sshStore = useSshStore();
const gitStore = useGitStore();
const packStore = usePackStore();
const { error, success } = useNotifier();
const { isLoading, text: loadingText, startLoading, stopLoading } = useLoading();

const remoteRepoPath = ref("");
const selectionMode = ref<"range" | "list">("range");

// 计算属性
const canAnalyze = computed(() => {
  return selectionMode.value === "list"
    ? gitStore.selectedCommits.length > 0
    : gitStore.selectedCommits.length >= 2;
});

// 选择 commit
function handleCommitSelect(commit: { hash: string }) {
  const selected = gitStore.selectedCommits;

  if (selectionMode.value === "range") {
    // 范围模式：依次选两个端点
    if (selected.length === 0) {
      gitStore.selectCommitRange(commit.hash, commit.hash);
    } else if (selected.length === 1) {
      if (selected[0] === commit.hash) {
        // 点击同一个 commit 取消选择
        gitStore.selectCommitRange("", "");
      } else {
        gitStore.selectCommitRange(selected[0], commit.hash);
      }
    } else {
      // 已选两个，重新开始
      gitStore.selectCommitRange(commit.hash, commit.hash);
    }
  }
  // 列表模式暂未实现
}

function resetSelection() {
  gitStore.selectCommitRange("", "");
}

onMounted(() => {
  sshStore.loadSshConfig();
});

async function loadRemoteCommits() {
  if (!sshStore.sessionId || !remoteRepoPath.value) return;

  startLoading("正在加载远程提交...");
  try {
    // TODO: 实现远程加载提交逻辑
    await new Promise(resolve => setTimeout(resolve, 1000));
  } catch (e) {
    const errStr = e instanceof Error ? e.message : String(e);
    error("加载失败", errStr);
  } finally {
    stopLoading();
  }
}

async function analyzeRemoteDiff() {
  if (!sshStore.sessionId || !remoteRepoPath.value || gitStore.selectedCommits.length < 2) return;

  startLoading("正在分析远程差异...");
  try {
    // TODO: 实现远程差异分析逻辑
    await new Promise(resolve => setTimeout(resolve, 1000));
  } catch (e) {
    const errStr = e instanceof Error ? e.message : String(e);
    error("分析失败", errStr);
  } finally {
    stopLoading();
  }
}

async function startPack() {
  const changes = gitStore.getSelectedChanges();
  if (changes.length === 0) {
    error("打包失败", "请至少选择一个文件");
    return;
  }

  try {
    await packStore.startPack(
      "remote",
      changes,
      sshStore.sessionId!,
      remoteRepoPath.value
    );
    success("打包成功", "打包任务已启动");
  } catch (e) {
    const errStr = e instanceof Error ? e.message : String(e);
    error("打包失败", errStr);
  }
}

</script>

<template>
  <div class="page-container">
    <!-- SSH 连接配置 -->
    <section class="card">
      <div class="card-header">
        <h2 class="card-title">
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
            <line x1="8" y1="21" x2="16" y2="21"/>
            <line x1="12" y1="17" x2="12" y2="21"/>
          </svg>
          SSH 连接
        </h2>
      </div>
      <SshConnectionStatus v-if="sshStore.isConnected" />
      <SshConfigForm v-else />
    </section>

    <!-- 远程仓库路径 -->
    <section v-if="sshStore.isConnected" class="card">
      <div class="card-header">
        <h2 class="card-title">
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <ellipse cx="12" cy="5" rx="9" ry="3"/>
            <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
            <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
          </svg>
          远程仓库路径
        </h2>
      </div>
      <div class="repo-input-row">
        <input
          v-model="remoteRepoPath"
          type="text"
          class="input-field"
          placeholder="输入远程服务器上的 Git 仓库路径..."
        />
        <button class="btn-primary" @click="loadRemoteCommits" :disabled="!remoteRepoPath">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="17 8 12 3 7 8"/>
            <line x1="12" y1="3" x2="12" y2="15"/>
          </svg>
          加载
        </button>
      </div>
    </section>

    <!-- Commit 选择 -->
    <section v-if="gitStore.hasRepo" class="card">
      <div class="card-header">
        <h2 class="card-title">选择 Commit</h2>
      </div>

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
        <button class="btn-text" @click="resetSelection">清空选择</button>
      </div>

      <div class="commit-list">
        <div
          v-for="commit in gitStore.commits"
          :key="commit.hash"
          class="commit-item"
          :class="{
            selected: gitStore.selectedCommits.includes(commit.hash),
          }"
          @click="handleCommitSelect(commit)"
        >
          <span class="commit-hash">{{ commit.shortHash }}</span>
          <span class="commit-message">{{ commit.message }}</span>
          <span class="commit-date">{{ commit.date }}</span>
        </div>
      </div>

      <div v-if="canAnalyze" class="selected-info">
        <span>已选择 {{ gitStore.selectedCommits.length }} 个 commit</span>
        <button class="btn-primary" @click="analyzeRemoteDiff" :disabled="isLoading">
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
        </div>
      </div>

      <div class="file-list">
        <div
          v-for="change in gitStore.diffResult?.changes"
          :key="change.path"
          class="file-item"
          :class="{ selected: gitStore.selectedFiles.includes(change.path) }"
          @click="gitStore.toggleFileSelection(change.path)"
        >
          <input
            type="checkbox"
            :checked="gitStore.selectedFiles.includes(change.path)"
            @click.stop
            class="checkbox"
          />
          <span class="change-badge" :class="`change-${change.changeType}`">
            {{ change.changeType }}
          </span>
          <span class="file-path">{{ change.path }}</span>
        </div>
      </div>
    </section>

    <!-- 打包 -->
    <section v-if="gitStore.hasDiff" class="card">
      <div class="card-header">
        <h2 class="card-title">打包选项</h2>
      </div>
      <PackOptions @pack="startPack" />

      <PackProgress v-if="packStore.progress" :progress="packStore.progress" />

      <PackResult v-if="packStore.currentTask?.result" :result="packStore.currentTask.result" />
    </section>
  </div>

  <LoadingOverlay :visible="isLoading" :text="loadingText" />
</template>

<style scoped>
.page-container {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  max-width: var(--max-content-width);
  margin: 0 auto;
}

.title-icon {
  width: 18px;
  height: 18px;
  margin-right: var(--space-2);
  color: var(--color-text-muted);
}

.repo-input-row {
  display: flex;
  gap: var(--space-2);
}

.repo-input-row .input-field {
  flex: 1;
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  background: var(--color-bg-surface);
}

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

.commit-list {
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-lg);
}

.commit-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  cursor: pointer;
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

.commit-hash {
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
  color: var(--color-primary);
  font-weight: var(--font-weight-medium);
}

.commit-message {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
}

.commit-date {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.selected-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: var(--space-4);
  padding-top: var(--space-4);
  border-top: 1px solid var(--color-border-light);
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
}

.diff-stats {
  display: flex;
  gap: var(--space-3);
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

.file-list {
  max-height: 300px;
  overflow-y: auto;
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-lg);
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

.change-added {
  background: var(--color-success-bg);
  color: var(--color-success);
}

.change-modified {
  background: var(--color-warning-bg);
  color: var(--color-warning);
}

.change-deleted {
  background: var(--color-error-bg);
  color: var(--color-error);
}

.change-renamed {
  background: var(--color-info-bg);
  color: var(--color-info);
}

.file-path {
  flex: 1;
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
}

.btn-icon {
  width: 14px;
  height: 14px;
  margin-right: 6px;
}
</style>
