<script setup lang="ts">
import { useNotificationStore } from "@/stores/notificationStore";

const store = useNotificationStore();

const icons = {
  success: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>`,
  error: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>`,
  warning: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>`,
  info: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>`,
};

function close(id: number) {
  store.removeNotification(id);
}
</script>

<template>
  <Teleport to="body">
    <div class="notification-container">
      <TransitionGroup name="notification">
        <div
          v-for="notification in store.notifications"
          :key="notification.id"
          class="notification"
          :class="notification.type"
        >
          <div class="notification-icon" v-html="icons[notification.type]"></div>
          <div class="notification-content">
            <div class="notification-title">{{ notification.title }}</div>
            <div v-if="notification.message" class="notification-message">{{ notification.message }}</div>
          </div>
          <button class="notification-close" @click="close(notification.id)" aria-label="关闭">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<style scoped>
.notification-container {
  position: fixed;
  top: var(--space-4);
  right: var(--space-4);
  z-index: 10000;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  max-width: 400px;
  width: 100%;
  pointer-events: none;
}

.notification {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  background: var(--color-bg-surface);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  border-left: 4px solid;
  pointer-events: auto;
}

.notification.success {
  border-left-color: var(--color-success);
}

.notification.error {
  border-left-color: var(--color-error);
}

.notification.warning {
  border-left-color: var(--color-warning);
}

.notification.info {
  border-left-color: var(--color-info);
}

.notification-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.notification-icon :deep(svg) {
  width: 100%;
  height: 100%;
}

.notification.success .notification-icon { color: var(--color-success); }
.notification.error .notification-icon { color: var(--color-error); }
.notification.warning .notification-icon { color: var(--color-warning); }
.notification.info .notification-icon { color: var(--color-info); }

.notification-content {
  flex: 1;
  min-width: 0;
}

.notification-title {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
}

.notification-message {
  font-size: var(--font-size-xs);
  color: var(--color-text-secondary);
  margin-top: var(--space-1);
}

.notification-close {
  width: 20px;
  height: 20px;
  padding: 0;
  background: transparent;
  border: none;
  cursor: pointer;
  color: var(--color-text-muted);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  transition: all var(--transition-fast);
}

.notification-close:hover {
  background: var(--color-bg-hover);
  color: var(--color-text-primary);
}

.notification-close svg {
  width: 14px;
  height: 14px;
}

/* Transitions */
.notification-enter-active {
  transition: all 200ms ease-out;
}

.notification-leave-active {
  transition: all 150ms ease-in;
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(100%);
}

.notification-move {
  transition: transform 200ms ease;
}
</style>
