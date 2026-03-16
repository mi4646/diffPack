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
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="visible" class="dialog-overlay" @click.self="emit('close')">
        <div class="dialog" :class="type">
          <div class="dialog-header">
            <span class="dialog-icon">
              <span v-if="type === 'success'">✓</span>
              <span v-else-if="type === 'error'">✗</span>
              <span v-else-if="type === 'warning'">⚠</span>
              <span v-else>ℹ</span>
            </span>
            <span class="dialog-title">{{ title }}</span>
            <button class="dialog-close" @click="emit('close')">×</button>
          </div>
          <div class="dialog-body">
            <slot>
              <p v-if="message">{{ message }}</p>
            </slot>
          </div>
          <div class="dialog-footer">
            <button
              v-if="showCancel"
              class="dialog-btn cancel-btn"
              @click="emit('cancel'); emit('close')"
            >
              {{ cancelText }}
            </button>
            <button
              class="dialog-btn confirm-btn"
              :class="type"
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
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.dialog {
  background: var(--card-bg, #fff);
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  min-width: 360px;
  max-width: 520px;
  width: 90%;
  overflow: hidden;
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.dialog-icon {
  font-size: 20px;
  flex-shrink: 0;
  font-weight: bold;
}

.dialog.success .dialog-icon { color: var(--success-color); }
.dialog.error .dialog-icon   { color: var(--error-color); }
.dialog.warning .dialog-icon { color: #faad14; }
.dialog.info .dialog-icon    { color: var(--primary-color); }

.dialog-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-color);
  flex: 1;
}

.dialog-close {
  background: none;
  border: none;
  font-size: 20px;
  line-height: 1;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0 4px;
  margin: 0;
  min-width: unset;
  height: auto;
}

.dialog-close:hover {
  color: var(--text-color);
  background: none;
}

.dialog-body {
  padding: 16px 20px;
}

.dialog-body p {
  margin: 0;
  font-size: 14px;
  color: var(--text-color);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-all;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 20px;
  border-top: 1px solid var(--border-color);
}

.dialog-btn {
  min-width: 80px;
  padding: 6px 20px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.cancel-btn {
  background: var(--bg-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
}

.cancel-btn:hover {
  background: var(--border-color);
}

.confirm-btn {
  color: #fff;
}

.confirm-btn.success { background: var(--success-color); }
.confirm-btn.success:hover { opacity: 0.85; }

.confirm-btn.error   { background: var(--error-color); }
.confirm-btn.error:hover { opacity: 0.85; }

.confirm-btn.warning { background: #faad14; }
.confirm-btn.warning:hover { opacity: 0.85; }

.confirm-btn.info    { background: var(--primary-color); }
.confirm-btn.info:hover { opacity: 0.85; }

/* 过渡动画 */
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.2s ease;
}

.dialog-fade-enter-active .dialog,
.dialog-fade-leave-active .dialog {
  transition: transform 0.2s ease;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}

.dialog-fade-enter-from .dialog {
  transform: scale(0.9);
}

.dialog-fade-leave-to .dialog {
  transform: scale(0.9);
}
</style>
