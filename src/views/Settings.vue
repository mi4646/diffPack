<script setup lang="ts">
import { ref, onMounted } from "vue";
import { usePackStore } from "@/stores";

const packStore = usePackStore();

const defaultFormat = ref<"zip" | "tarGz">("zip");
const defaultOutputPath = ref("");

onMounted(() => {
  defaultFormat.value = packStore.options.format;
  defaultOutputPath.value = packStore.options.outputPath;
});

function saveSettings() {
  packStore.updateOptions({
    format: defaultFormat.value,
    outputPath: defaultOutputPath.value,
  });
  // 保存到本地存储
  localStorage.setItem("settings", JSON.stringify({
    defaultFormat: defaultFormat.value,
    defaultOutputPath: defaultOutputPath.value,
  }));
}

async function browseOutputPath() {
  const path = await packStore.selectOutputDirectory();
  if (path) {
    defaultOutputPath.value = path;
  }
}

function clearHistory() {
  packStore.clearHistory();
}
</script>

<template>
  <div class="settings-page">
    <section class="card">
      <h3>默认设置</h3>
      
      <div class="setting-row">
        <label>默认打包格式：</label>
        <select v-model="defaultFormat">
          <option value="zip">ZIP</option>
          <option value="tarGz">tar.gz</option>
        </select>
      </div>

      <div class="setting-row">
        <label>默认输出目录：</label>
        <input type="text" v-model="defaultOutputPath" placeholder="选择默认输出目录..." />
        <button @click="browseOutputPath">浏览</button>
      </div>

      <button @click="saveSettings" class="save-btn">保存设置</button>
    </section>

    <section class="card">
      <h3>历史记录</h3>
      <p class="history-count">共 {{ packStore.history.length }} 条打包记录</p>
      <button @click="clearHistory" class="clear-btn">清除历史记录</button>
    </section>

    <section class="card">
      <h3>关于</h3>
      <p><strong>DiffPack</strong> - Git 差异项目版本打包器</p>
      <p class="version">版本: 0.1.0</p>
      <p class="tech-stack">技术栈: Tauri 2.x + Vue 3 + TypeScript</p>
    </section>
  </div>
</template>

<style scoped>
.settings-page {
  display: flex;
  flex-direction: column;
  gap: 20px;
  max-width: 600px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.setting-row label {
  min-width: 120px;
}

.setting-row input {
  flex: 1;
}

.save-btn {
  margin-top: 8px;
}

.history-count {
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.clear-btn {
  background: var(--error-color);
}

.clear-btn:hover {
  background: #ff4d4f;
}

.version, .tech-stack {
  color: var(--text-secondary);
  font-size: 13px;
  margin-top: 8px;
}
</style>
