<template>
  <div class="control-button">
    <el-button v-if="aboutVisible" class="control-el-button" link @click="aboutHandle">
      关于
    </el-button>
    <el-divider v-if="aboutVisible" direction="vertical" />
    <el-button
      v-if="minVisible"
      class="control-el-button"
      link
      @click="minHandle"
    >
      <i class="iconfont icon-window-minimize2"></i>
    </el-button>
    <el-button
      v-if="maxVisible"
      class="control-el-button"
      link
      @click="maxHandle"
    >
      <i v-if="isMaximized" class="iconfont icon-window-restore"></i>
      <i v-else class="iconfont icon-window-maximize"></i>
    </el-button>
    <el-button class="control-el-button close" link @click="closeHandle">
      <i class="iconfont icon-window-close"></i>
    </el-button>
  </div>
</template>
<script setup>
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { message, confirm } from '@tauri-apps/plugin-dialog';
  import { getName,getVersion,getTauriVersion } from '@tauri-apps/api/app';
  import { listen } from '@tauri-apps/api/event'
  import { onMounted, ref } from 'vue'
  const appWindow = getCurrentWebviewWindow()

  const props = defineProps({
    minVisible: {
      type: Boolean,
      default: () => true,
    },
    maxVisible: {
      type: Boolean,
      default: () => true,
    },
    aboutVisible: {
      type: Boolean,
      default: () => false,
    },
  })
  const isMaximized = ref(false)

  //监视窗体最大化和还原状态，修改对应图标
  listen('tauri://resize', async() => {
    isMaximized.value = await appWindow.isMaximized()
  })

  const minHandle = async () => {
    await appWindow.minimize()
  }
  const maxHandle = async () => {
    await appWindow.toggleMaximize()
  }
  const closeHandle = async () => {
    if(appWindow.label == 'main'){
      const confirmation = await confirm(
        '确定要退出程序吗？',
        { title: '退出', kind: 'warning' }
      );
      if(confirmation)  {
        appWindow.close()
      }
    }
    else {
      appWindow.close()
    }
  }

  const aboutHandle = async () => {
    const aboutMessage =
      `名称：${await getName()}              \r\n` +
      `版本：${await getVersion()}              \r\n` +
      `内核：${await getTauriVersion()}         \r\n` 
    await message(aboutMessage)
  }

  onMounted(async () => {
    isMaximized.value = await appWindow.isMaximized()
  })
</script>
<style lang="scss" scoped>
  .control-button {
    position: absolute;
    right: 1px;
    top: 5px;
    color: #909399;
    .el-button {
      background-color: unset;
    }

    .control-el-button {
      color: #909399;
      padding: 5px;
      &:hover {
        background-color: rgba(129, 127, 127, 0.5);
        color: #ffffff;
      }
      &.close:hover {
        background-color: #f56c6c;
        color: #ffffff;
      }
    }
  }
</style>
