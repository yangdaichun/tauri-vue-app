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
  import { appWindow } from '@tauri-apps/api/window';
  import { message } from '@tauri-apps/api/dialog';
  import { getName,getVersion,getTauriVersion } from '@tauri-apps/api/app';
  import { listen } from '@tauri-apps/api/event'
  import { onMounted, ref } from 'vue'

  const props=defineProps({
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
    //appWindow.close()
    // 采用CloseRequested 处理关闭提示消息，在自定义中调用close关闭窗体并没有触发CloseRequested事件，未确定原因
    // 暂时采用事件 方式实现
    if(appWindow.label == 'main'){
      appWindow.emit("app-exist",{})
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
