<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useSshStore } from "@/stores";
import AppDialog from "@/components/common/AppDialog.vue";
import type { SshConfig, AuthMethod } from "@/types";

const sshStore = useSshStore();

// 使用store里的临时配置，避免组件销毁后值丢失
const host = ref(sshStore.tempConfig.host || "");
const port = ref(sshStore.tempConfig.port || 22);
const username = ref(sshStore.tempConfig.username || "");
const authType = ref<"password" | "keyFile" | "sshAgent">(
  (sshStore.tempConfig.authMethod?.type as any) || "password"
);
const password = ref(
  sshStore.tempConfig.authMethod?.type === "password"
    ? sshStore.tempConfig.authMethod.password || ""
    : ""
);
const keyPath = ref(
  sshStore.tempConfig.authMethod?.type === "keyFile"
    ? sshStore.tempConfig.authMethod.keyPath || ""
    : ""
);
const passphrase = ref(
  sshStore.tempConfig.authMethod?.type === "keyFile"
    ? sshStore.tempConfig.authMethod.passphrase || ""
    : ""
);

// 监听配置变化，同步到store
watch([host, port, username, authType, password, keyPath, passphrase], () => {
  let authMethod: any;
  switch (authType.value) {
    case "password":
      authMethod = { type: "password", password: password.value };
      break;
    case "keyFile":
      authMethod = { type: "keyFile", keyPath: keyPath.value, passphrase: passphrase.value || undefined };
      break;
    case "sshAgent":
      authMethod = { type: "sshAgent" };
      break;
  }

  sshStore.tempConfig = {
    host: host.value,
    port: port.value,
    username: username.value,
    authMethod,
  };
}, { deep: true });

const isConnecting = ref(false);

// 弹框状态
const showDialog = ref(false);
const dialogTitle = ref("");
const dialogMessage = ref("");
const dialogType = ref<"success" | "error" | "warning" | "info">("info");

onMounted(() => {
  sshStore.loadSshConfig();
  sshStore.loadSavedHosts();
});

function getConfig(): SshConfig {
  let authMethod: AuthMethod;
  switch (authType.value) {
    case "password":
      authMethod = { type: "password", password: password.value };
      break;
    case "keyFile":
      authMethod = { type: "keyFile", keyPath: keyPath.value, passphrase: passphrase.value || undefined };
      break;
    case "sshAgent":
      authMethod = { type: "sshAgent" };
      break;
  }

  return {
    host: host.value,
    port: port.value,
    username: username.value,
    authMethod,
  };
}

async function testConnection() {
  isConnecting.value = true;
  try {
    const status = await sshStore.testConnection(getConfig());
    dialogTitle.value = status.connected ? "连接成功" : "连接失败";
    dialogMessage.value = status.connected
      ? `连接成功: ${status.serverInfo}`
      : status.error || "连接失败";
    dialogType.value = status.connected ? "success" : "error";
    showDialog.value = true;
  } catch (e) {
    dialogTitle.value = "连接失败";
    dialogMessage.value = e instanceof Error ? e.message : String(e);
    dialogType.value = "error";
    showDialog.value = true;
  } finally {
    isConnecting.value = false;
  }
}

async function connect() {
  isConnecting.value = true;
  try {
    await sshStore.connect(getConfig());
  } catch (e) {
    dialogTitle.value = "连接失败";
    dialogMessage.value = e instanceof Error ? e.message : String(e);
    dialogType.value = "error";
    showDialog.value = true;
  } finally {
    isConnecting.value = false;
  }
}

function loadConfigEntry(entry: { hostname?: string; port?: number; user?: string; identityFile?: string }) {
  host.value = entry.hostname || "";
  port.value = entry.port || 22;
  username.value = entry.user || "";
  if (entry.identityFile) {
    authType.value = "keyFile";
    keyPath.value = entry.identityFile;
  }
}

async function browseKeyFile() {
  const { open } = await import("@tauri-apps/plugin-dialog");
  const selected = await open({
    multiple: false,
    title: "选择 SSH 私钥文件",
  });
  if (selected && typeof selected === "string") {
    keyPath.value = selected;
  }
}
</script>

<template>
  <div class="ssh-config-form">
    <!-- SSH Config 预设 -->
    <div v-if="sshStore.sshConfigEntries.length > 0" class="config-presets">
      <label>从 SSH Config 加载：</label>
      <select @change="(e) => loadConfigEntry(sshStore.sshConfigEntries[+(e.target as HTMLSelectElement).value])">
        <option value="">选择预设...</option>
        <option
          v-for="(entry, index) in sshStore.sshConfigEntries"
          :key="entry.host"
          :value="index"
        >
          {{ entry.host }} {{ entry.hostname ? `(${entry.hostname})` : "" }}
        </option>
      </select>
    </div>

    <!-- 连接信息 -->
    <div class="form-row">
      <label>主机：</label>
      <input type="text" v-model="host" placeholder="hostname or IP" />
    </div>

    <div class="form-row">
      <label>端口：</label>
      <input type="number" v-model="port" min="1" max="65535" />
    </div>

    <div class="form-row">
      <label>用户名：</label>
      <input type="text" v-model="username" placeholder="username" />
    </div>

    <!-- 认证方式 -->
    <div class="auth-section">
      <label>认证方式：</label>
      <div class="auth-options">
        <label>
          <input type="radio" v-model="authType" value="password" />
          密码
        </label>
        <label>
          <input type="radio" v-model="authType" value="keyFile" />
          密钥文件
        </label>
        <label>
          <input type="radio" v-model="authType" value="sshAgent" />
          SSH Agent
        </label>
      </div>

      <!-- 密码认证 -->
      <div v-if="authType === 'password'" class="auth-input">
        <input type="password" v-model="password" placeholder="输入密码" />
      </div>

      <!-- 密钥文件认证 -->
      <div v-if="authType === 'keyFile'" class="auth-input">
        <div class="key-file-row">
          <input type="text" v-model="keyPath" placeholder="私钥文件路径" />
          <button @click="browseKeyFile">浏览</button>
        </div>
        <input type="password" v-model="passphrase" placeholder="密钥密码（可选）" />
      </div>
    </div>

    <!-- 操作按鈕 -->
    <div class="form-actions">
      <button @click="testConnection" :disabled="isConnecting || !host || !username">
        测试连接
      </button>
      <button @click="connect" :disabled="isConnecting || !host || !username">
        连接
      </button>
    </div>
  </div>

  <AppDialog
    :visible="showDialog"
    :title="dialogTitle"
    :message="dialogMessage"
    :type="dialogType"
    @close="showDialog = false"
  />
</template>

<style scoped>
.ssh-config-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.config-presets {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color);
}

.config-presets select {
  flex: 1;
}

.form-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.form-row label {
  min-width: 80px;
}

.form-row input {
  flex: 1;
}

.auth-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.auth-options {
  display: flex;
  gap: 16px;
}

.auth-options label {
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
}

.auth-input {
  margin-top: 8px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.key-file-row {
  display: flex;
  gap: 8px;
}

.key-file-row input {
  flex: 1;
}

.form-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}

.form-actions button {
  flex: 1;
}
</style>
