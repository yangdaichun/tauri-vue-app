<template>
  <div class="control-button">
    <el-button v-if="aboutVisible" class="control-el-button" link @click="aboutHandle">
      关于
    </el-button>
  </div>
</template>
<script setup>
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { message } from '@tauri-apps/plugin-dialog';
  import { getName,getVersion,getTauriVersion } from '@tauri-apps/api/app';
  import { onMounted } from 'vue'
  const appWindow = getCurrentWebviewWindow()

  const props = defineProps({
    aboutVisible: {
      type: Boolean,
      default: () => false,
    },
  })

  const aboutHandle = async () => {
    const aboutMessage =
      `名称：${await getName()}              \r\n` +
      `版本：${await getVersion()}              \r\n` +
      `内核：${await getTauriVersion()}         \r\n` 
    await message(aboutMessage)
  }

  onMounted(async () => {
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
      margin-right: 5px;
      padding: 5px;
      &:hover {
        // background-color: rgba(129, 127, 127, 0.5);
        color: #000;
      }
    }
  }
</style>
