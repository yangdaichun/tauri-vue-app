<template>
  <el-aside width="65px" data-tauri-drag-region> 
    <el-avatar @click="avatarHandle">Tauri</el-avatar>
    <el-menu :collapse="true" router :default-active="menuData.defaultRouter">
      <el-menu-item :index="item.routerPath"  v-for="item in menuData.menuItem" :key="item.routerPath">
        <i :class="'iconfont ' + item.icon"></i>
        <template #title>{{ item.title }}</template>
      </el-menu-item>
    </el-menu>
    <el-button link class="menu-system" @click="systemHandle">
      <i class="iconfont icon-menu"></i>
    </el-button>
  </el-aside>
</template>
<script setup>
  import { ElMessage } from 'element-plus'
  const menuData = {
    defaultRouter: '/home',
    menuItem: [
     {
      title: '首页',
      routerPath: '/home',
      icon: 'icon-cloud',
     },
     {
      title: '查询',
      routerPath: '/search',
      icon: 'icon-config-icon',
     }
    ]
  }

  const avatarHandle = () => {
    ElMessage('点击了头像')
  }
  const systemHandle = () => {
    ElMessage('点击了系统管理按钮')
  }
</script>
<style lang="scss" scoped>
  .el-aside {
    background: #409EFF;
    .el-avatar {
      margin: 20px 10px;
      cursor: pointer;
      background-color: #55b1f9;
    }
    .el-menu {
      background: unset;
      border-right: unset;
      .el-menu-item {
        padding: 2px;
        color: #CDD0D6;
        &:hover {
          background: unset;
          color: #ffffff;
        }
        &.is-active {
          color: #ffffff;
        }
      }
    }

    .menu-system{
      color: #ffffff;
      position: absolute;
      bottom: 10px;
      height: 40px;
      left: 15px;
      &:hover {
        color: rgba(255, 255, 255, 1);
      }
    }

    i {
      font-size: 28px
    }
  }
</style>