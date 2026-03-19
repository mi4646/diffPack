<script setup lang="ts">
withDefaults(
  defineProps<{
    visible: boolean;
    title: string;
    message?: string;
    type?: "success" | "error" | "warning" | "info";
    confirmText?: string;
    cancelText?: string;
    showCancel?: boolean;
  }>(),
  {
    type: "info",
    confirmText: "确定",
    cancelText: "取消",
    showCancel: false,
  }
);

const emit = defineEmits<{
  close: [];
  confirm: [];
  cancel: [];
}>();

// SVG icons for different types
const icons = {
  success: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>`,
  error: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>`,
  warning: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>`,
  info: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>`,
};
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="visible" class="dialog-overlay" @click.self="emit('close')">
        <div class="dialog" :class="type" role="dialog" aria-modal="true">
          <div class="dialog-header">
            <div class="dialog-icon" :class="type" v-html="icons[type]"></div>
            <h3 class="dialog-title">{{ title }}</h3>
            <button class="dialog-close" @click="emit('close')" aria-label="关闭">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>

          <div class="dialog-body">
            <slot>
              <p v-if="message">{{ message }}</p>
            </slot>
          </div>

          <div class="dialog-footer">
            <button
              v-if="showCancel"
              class="btn btn-secondary"
              @click="emit('cancel'); emit('close')"
            >
              {{ cancelText }}
            </button>
            <button
              class="btn"
              :class="type === 'error' ? 'btn-danger' : 'btn-primary'"
              @click="emit('confirm'); emit('close')"
            >
              {{ confirmText }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  padding: var(--space-4);
}

.dialog {
  background: var(--color-bg-surface);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  min-width: 360px;
  max-width: 480px;
  width: 100%;
  overflow: hidden;
}

/* Header */
.dialog-header {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  padding: var(--space-4) var(--space-5);
  border-bottom: 1px solid var(--color-border-light);
}

.dialog-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.dialog-icon :deep(svg) {
  width: 100%;
  height: 100%;
}

.dialog-icon.success { color: var(--color-success); }
.dialog-icon.error { color: var(--color-error); }
.dialog-icon.warning { color: var(--color-warning); }
.dialog-icon.info { color: var(--color-info); }

.dialog-title {
  flex: 1;
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  margin: 0;
  line-height: var(--line-height-tight);
}

.dialog-close {
  width: 28px;
  height: 28px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  cursor: pointer;
  color: var(--color-text-muted);
  border-radius: var(--radius-md);
  transition: all var(--transition-normal);
}

.dialog-close:hover {
  background: var(--color-bg-hover);
  color: var(--color-text-primary);
}

.dialog-close svg {
  width: 16px;
  height: 16px;
}

/* Body */
.dialog-body {
  padding: var(--space-5);
}

.dialog-body p {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  line-height: var(--line-height-relaxed);
  white-space: pre-wrap;
  word-break: break-word;
}

/* Footer */
.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-5);
  border-top: 1px solid var(--color-border-light);
  background: var(--color-bg-elevated);
}

/* Buttons */
.btn {
  min-width: 80px;
  padding: var(--space-2) var(--space-4);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-normal);
}

.btn-primary {
  background: var(--color-primary);
  color: var(--color-text-inverse);
  border: none;
}

.btn-primary:hover {
  background: var(--color-primary-hover);
}

.btn-danger {
  background: var(--color-error);
  color: var(--color-text-inverse);
  border: none;
}

.btn-danger:hover {
  background: #b71c1c;
}

.btn-secondary {
  background: var(--color-bg-surface);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
}

.btn-secondary:hover {
  background: var(--color-bg-hover);
  border-color: var(--color-text-muted);
}

/* Transitions */
.dialog-enter-active,
.dialog-leave-active {
  transition: opacity 150ms ease;
}

.dialog-enter-active .dialog,
.dialog-leave-active .dialog {
  transition: transform 150ms ease, opacity 150ms ease;
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-from .dialog,
.dialog-leave-to .dialog {
  transform: scale(0.95);
  opacity: 0;
}
</style>
