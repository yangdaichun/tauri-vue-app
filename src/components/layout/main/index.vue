<template>
  <el-main>
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component
          :is="Component"
          v-if="$route.meta.keepAlive"
          :key="$route.name"
        />
      </keep-alive>
      <component
        :is="Component"
        v-if="!$route.meta.keepAlive"
        :key="$route.name"
      />
    </router-view>
  </el-main>
</template>
<script setup>

</script>
<style lang="scss" scoped>
  .el-main {
    padding: 10px;
  }
</style>