<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { usePackStore, useAppStore } from "@/stores";
import { useNotifier } from "@/composables";
import AppDialog from "@/components/common/AppDialog.vue";

const packStore = usePackStore();
const appStore = useAppStore();
const { success } = useNotifier();

const defaultFormat = ref<"zip" | "tarGz">("zip");
const defaultOutputPath = ref("");
const showConfirmDialog = ref(false);

// commit ID 显示开关直接绑定 appStore，切换即时生效并自动持久化
const showFullCommitId = computed({
  get: () => appStore.showFullCommitId,
  set: (val: boolean) => appStore.setShowFullCommitId(val),
});

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
  success("保存成功", "设置已成功保存。");
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
  success("已清除", "历史记录已全部清除。");
  showConfirmDialog.value = false;
}
</script>

<template>
  <div class="settings-page">
    <section class="card">
      <div class="card-header">
        <h2 class="card-title">
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
          默认设置
        </h2>
      </div>

      <div class="setting-group">
        <label class="form-label">默认打包格式</label>
        <select v-model="defaultFormat" class="input-field">
          <option value="zip">ZIP</option>
          <option value="tarGz">tar.gz</option>
        </select>
      </div>

      <div class="setting-group">
        <label class="form-label">默认输出目录</label>
        <div class="input-with-button">
          <input
            type="text"
            v-model="defaultOutputPath"
            class="input-field"
            placeholder="选择默认输出目录..."
            readonly
          />
          <button class="btn-secondary" @click="browseOutputPath">
            <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
            浏览
          </button>
        </div>
      </div>

      <div class="setting-group">
        <label class="form-label">Commit ID 格式</label>
        <div class="radio-group">
          <label class="radio-label">
            <input type="radio" :value="false" v-model="showFullCommitId" class="radio-input" />
            <span class="radio-control"></span>
            <span class="radio-text">短 ID（7 位）</span>
          </label>
          <label class="radio-label">
            <input type="radio" :value="true" v-model="showFullCommitId" class="radio-input" />
            <span class="radio-control"></span>
            <span class="radio-text">完整 ID（40 位）</span>
          </label>
        </div>
      </div>

      <button class="btn-primary save-btn" @click="saveSettings">
        <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
          <polyline points="17 21 17 13 7 13 7 21"/>
          <polyline points="7 3 7 8 15 8"/>
        </svg>
        保存设置
      </button>
    </section>

    <section class="card">
      <div class="card-header">
        <h2 class="card-title">
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <polyline points="12 6 12 12 16 14"/>
          </svg>
          历史记录
        </h2>
      </div>

      <div class="history-info">
        <span class="history-count">共 {{ packStore.history.length }} 条打包记录</span>
        <button class="btn-danger" @click="clearHistory" :disabled="packStore.history.length === 0">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
          </svg>
          清除历史记录
        </button>
      </div>
    </section>

    <section class="card">
      <div class="card-header">
        <h2 class="card-title">
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          关于
        </h2>
      </div>

      <div class="about-content">
        <div class="app-name">
          <svg class="app-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
            <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
            <line x1="12" y1="22.08" x2="12" y2="12"/>
          </svg>
          <div>
            <strong>DiffPack</strong>
            <span class="app-subtitle">Git 差异项目版本打包器</span>
          </div>
        </div>

        <div class="about-details">
          <div class="detail-row">
            <span class="detail-label">版本</span>
            <span class="detail-value">0.1.0</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">技术栈</span>
            <span class="detail-value">Tauri 2.x + Vue 3 + TypeScript</span>
          </div>
        </div>
      </div>
    </section>
  </div>

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
  gap: var(--space-4);
  max-width: 600px;
  margin: 0 auto;
}

.title-icon {
  width: 18px;
  height: 18px;
  margin-right: var(--space-2);
  color: var(--color-text-muted);
}

.setting-group {
  margin-bottom: var(--space-4);
}

.setting-group .form-label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-2);
}

.input-field {
  width: 100%;
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

.input-with-button {
  display: flex;
  gap: var(--space-2);
}

.input-with-button .input-field {
  flex: 1;
}

.btn-icon {
  width: 14px;
  height: 14px;
  margin-right: 6px;
}

.radio-group {
  display: flex;
  gap: var(--space-4);
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
  flex-shrink: 0;
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

.save-btn {
  margin-top: var(--space-2);
  display: flex;
  align-items: center;
}

.history-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.history-count {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
}

/* About section */
.about-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.app-name {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.app-icon {
  width: 40px;
  height: 40px;
  color: var(--color-primary);
  flex-shrink: 0;
}

.app-name strong {
  display: block;
  font-size: var(--font-size-lg);
  color: var(--color-text-primary);
}

.app-subtitle {
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
}

.about-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-md);
}

.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.detail-label {
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
}

.detail-value {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
}
</style>
