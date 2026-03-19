<script setup lang="ts">
import type { PackProgress } from "@/types";

defineProps<{
  progress: PackProgress;
}>();
</script>

<template>
  <div class="pack-progress">
    <div class="progress-header">
      <span class="progress-label">打包进度</span>
      <span class="progress-percentage">{{ progress.percentage.toFixed(1) }}%</span>
    </div>

    <div class="progress-bar">
      <div class="progress-fill" :style="{ width: `${progress.percentage}%` }"></div>
    </div>

    <div class="progress-details">
      <span class="progress-count">{{ progress.processed }} / {{ progress.total }} 文件</span>
      <span class="current-file" :title="progress.currentFile">
        <svg class="file-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
          <polyline points="14 2 14 8 20 8"/>
        </svg>
        {{ progress.currentFile }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.pack-progress {
  padding: var(--space-4) 0;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-2);
}

.progress-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
}

.progress-percentage {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--color-primary);
  font-family: var(--font-mono);
}

.progress-bar {
  height: 8px;
  background: var(--color-bg-hover);
  border-radius: var(--radius-full);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--color-primary);
  transition: width 300ms ease;
  border-radius: var(--radius-full);
}

.progress-details {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: var(--space-2);
  gap: var(--space-3);
}

.progress-count {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
  font-family: var(--font-mono);
}

.current-file {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  font-size: var(--font-size-xs);
  color: var(--color-text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  justify-content: flex-end;
}

.file-icon {
  width: 12px;
  height: 12px;
  flex-shrink: 0;
  color: var(--color-text-muted);
}
</style>
