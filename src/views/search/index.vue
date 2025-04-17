<template>
  <div class="settings-container">
    <h2 class="settings-title">设置</h2>
    
    <div class="settings-section">
      <h3>外观设置</h3>
      <div class="settings-item">
        <span>主题</span>
        <div class="theme-options">
          <button 
            class="theme-btn" 
            :class="{ active: theme === 'light' }"
            @click="setTheme('light')"
          >
            浅色
          </button>
          <button 
            class="theme-btn" 
            :class="{ active: theme === 'dark' }"
            @click="setTheme('dark')"
          >
            深色
          </button>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h3>语言设置</h3>
      <div class="settings-item">
        <span>界面语言</span>
        <select v-model="language" class="language-select">
          <option value="zh">中文</option>
          <option value="en">English</option>
        </select>
      </div>
    </div>

    <div class="settings-section">
      <h3>其他设置</h3>
      <div class="settings-item">
        <span>开机自启动</span>
        <el-switch v-model="autoStart" />
      </div>
      <div class="settings-item">
        <span>显示托盘图标</span>
        <el-switch v-model="showTray" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { ElSwitch } from 'element-plus'

const theme = ref('light')
const language = ref('zh')
const autoStart = ref(false)
const showTray = ref(true)

const setTheme = (newTheme) => {
  theme.value = newTheme
  // TODO: 实现主题切换逻辑
}
</script>

<style scoped>
.settings-container {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.settings-title {
  margin-bottom: 30px;
  color: var(--el-text-color-primary);
}

.settings-section {
  margin-bottom: 30px;
  background: var(--el-bg-color);
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.settings-section h3 {
  margin-bottom: 20px;
  color: var(--el-text-color-primary);
}

.settings-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid var(--el-border-color-light);
}

.settings-item:last-child {
  border-bottom: none;
}

.theme-options {
  display: flex;
  gap: 10px;
}

.theme-btn {
  padding: 8px 16px;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  background: var(--el-bg-color);
  cursor: pointer;
  transition: all 0.3s;
}

.theme-btn.active {
  background: var(--el-color-primary);
  color: white;
  border-color: var(--el-color-primary);
}

.language-select {
  padding: 8px;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  background: var(--el-bg-color);
}
</style>