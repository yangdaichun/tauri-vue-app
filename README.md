# Tauri + Vue 3 + Element Plus 版本

该源码可帮助你快速开始一个Tauri + Vue 3的项目，采用Vue 3 `<script setup>`风格

Tauri地址：[https://tauri.app](https://tauri.app/)

Vue地址：[https://cn.vuejs.org](https://cn.vuejs.org/)

Element Plus地址：[https://element-plus.gitee.io](https://element-plus.gitee.io/)

码云地址：[https://gitee.com/agafonov/tauri-vue-app](https://gitee.com/agafonov/tauri-vue-app/)

GitHub地址：[https://github.com/yangdaichun/tauri-vue-app](https://github.com/yangdaichun/tauri-vue-app/)

程序效果

![image](src/assets/app.png)

## 环境要求

- Node.js (推荐使用 LTS 版本)
- Rust (推荐使用最新稳定版)
- 系统依赖：
  - macOS: Xcode 命令行工具
  - Windows: Microsoft Visual Studio C++ 构建工具
  - Linux: 请参考 [Tauri 官方文档](https://tauri.app/v1/guides/getting-started/prerequisites)

## 安装步骤

1. 克隆项目
```bash
git clone https://github.com/yangdaichun/tauri-vue-app.git
cd tauri-vue-app
```

2. 安装依赖
```bash
# 安装前端依赖
npm install

# 安装 Rust 依赖
cargo install tauri-cli
```

3. 开发模式运行
```bash
npm run tauri dev
```

4. 构建生产版本
```bash
npm run tauri build
```

## 项目结构

- `src/` - Vue 前端源代码
- `src-tauri/` - Tauri 后端源代码
- `public/` - 静态资源文件
