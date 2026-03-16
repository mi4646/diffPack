<script setup lang="ts">
import { usePackStore } from "@/stores";

const packStore = usePackStore();
</script>

<template>
  <div class="pack-options">
    <div class="option-row">
      <label>打包格式：</label>
      <select v-model="packStore.options.format">
        <option value="zip">ZIP</option>
        <option value="tarGz">tar.gz</option>
      </select>
    </div>

    <div class="option-row">
      <label>输出目录：</label>
      <input
        type="text"
        v-model="packStore.options.outputPath"
        placeholder="选择输出目录..."
        readonly
      />
      <button @click="packStore.selectOutputDirectory">浏览</button>
    </div>

    <div class="option-row checkbox">
      <label>
        <input type="checkbox" v-model="packStore.options.preserveStructure" />
        保持目录结构
      </label>
    </div>

    <div class="option-row checkbox">
      <label>
        <input type="checkbox" v-model="packStore.options.includeDeleted" />
        包含已删除文件标记
      </label>
    </div>

    <div v-if="!packStore.options.outputPath" class="warn-message">
      请先选择输出目录
    </div>

    <button
      class="pack-btn"
      @click="$emit('pack')"
      :disabled="packStore.isPacking || !packStore.options.outputPath"
    >
      {{ packStore.isPacking ? "打包中..." : "开始打包" }}
    </button>
  </div>
</template>

<style scoped>
.pack-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.option-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.option-row label {
  min-width: 80px;
}

.option-row input[type="text"] {
  flex: 1;
}

.option-row.checkbox {
  margin-top: 4px;
}

.option-row.checkbox label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.pack-btn {
  margin-top: 12px;
  padding: 12px 24px;
  font-size: 16px;
  font-weight: 500;
}

.warn-message {
  font-size: 12px;
  color: #faad14;
  margin-top: 4px;
}
</style>
