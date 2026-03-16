<script setup lang="ts">
import { useSshStore } from "@/stores";

const sshStore = useSshStore();
</script>

<template>
  <div class="connection-status" :class="{ connected: sshStore.isConnected, disconnected: !sshStore.isConnected }">
    <div class="status-indicator">
      <span class="status-dot"></span>
      <span class="status-text">
        {{ sshStore.isConnected ? "已连接" : "未连接" }}
      </span>
    </div>
    <div v-if="sshStore.isConnected && sshStore.currentConfig" class="connection-info">
      <span>{{ sshStore.currentConfig.username }}@{{ sshStore.currentConfig.host }}:{{ sshStore.currentConfig.port }}</span>
    </div>
    <div v-if="sshStore.connectionStatus.error" class="error-info">
      {{ sshStore.connectionStatus.error }}
    </div>
    <button v-if="sshStore.isConnected" @click="sshStore.disconnect" class="disconnect-btn">
      断开连接
    </button>
  </div>
</template>

<style scoped>
.connection-status {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-radius: 6px;
}

.connection-status.connected {
  background: #f6ffed;
  border: 1px solid #b7eb8f;
}

.connection-status.disconnected {
  background: var(--bg-color);
  border: 1px solid var(--border-color);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.connected .status-dot {
  background: var(--success-color);
}

.disconnected .status-dot {
  background: #d9d9d9;
}

.status-text {
  font-weight: 500;
}

.connection-info {
  font-family: monospace;
  font-size: 13px;
  color: var(--text-secondary);
}

.error-info {
  color: var(--error-color);
  font-size: 13px;
}

.disconnect-btn {
  margin-left: auto;
  background: var(--error-color);
  padding: 4px 12px;
  font-size: 12px;
}

.disconnect-btn:hover {
  background: #ff4d4f;
}
</style>
