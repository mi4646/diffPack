<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { usePackStore, useAppStore } from "@/stores";
import AppDialog from "@/components/common/AppDialog.vue";

const packStore = usePackStore();
const appStore = useAppStore();

const defaultFormat = ref<"zip" | "tarGz">("zip");
const defaultOutputPath = ref("");

// commit ID 显示开关直接绑定 appStore，切换即时生效并自动持久化
const showFullCommitId = computed({
  get: () => appStore.showFullCommitId,
  set: (val: boolean) => appStore.setShowFullCommitId(val),
});

// 弹框状态
const showDialog = ref(false);
const dialogTitle = ref("");
const dialogMessage = ref("");
const dialogType = ref<"success" | "error" | "warning" | "info">("info");
const showConfirmDialog = ref(false);

onMounted(() => {
  defaultFormat.value = packStore.options.format;
  defaultOutputPath.value = packStore.options.outputPath;
});

function saveSettings() {
  packStore.updateOptions({
    format: defaultFormat.value,
    outputPath: defaultOutputPath.value,
  });
  // 保存到本地存储
  localStorage.setItem("settings", JSON.stringify({
    defaultFormat: defaultFormat.value,
    defaultOutputPath: defaultOutputPath.value,
  }));
  dialogTitle.value = "保存成功";
  dialogMessage.value = "设置已成功保存。";
  dialogType.value = "success";
  showDialog.value = true;
}

async function browseOutputPath() {
  const path = await packStore.selectOutputDirectory();
  if (path) {
    defaultOutputPath.value = path;
  }
}

function clearHistory() {
  showConfirmDialog.value = true;
}

function doConfirmClear() {
  packStore.clearHistory();
  dialogTitle.value = "已清除";
  dialogMessage.value = "历史记录已全部清除。";
  dialogType.value = "success";
  showDialog.value = true;
}
</script>

<template>
  <div class="settings-page">
    <section class="card">
      <h3>默认设置</h3>
      
      <div class="setting-row">
        <label>默认打包格式：</label>
        <select v-model="defaultFormat">
          <option value="zip">ZIP</option>
          <option value="tarGz">tar.gz</option>
        </select>
      </div>

      <div class="setting-row">
        <label>默认输出目录：</label>
        <input type="text" v-model="defaultOutputPath" placeholder="选择默认输出目录..." />
        <button @click="browseOutputPath">浏览</button>
      </div>

      <div class="setting-row">
        <label>Commit ID 格式：</label>
        <div class="radio-group">
          <label class="radio-label">
            <input type="radio" :value="false" v-model="showFullCommitId" />
            短 ID（7 位）
          </label>
          <label class="radio-label">
            <input type="radio" :value="true" v-model="showFullCommitId" />
            完整 ID（40 位）
          </label>
        </div>
      </div>

      <button @click="saveSettings" class="save-btn">保存设置</button>
    </section>

    <section class="card">
      <h3>历史记录</h3>
      <p class="history-count">共 {{ packStore.history.length }} 条打包记录</p>
      <button @click="clearHistory" class="clear-btn">清除历史记录</button>
    </section>

    <section class="card">
      <h3>关于</h3>
      <p><strong>DiffPack</strong> - Git 差异项目版本打包器</p>
      <p class="version">版本: 0.1.0</p>
      <p class="tech-stack">技术栈: Tauri 2.x + Vue 3 + TypeScript</p>
    </section>
  </div>

  <!-- 成功/信息弹框 -->
  <AppDialog
    :visible="showDialog"
    :title="dialogTitle"
    :message="dialogMessage"
    :type="dialogType"
    @close="showDialog = false"
  />

  <!-- 确认清除历史弹框 -->
  <AppDialog
    :visible="showConfirmDialog"
    title="确认清除"
    message="确定要清除所有历史记录吗？此操作无法恢复。"
    type="warning"
    :show-cancel="true"
    confirm-text="确认清除"
    @confirm="doConfirmClear"
    @close="showConfirmDialog = false"
  />
</template>

<style scoped>
.settings-page {
  display: flex;
  flex-direction: column;
  gap: 20px;
  max-width: 600px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.setting-row label {
  min-width: 120px;
}

.setting-row input {
  flex: 1;
}

.save-btn {
  margin-top: 8px;
}

.history-count {
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.clear-btn {
  background: var(--error-color);
}

.clear-btn:hover {
  background: #ff4d4f;
}

.version, .tech-stack {
  color: var(--text-secondary);
  font-size: 13px;
  margin-top: 8px;
}

.radio-group {
  display: flex;
  gap: 16px;
  align-items: center;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 5px;
  cursor: pointer;
  font-size: 13px;
  min-width: unset;
}
</style>
