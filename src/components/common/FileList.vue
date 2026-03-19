<script setup lang="ts">
import type { FileChange } from "@/types";

const props = withDefaults(
  defineProps<{
    files: FileChange[];
    selectedFiles: string[];
    emptyText?: string;
  }>(),
  {
    emptyText: "暂无变更文件",
  }
);

const emit = defineEmits<{
  toggle: [path: string];
  "update:selectedFiles": [paths: string[]];
  selectAll: [];
  deselectAll: [];
}>();

function isSelected(path: string): boolean {
  return props.selectedFiles.includes(path);
}

function toggleSelect(path: string) {
  emit("toggle", path);
}

function getChangeIcon(changeType: string): string {
  const icons: Record<string, string> = {
    added: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14M5 12h14"/></svg>`,
    modified: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>`,
    deleted: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>`,
    renamed: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="12" y1="18" x2="12" y2="12"/><line x1="9" y1="15" x2="15" y2="15"/></svg>`,
    copied: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>`,
  };
  return icons[changeType] || icons.modified;
}

function getFileName(path: string): string {
  const parts = path.split("/");
  return parts[parts.length - 1];
}

function getFileDir(path: string): string {
  const parts = path.split("/");
  if (parts.length > 1) {
    return parts.slice(0, -1).join("/");
  }
  return "";
}
</script>

<template>
  <div class="file-list">
    <div v-if="files.length > 0" class="file-list-header">
      <span class="file-count">{{ selectedFiles.length }} / {{ files.length }}</span>
      <div class="file-list-actions">
        <button class="action-btn" @click="emit('selectAll')">全选</button>
        <button class="action-btn" @click="emit('deselectAll')">取消</button>
      </div>
    </div>
    <div v-if="files.length === 0" class="file-empty">
      {{ emptyText }}
    </div>
    <div
      v-for="file in files"
      :key="file.path"
      class="file-item"
      :class="{ selected: isSelected(file.path), [file.changeType]: true }"
      @click="toggleSelect(file.path)"
    >
      <div class="file-checkbox">
        <div v-if="isSelected(file.path)" class="checkbox-checked">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </div>
      </div>
      <div class="file-icon" :class="file.changeType" v-html="getChangeIcon(file.changeType)"></div>
      <div class="file-content">
        <div class="file-name">{{ getFileName(file.path) }}</div>
        <div class="file-path">{{ getFileDir(file.path) }}</div>
      </div>
      <div class="file-badge" :class="file.changeType">{{ file.changeType }}</div>
    </div>
  </div>
</template>

<style scoped>
.file-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.file-list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2) var(--space-3);
}

.file-count {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.file-list-actions {
  display: flex;
  gap: var(--space-2);
}

.action-btn {
  font-size: var(--font-size-xs);
  color: var(--color-primary);
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 0;
}

.action-btn:hover {
  text-decoration: underline;
}

.file-empty {
  padding: var(--space-8);
  text-align: center;
  color: var(--color-text-muted);
  font-size: var(--font-size-sm);
}

.file-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  background: var(--color-bg-surface);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.file-item:hover {
  background: var(--color-bg-hover);
  border-color: var(--color-border);
}

.file-item.selected {
  background: var(--color-primary-light);
  border-color: var(--color-primary);
}

.file-checkbox {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.checkbox-checked {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-primary);
  border-radius: var(--radius-sm);
  color: white;
}

.checkbox-checked svg {
  width: 10px;
  height: 10px;
}

.file-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.file-icon :deep(svg) {
  width: 100%;
  height: 100%;
}

.file-icon.added { color: var(--color-success); }
.file-icon.modified { color: var(--color-warning); }
.file-icon.deleted { color: var(--color-error); }
.file-icon.renamed { color: var(--color-info); }
.file-icon.copied { color: var(--color-info); }

.file-content {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-size: var(--font-size-sm);
  color: var(--color-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-path {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-badge {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  text-transform: uppercase;
  flex-shrink: 0;
}

.file-badge.added {
  background: var(--color-success-light);
  color: var(--color-success);
}

.file-badge.modified {
  background: var(--color-warning-light);
  color: var(--color-warning);
}

.file-badge.deleted {
  background: var(--color-error-light);
  color: var(--color-error);
}

.file-badge.renamed,
.file-badge.copied {
  background: var(--color-info-light);
  color: var(--color-info);
}
</style>
