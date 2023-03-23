<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import WindowMode from "./components/WindowMode.vue";


</script>

<template>
  <el-card class="card">
    <WindowMode />
  </el-card>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.card {
  height: 90vh;
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
</style>
