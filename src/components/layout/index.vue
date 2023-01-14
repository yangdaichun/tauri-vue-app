<template>
  <el-container>
    <t-side></t-side>
    <el-container>
      <t-header></t-header>
      <t-main></t-main>
      <el-footer>XX科技股份有限公司</el-footer>
    </el-container> 
  </el-container>
</template>
<script setup>
  import THeader from './header/index.vue'
  import TSide from './side/index.vue'
  import TMain from './main/index.vue'
</script>
<style lang="scss" scoped>
  .el-container {
    height: 100%;
    z-index: 1;
    .el-footer {
      height: 40px;
      padding: unset;
      line-height: 40px;
      font-size: 12px;
      text-align: center;
      border-top: 1px solid var(--el-border-color);
    }
  }
</style>
