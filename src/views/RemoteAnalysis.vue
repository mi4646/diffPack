<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSshStore, useGitStore, usePackStore } from "@/stores";
import SshConfigForm from "@/components/ssh/SshConfigForm.vue";
import SshConnectionStatus from "@/components/ssh/SshConnectionStatus.vue";
import PackOptions from "@/components/pack/PackOptions.vue";
import PackProgress from "@/components/pack/PackProgress.vue";
import PackResult from "@/components/pack/PackResult.vue";
import AppDialog from "@/components/common/AppDialog.vue";

const sshStore = useSshStore();
const gitStore = useGitStore();
const packStore = usePackStore();

const remoteRepoPath = ref("");
const selectionMode = ref<"range" | "date">("range");

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

onMounted(() => {
  sshStore.loadSshConfig();
});

async function loadRemoteCommits() {
  if (!sshStore.sessionId || !remoteRepoPath.value) return;
  // 调用远程获取 commits
}

async function analyzeRemoteDiff() {
  if (!sshStore.sessionId || !remoteRepoPath.value || gitStore.selectedCommits.length < 2) return;
  // 调用远程差异分析
}

async function startPack() {
  const changes = gitStore.getSelectedChanges();
  if (changes.length === 0) return;

  try {
    await packStore.startPack(
      "remote",
      changes,
      sshStore.sessionId!,
      remoteRepoPath.value
    );
  } catch (e) {
    const errStr = e instanceof Error ? e.message : String(e);
    showError("打包失败", errStr);
  }
}
</script>

<template>
  <div class="remote-analysis">
    <!-- SSH 连接配置 -->
    <section class="card ssh-section">
      <h3>SSH 连接</h3>
      <SshConnectionStatus v-if="sshStore.isConnected" />
      <SshConfigForm v-else />
    </section>

    <!-- 远程仓库路径 -->
    <section v-if="sshStore.isConnected" class="card repo-section">
      <h3>远程仓库路径</h3>
      <div class="repo-input-row">
        <input
          v-model="remoteRepoPath"
          type="text"
          placeholder="输入远程服务器上的 Git 仓库路径..."
        />
        <button @click="loadRemoteCommits" :disabled="!remoteRepoPath">
          加载
        </button>
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
          <input type="radio" v-model="selectionMode" value="date" />
          时间范围
        </label>
      </div>

      <div class="commit-list">
        <div
          v-for="commit in gitStore.commits"
          :key="commit.hash"
          class="commit-item"
          :class="{
            selected: gitStore.selectedCommits.includes(commit.hash),
          }"
          @click="gitStore.selectCommitRange(commit.hash, gitStore.selectedCommits[0] || commit.hash)"
        >
          <span class="commit-hash">{{ commit.shortHash }}</span>
          <span class="commit-message">{{ commit.message }}</span>
          <span class="commit-date">{{ commit.date }}</span>
        </div>
      </div>

      <div class="selected-info" v-if="gitStore.selectedCommits.length >= 2">
        <span>已选择 {{ gitStore.selectedCommits.length }} 个 commit</span>
        <button @click="analyzeRemoteDiff" :disabled="gitStore.isLoading">
          分析差异
        </button>
      </div>
    </section>

    <!-- 差异预览 -->
    <section v-if="gitStore.hasDiff" class="card diff-section">
      <h3>差异预览</h3>
      <div class="diff-stats">
        <span>共 {{ gitStore.diffResult?.totalFiles }} 个文件变更</span>
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
          />
          <span class="change-type" :class="change.changeType">
            {{ change.changeType }}
          </span>
          <span class="file-path">{{ change.path }}</span>
        </div>
      </div>
    </section>

    <!-- 打包 -->
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
</template>

<style scoped>
.remote-analysis {
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

.commit-list {
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.commit-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
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

.commit-hash {
  font-family: monospace;
  color: var(--primary-color);
  font-weight: 500;
}

.commit-message {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.commit-date {
  font-size: 12px;
  color: var(--text-secondary);
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

.file-path {
  flex: 1;
  font-family: monospace;
  font-size: 13px;
}
</style>
