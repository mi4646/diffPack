<script setup lang="ts">
import type { CommitInfo } from "@/types";

const props = withDefaults(
  defineProps<{
    commits: CommitInfo[];
    selectedHashes: string[];
    emptyText?: string;
  }>(),
  {
    emptyText: "暂无提交记录",
  }
);

const emit = defineEmits<{
  select: [commit: CommitInfo];
  "update:selectedHashes": [hashes: string[]];
}>();

function isSelected(hash: string): boolean {
  return props.selectedHashes.includes(hash);
}

function toggleSelect(commit: CommitInfo) {
  emit("select", commit);
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));

  if (days === 0) {
    return "今天";
  } else if (days === 1) {
    return "昨天";
  } else if (days < 7) {
    return `${days}天前`;
  } else {
    return date.toLocaleDateString("zh-CN");
  }
}
</script>

<template>
  <div class="commit-list">
    <div v-if="commits.length === 0" class="commit-empty">
      {{ emptyText }}
    </div>
    <div
      v-for="commit in commits"
      :key="commit.hash"
      class="commit-item"
      :class="{ selected: isSelected(commit.hash) }"
      @click="toggleSelect(commit)"
    >
      <div class="commit-checkbox">
        <div v-if="isSelected(commit.hash)" class="checkbox-checked">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </div>
      </div>
      <div class="commit-content">
        <div class="commit-message">{{ commit.message }}</div>
        <div class="commit-meta">
          <span class="commit-hash">{{ commit.shortHash }}</span>
          <span class="commit-author">{{ commit.author }}</span>
          <span class="commit-date">{{ formatDate(commit.date) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.commit-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.commit-empty {
  padding: var(--space-8);
  text-align: center;
  color: var(--color-text-muted);
  font-size: var(--font-size-sm);
}

.commit-item {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  padding: var(--space-3);
  background: var(--color-bg-surface);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.commit-item:hover {
  background: var(--color-bg-hover);
  border-color: var(--color-border);
}

.commit-item.selected {
  background: var(--color-primary-light);
  border-color: var(--color-primary);
}

.commit-checkbox {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  margin-top: 2px;
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
  width: 12px;
  height: 12px;
}

.commit-content {
  flex: 1;
  min-width: 0;
}

.commit-message {
  font-size: var(--font-size-sm);
  color: var(--color-text-primary);
  line-height: var(--line-height-tight);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.commit-meta {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-top: var(--space-1);
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.commit-hash {
  font-family: var(--font-family-mono);
  background: var(--color-bg-base);
  padding: 1px 4px;
  border-radius: var(--radius-sm);
}

.commit-author {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100px;
}
</style>
