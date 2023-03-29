<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import GeneralPreference from "./components/GeneralPreference.vue";
import NewUI from "./components/NewUI.vue";


</script>

<template>
  <div class="card" @contextmenu.prevent>
    <GeneralPreference />
    <!-- <NewUI /> -->
  </div>
  <!-- <el-card>
   
  </el-card> -->
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.card {
  width: 100%;
  height: 95vh;
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
</style>
