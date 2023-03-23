<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const mode = ref(0)

async function greet() {

}

onMounted(() => {
  get_window_mode();
});

async function get_window_mode() {
  mode.value = await invoke("get_window_mode_handler");
  console.log(mode.value);
}

async function set_window_mode() {
  console.log(mode.value);
  await invoke("set_window_mode_handler", { mode: parseInt(mode.value) });
}
</script>

<template>
  <div class="mode" @change="set_window_mode">
    <el-text>模式切换: </el-text>
    <el-radio-group v-model="mode" style="margin-left: 10px;">
      <el-radio-button label="0">窗口模式</el-radio-button>
      <el-radio-button label="1">任务栏模式</el-radio-button>
    </el-radio-group>
  </div>
</template>

<style>
.mode {
  display: flex;
  flex: 1;
  flex-direction: row;
  align-items: center;
}
</style>
