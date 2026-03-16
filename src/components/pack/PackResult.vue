<script setup lang="ts">
import type { PackResult } from "@/types";

defineProps<{
  result: PackResult;
}>();

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
  <div class="pack-result" :class="{ success: result.success, error: !result.success }">
    <div v-if="result.success" class="result-success">
      <div class="result-icon">✓</div>
      <h4>打包完成</h4>
      <div class="result-details">
        <p><strong>输出路径：</strong>{{ result.outputPath }}</p>
        <p><strong>文件数量：</strong>{{ result.fileCount }}</p>
        <p><strong>文件大小：</strong>{{ formatSize(result.totalSize) }}</p>
        <p><strong>耗时：</strong>{{ formatDuration(result.durationMs) }}</p>
      </div>
    </div>
    <div v-else class="result-error">
      <div class="result-icon">✗</div>
      <h4>打包失败</h4>
      <p class="error-message">{{ result.error }}</p>
    </div>
  </div>
</template>

<style scoped>
.pack-result {
  padding: 20px;
  border-radius: 8px;
}

.pack-result.success {
  background: #f6ffed;
  border: 1px solid #b7eb8f;
}

.pack-result.error {
  background: #fff2f0;
  border: 1px solid #ffccc7;
}

.result-icon {
  font-size: 32px;
  margin-bottom: 8px;
}

.success .result-icon {
  color: var(--success-color);
}

.error .result-icon {
  color: var(--error-color);
}

.result-success h4,
.result-error h4 {
  margin-bottom: 12px;
}

.result-details {
  font-size: 14px;
}

.result-details p {
  margin: 4px 0;
}

.error-message {
  color: var(--error-color);
}
</style>
