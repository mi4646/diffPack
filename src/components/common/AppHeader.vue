<script setup lang="ts">
import { RouterLink, useRoute } from "vue-router";

const route = useRoute();

const navItems = [
  { path: "/", label: "本地分析", icon: "📁" },
  { path: "/remote", label: "远程分析", icon: "🌐" },
  { path: "/settings", label: "设置", icon: "⚙️" },
];
</script>

<template>
  <header class="app-header">
    <div class="logo">
      <span class="logo-icon">📦</span>
      <span class="logo-text">DiffPack</span>
    </div>
    <nav class="nav-menu">
      <RouterLink
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        class="nav-item"
        :class="{ active: route.path === item.path }"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span>{{ item.label }}</span>
      </RouterLink>
    </nav>
  </header>
</template>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  background: var(--card-bg);
  border-bottom: 1px solid var(--border-color);
  box-shadow: var(--shadow);
}

.logo {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-icon {
  font-size: 24px;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  color: var(--primary-color);
}

.nav-menu {
  display: flex;
  gap: 8px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 6px;
  text-decoration: none;
  color: var(--text-secondary);
  transition: all 0.2s;
}

.nav-item:hover {
  background: var(--bg-color);
  color: var(--text-color);
}

.nav-item.active {
  background: var(--primary-color);
  color: white;
}

.nav-icon {
  font-size: 16px;
}
</style>
