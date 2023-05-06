<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { getVersion } from "@tauri-apps/api/app";
import { onMounted, ref } from "vue";
import GeneralPreference from "./components/GeneralPreference.vue";
import CustomMenu from "./components/tab/CustomMenu.vue";
import MultiTabPage from "./components/tab/MultiTabPage.vue";

const appVersion = ref("未知版本");

onMounted(() => {
  getData();
})

async function getData() {
  appVersion.value = await getVersion()
}

</script>

<template>
  <div class="card" @contextmenu.prevent>
    <!-- <GeneralPreference /> -->
    <!-- <CustomMenu /> -->
    <!-- <NewUI /> -->
    <!-- <MultiTabPage />

    <footer class="footer">© Github@Cloris Version {{ appVersion }} </footer> -->

    <div>
      <nav>
        <router-link to="/"></router-link>
        <router-link to="/mirror"></router-link>
      </nav>
      <router-view></router-view>
    </div>
  </div>
  <!-- <el-card>
   
          </el-card> -->
</template>

<style scoped>
.footer {
  height: 25px;
  position: fixed;
  left: 0;
  bottom: 0;
  width: 100%;
  font-size: small;
  color: rgba(0, 0, 0, 0.3);
  text-align: center;
  z-index: 999;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.card {
  margin-bottom: 30px;
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  padding: 0;
  box-sizing: border-box;
}
</style>
