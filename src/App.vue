<script setup lang="ts">
import { RouterView } from "vue-router";
import AppHeader from "@/components/common/AppHeader.vue";
</script>

<template>
  <div class="app-container">
    <AppHeader />
    <main class="main-content">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-color);
}

.main-content {
  flex: 1;
  overflow: auto;
  padding: 20px;
}
</style>
