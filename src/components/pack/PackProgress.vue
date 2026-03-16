<script setup lang="ts">
import type { PackProgress } from "@/types";

defineProps<{
  progress: PackProgress;
}>();
</script>

<template>
  <div class="pack-progress">
    <div class="progress-bar">
      <div class="progress-fill" :style="{ width: `${progress.percentage}%` }"></div>
    </div>
    <div class="progress-info">
      <span>{{ progress.processed }} / {{ progress.total }} 文件</span>
      <span>{{ progress.percentage.toFixed(1) }}%</span>
    </div>
    <div class="current-file">
      正在处理: {{ progress.currentFile }}
    </div>
  </div>
</template>

<style scoped>
.pack-progress {
  padding: 16px 0;
}

.progress-bar {
  height: 8px;
  background: var(--bg-color);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--primary-color);
  transition: width 0.3s ease;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.current-file {
  margin-top: 8px;
  font-size: 12px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
