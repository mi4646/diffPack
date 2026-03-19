<script setup lang="ts">
import { usePackStore } from "@/stores";

const packStore = usePackStore();

defineEmits<{
  pack: [];
}>();
</script>

<template>
  <div class="pack-options">
    <div class="option-row">
      <label class="form-label">打包格式</label>
      <select v-model="packStore.options.format" class="input-field">
        <option value="zip">ZIP</option>
        <option value="tarGz">tar.gz</option>
      </select>
    </div>

    <div class="option-row">
      <label class="form-label">输出目录</label>
      <input
        type="text"
        v-model="packStore.options.outputPath"
        placeholder="选择输出目录..."
        class="input-field"
        readonly
      />
      <button class="btn-secondary" @click="packStore.selectOutputDirectory">
        <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
        </svg>
        浏览
      </button>
    </div>

    <div class="option-row checkbox">
      <label class="checkbox-label">
        <input type="checkbox" v-model="packStore.options.preserveStructure" class="checkbox-input" />
        <span class="checkbox-control"></span>
        <span class="checkbox-text">保持目录结构</span>
      </label>
    </div>

    <div class="option-row checkbox">
      <label class="checkbox-label">
        <input type="checkbox" v-model="packStore.options.includeDeleted" class="checkbox-input" />
        <span class="checkbox-control"></span>
        <span class="checkbox-text">包含已删除文件标记</span>
      </label>
    </div>

    <div v-if="!packStore.options.outputPath" class="warning-message">
      <svg class="warning-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
        <line x1="12" y1="9" x2="12" y2="13"/>
        <line x1="12" y1="17" x2="12.01" y2="17"/>
      </svg>
      请先选择输出目录
    </div>

    <button
      class="btn-primary pack-btn"
      @click="$emit('pack')"
      :disabled="packStore.isPacking || !packStore.options.outputPath"
    >
      <svg v-if="!packStore.isPacking" class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
        <polyline points="7 10 12 15 17 10"/>
        <line x1="12" y1="15" x2="12" y2="3"/>
      </svg>
      <svg v-else class="btn-icon spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="12" y1="2" x2="12" y2="6"/>
        <line x1="12" y1="18" x2="12" y2="22"/>
        <line x1="4.93" y1="4.93" x2="7.76" y2="7.76"/>
        <line x1="16.24" y1="16.24" x2="19.07" y2="19.07"/>
        <line x1="2" y1="12" x2="6" y2="12"/>
        <line x1="18" y1="12" x2="22" y2="12"/>
        <line x1="4.93" y1="19.07" x2="7.76" y2="16.24"/>
        <line x1="16.24" y1="7.76" x2="19.07" y2="4.93"/>
      </svg>
      {{ packStore.isPacking ? "打包中..." : "开始打包" }}
    </button>
  </div>
</template>

<style scoped>
.pack-options {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.option-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.option-row .form-label {
  min-width: 70px;
  margin: 0;
}

.option-row .input-field {
  flex: 1;
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  background: var(--color-bg-surface);
}

/* Checkbox styling */
.checkbox-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
}

.checkbox-input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.checkbox-control {
  width: 16px;
  height: 16px;
  border: 2px solid var(--color-border);
  border-radius: var(--radius-sm);
  position: relative;
  transition: all var(--transition-normal);
  flex-shrink: 0;
}

.checkbox-input:checked + .checkbox-control {
  background: var(--color-primary);
  border-color: var(--color-primary);
}

.checkbox-input:checked + .checkbox-control::after {
  content: '';
  position: absolute;
  left: 4px;
  top: 1px;
  width: 4px;
  height: 8px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.checkbox-input:focus + .checkbox-control {
  box-shadow: 0 0 0 2px var(--color-primary-light);
}

/* Warning message */
.warning-message {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--font-size-sm);
  color: var(--color-warning);
  padding: var(--space-2) var(--space-3);
  background: var(--color-warning-bg);
  border-radius: var(--radius-md);
}

.warning-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

/* Pack button */
.pack-btn {
  margin-top: var(--space-2);
  padding: var(--space-3) var(--space-5);
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
}

.btn-icon {
  width: 16px;
  height: 16px;
}

.btn-icon.spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
