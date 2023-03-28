<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

// 模式
const mode = ref(0)

// 是否启用内置脚本
const enableInternalScript = ref(false)

onMounted(() => {
  get_window_mode();
});

async function get_window_mode() {
  mode.value = await invoke("get_window_mode_handler");
  console.log(mode.value);
}

async function set_window_mode() {
  console.log(mode.value);
  await invoke("set_window_mode_handler", { mode: parseInt(mode.value + "") });
}
</script>

<template>
  <div class="column-layout">
    <!-- 模式切换 -->
    <el-card class="width-parent">
      <div class="row-layout" @change="set_window_mode">
        <el-text>模式切换 </el-text>
        <el-radio-group v-model="mode" style="margin-left: 10px;">
          <el-radio-button label="0">窗口模式</el-radio-button>
          <el-radio-button label="1">任务栏模式</el-radio-button>
        </el-radio-group>
      </div>
    </el-card>

    <!-- 是否启用内置脚本 -->
    <el-card class="column-layout width-parent" style="margin-top: 4px;">
      <div class="row-layout">
        <el-text>内置脚本 </el-text>
        <el-radio-group v-model="enableInternalScript" style="margin-left: 10px;">
          <el-radio-button label="true">启用</el-radio-button>
          <el-radio-button label="false">关闭</el-radio-button>
        </el-radio-group>
      </div>
      <span class="font-size-10">注：该功能启用后，可能会导致程序不稳定，请谨慎开启！</span>
    </el-card>
  </div>
</template>

<style>
.width-parent {
  width: 100%;
}

.el-card {
  --el-card-padding: 15px;
  --el-card-border-radius: 8px
}
</style>
