<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();

function navigateToLocal() {
  router.push("/local");
}

function navigateToRemote() {
  router.push("/remote");
}
</script>

<template>
  <div class="home-container">
    <div class="home-content">
      <div class="home-header">
        <div class="home-logo">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
            <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
            <line x1="12" y1="22.08" x2="12" y2="12"/>
          </svg>
        </div>
        <h1 class="home-title">DiffPack</h1>
        <p class="home-subtitle">选择仓库类型开始分析</p>
      </div>

      <div class="option-cards">
        <button class="option-card" @click="navigateToLocal">
          <div class="card-icon local-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </div>
          <h2 class="card-title">本地仓库</h2>
          <p class="card-description">选择本地 Git 仓库进行分析</p>
        </button>

        <button class="option-card" @click="navigateToRemote">
          <div class="card-icon remote-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z"/>
            </svg>
          </div>
          <h2 class="card-title">远程仓库</h2>
          <p class="card-description">克隆并分析远程 Git 仓库</p>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - var(--header-height));
  padding: var(--space-6);
  background-color: var(--color-bg-primary);
}

.home-content {
  max-width: 640px;
  width: 100%;
  text-align: center;
}

.home-header {
  margin-bottom: var(--space-8);
}

.home-logo {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 72px;
  height: 72px;
  margin-bottom: var(--space-4);
  border-radius: var(--radius-lg);
  background-color: var(--color-primary-subtle);
}

.home-logo svg {
  width: 40px;
  height: 40px;
  color: var(--color-primary);
}

.home-title {
  font-size: var(--font-size-3xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0 0 var(--space-2);
  letter-spacing: -0.02em;
}

.home-subtitle {
  font-size: var(--font-size-lg);
  color: var(--color-text-secondary);
  margin: 0;
}

.option-cards {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--space-4);
}

.option-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--space-6);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-lg);
  background-color: var(--color-bg-surface);
  cursor: pointer;
  transition: all var(--transition-normal);
}

.option-card:hover {
  border-color: var(--color-primary);
  background-color: var(--color-bg-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.card-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 56px;
  height: 56px;
  margin-bottom: var(--space-4);
  border-radius: var(--radius-md);
}

.card-icon svg {
  width: 28px;
  height: 28px;
}

.local-icon {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.remote-icon {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.card-title {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  margin: 0 0 var(--space-2);
}

.card-description {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.5;
}

@media (max-width: 640px) {
  .option-cards {
    grid-template-columns: 1fr;
  }
}
</style>
