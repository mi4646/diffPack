<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import AppDialog from "@/components/common/AppDialog.vue";
import type { PackResult } from "@/types";

const props = defineProps<{
  result: PackResult;
}>();

const showDialog = ref(false);

onMounted(() => {
  showDialog.value = true;
});

watch(() => props.result, () => {
  showDialog.value = true;
});

function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + " KB";
  return (bytes / (1024 * 1024)).toFixed(2) + " MB";
}

function formatDuration(ms: number): string {
  if (ms < 1000) return ms + " ms";
  return (ms / 1000).toFixed(2) + " s";
}
</script>

<template>
  <AppDialog
    :visible="showDialog"
    :title="result.success ? '打包完成' : '打包失败'"
    :type="result.success ? 'success' : 'error'"
    @close="showDialog = false"
  >
    <div v-if="result.success" class="result-details">
      <div class="detail-item">
        <span class="detail-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
          </svg>
        </span>
        <div class="detail-content">
          <span class="detail-label">输出路径</span>
          <span class="detail-value path">{{ result.outputPath }}</span>
        </div>
      </div>

      <div class="detail-stats">
        <div class="stat-item">
          <span class="stat-value">{{ result.fileCount }}</span>
          <span class="stat-label">文件</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat-item">
          <span class="stat-value">{{ formatSize(result.totalSize) }}</span>
          <span class="stat-label">大小</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat-item">
          <span class="stat-value">{{ formatDuration(result.durationMs) }}</span>
          <span class="stat-label">耗时</span>
        </div>
      </div>
    </div>
    <div v-else class="result-error">
      <svg class="error-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
      </svg>
      <span class="error-message">{{ result.error }}</span>
    </div>
  </AppDialog>
</template>

<style scoped>
.result-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.detail-item {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  padding: var(--space-3);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-md);
}

.detail-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-success-bg);
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.detail-icon svg {
  width: 18px;
  height: 18px;
  color: var(--color-success);
}

.detail-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  min-width: 0;
}

.detail-label {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.detail-value {
  font-size: var(--font-size-sm);
  color: var(--color-text-primary);
  word-break: break-all;
}

.detail-value.path {
  font-family: var(--font-mono);
  font-size: var(--font-size-xs);
}

/* Stats row */
.detail-stats {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-4);
  padding: var(--space-4);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-md);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-1);
}

.stat-value {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  font-family: var(--font-mono);
}

.stat-label {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.stat-divider {
  width: 1px;
  height: 32px;
  background: var(--color-border-light);
}

/* Error state */
.result-error {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  padding: var(--space-3);
  background: var(--color-error-bg);
  border-radius: var(--radius-md);
}

.error-icon {
  width: 20px;
  height: 20px;
  color: var(--color-error);
  flex-shrink: 0;
}

.error-message {
  font-size: var(--font-size-sm);
  color: var(--color-error);
  line-height: var(--line-height-relaxed);
  word-break: break-all;
}
</style>
