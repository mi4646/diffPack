<script setup lang="ts">
import { ref, watch } from "vue";

const props = withDefaults(
  defineProps<{
    modelValue: string;
    placeholder?: string;
    debounce?: number;
  }>(),
  {
    placeholder: "搜索...",
    debounce: 300,
  }
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const inputValue = ref(props.modelValue);
let debounceTimer: ReturnType<typeof setTimeout> | null = null;

watch(
  () => props.modelValue,
  (val) => {
    inputValue.value = val;
  }
);

function onInput(event: Event) {
  const value = (event.target as HTMLInputElement).value;
  inputValue.value = value;

  if (debounceTimer) {
    clearTimeout(debounceTimer);
  }

  debounceTimer = setTimeout(() => {
    emit("update:modelValue", value);
  }, props.debounce);
}

function clear() {
  inputValue.value = "";
  emit("update:modelValue", "");
}
</script>

<template>
  <div class="search-input">
    <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="11" cy="11" r="8"/>
      <path d="M21 21l-4.35-4.35"/>
    </svg>
    <input
      type="text"
      class="search-field"
      :value="inputValue"
      :placeholder="placeholder"
      @input="onInput"
    />
    <button v-if="inputValue" class="search-clear" @click="clear" aria-label="清除搜索">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.search-input {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
}

.search-icon {
  position: absolute;
  left: var(--space-3);
  width: 16px;
  height: 16px;
  color: var(--color-text-muted);
  pointer-events: none;
}

.search-field {
  width: 100%;
  padding: var(--space-2) var(--space-3);
  padding-left: calc(var(--space-3) + 16px + var(--space-2));
  padding-right: calc(var(--space-3) + 20px);
  font-size: var(--font-size-sm);
  background: var(--color-bg-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text-primary);
  transition: all var(--transition-normal);
}

.search-field::placeholder {
  color: var(--color-text-muted);
}

.search-field:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px var(--color-primary-alpha);
}

.search-clear {
  position: absolute;
  right: var(--space-2);
  width: 20px;
  height: 20px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  cursor: pointer;
  color: var(--color-text-muted);
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.search-clear:hover {
  background: var(--color-bg-hover);
  color: var(--color-text-primary);
}

.search-clear svg {
  width: 14px;
  height: 14px;
}
</style>
