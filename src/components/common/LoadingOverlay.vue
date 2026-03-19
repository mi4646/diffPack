<script setup lang="ts">
withDefaults(
  defineProps<{
    visible: boolean;
    text?: string;
    blur?: boolean;
  }>(),
  {
    text: "加载中...",
    blur: false,
  }
);
</script>

<template>
  <Teleport to="body">
    <Transition name="loading">
      <div v-if="visible" class="loading-overlay" :class="{ blur }">
        <div class="loading-content">
          <div class="loading-spinner">
            <svg viewBox="0 0 50 50">
              <circle cx="25" cy="25" r="20" fill="none" stroke-width="4"></circle>
            </svg>
          </div>
          <p v-if="text" class="loading-text">{{ text }}</p>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.loading-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9998;
}

.loading-overlay.blur {
  backdrop-filter: blur(4px);
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-4);
}

.loading-spinner {
  width: 48px;
  height: 48px;
}

.loading-spinner svg {
  width: 100%;
  height: 100%;
  animation: rotate 1.4s linear infinite;
}

.loading-spinner circle {
  stroke: var(--color-primary);
  stroke-linecap: round;
  animation: dash 1.4s ease-in-out infinite;
}

.loading-text {
  font-size: var(--font-size-sm);
  color: var(--color-text-inverse);
  margin: 0;
  text-align: center;
}

@keyframes rotate {
  100% {
    transform: rotate(360deg);
  }
}

@keyframes dash {
  0% {
    stroke-dasharray: 1, 150;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: -35;
  }
  100% {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: -124;
  }
}

/* Transitions */
.loading-enter-active,
.loading-leave-active {
  transition: opacity 200ms ease;
}

.loading-enter-from,
.loading-leave-to {
  opacity: 0;
}
</style>
