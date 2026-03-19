<script setup lang="ts">
import { computed } from "vue";
import { RouterView } from "vue-router";
import AppHeader from "@/components/common/AppHeader.vue";
import AppNotification from "@/components/common/AppNotification.vue";
import LoadingOverlay from "@/components/common/LoadingOverlay.vue";
import { useGitStore } from "@/stores";

const gitStore = useGitStore();

const isGlobalLoading = computed(() => gitStore.isLoading);
</script>

<template>
  <div class="app-container">
    <AppHeader />
    <main class="main-content">
      <RouterView />
    </main>
    <AppNotification />
    <LoadingOverlay :visible="isGlobalLoading" text="处理中..." />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--color-bg-base);
}

.main-content {
  flex: 1;
  overflow: auto;
  padding: var(--space-5);
}
</style>
