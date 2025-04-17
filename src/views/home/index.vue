<template>
  <div class="home-container">
    <main class="main-content">
      <h1 class="welcome-title">欢迎使用 Tauri Vue App</h1>
      <div class="feature-cards">
        <div class="card">
          <h3>跨平台</h3>
          <p>基于 Tauri 构建，支持 Windows、macOS 和 Linux</p>
        </div>
        <div class="card">
          <h3>高性能</h3>
          <p>使用 Rust 后端，提供卓越的性能表现</p>
        </div>
        <div class="card">
          <h3>现代化</h3>
          <p>采用 Vue 3 和最新的前端技术栈</p>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref } from 'vue'
</script>

<style scoped>
.home-container {
  min-height: 100vh;
  background-color: #fff;
}

.header {
  background-color: #ffffff;
  padding: 1rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  color: #2c3e50;
}

.nav {
  display: flex;
  gap: 1.5rem;
}

.nav-item {
  text-decoration: none;
  color: #2c3e50;
  font-weight: 500;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  transition: background-color 0.3s;
}

.nav-item:hover {
  background-color: #f0f0f0;
}

.main-content {
  max-width: 1200px;
  margin: 2rem auto;
  padding: 0 2rem;
  /* background-color: #fff; */
}

.welcome-title {
  text-align: center;
  color: #2c3e50;
  margin-bottom: 3rem;
}

.feature-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
}

.card {
  background-color: #ffffff;
  padding: 2rem;
  border-radius: 8px;
  /* box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); */
  transition: transform 0.3s;
}

.card:hover {
  transform: translateY(-5px);
}

.card h3 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.card p {
  color: #666;
  line-height: 1.6;
}
</style>