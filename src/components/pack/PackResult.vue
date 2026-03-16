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
      <div class="detail-row">
        <span class="detail-label">输出路径：</span>
        <span class="detail-value path">{{ result.outputPath }}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">文件数量：</span>
        <span class="detail-value">{{ result.fileCount }} 个文件</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">文件大小：</span>
        <span class="detail-value">{{ formatSize(result.totalSize) }}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">耗时：</span>
        <span class="detail-value">{{ formatDuration(result.durationMs) }}</span>
      </div>
    </div>
    <div v-else class="result-error-msg">
      {{ result.error }}
    </div>
  </AppDialog>
</template>

<style scoped>
.result-details {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.detail-row {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  font-size: 14px;
}

.detail-label {
  min-width: 80px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.detail-value {
  color: var(--text-color);
  flex: 1;
  word-break: break-all;
}

.detail-value.path {
  font-family: monospace;
  font-size: 13px;
}

.result-error-msg {
  color: var(--error-color);
  font-size: 14px;
  line-height: 1.6;
  word-break: break-all;
}
</style>
