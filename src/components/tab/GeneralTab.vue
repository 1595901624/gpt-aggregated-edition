<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { PREFERENCE_AUTO_HIDE_WHEN_CLICK_OUTSIDE } from "../../constant";
import { Preference } from "../../model/preference";
import { WindowMode } from "../../model/preference";

// 模式
// const mode = ref(0)

// 是否启用内置脚本
// const enableInternalScript = ref<boolean>(false)

// 任务栏模式点击窗口外侧自动隐藏
// const autoHideWhenClickOutside = ref("true");

// 当点击关闭时退出应用
// const exitWhenClickClose = ref<boolean>(false);

const preference = ref<Preference>({
  window_mode: 0,
  enable_internal_script: false,
  enable_extendsion_script: false,
  auto_hide_when_click_outside: true,
  current_page_url: '',
  exit_app: false
});

onMounted(() => {
  getPreferenceData();
});

/**
 * 拉取设置数据
 */
async function getPreferenceData() {
  // mode.value = await invoke('get_window_mode_handler');
  // enableInternalScript.value = await invoke('is_enable_internal_script_handler');
  // autoHideWhenClickOutside.value = await invoke('get_preference_handler', { key: PREFERENCE_AUTO_HIDE_WHEN_CLICK_OUTSIDE, value: "true" });
  // exitWhenClickClose.value = await invoke('get_preference_handler', )

  preference.value = await invoke('get_app_preference_handler') as Preference;
  console.log(preference.value);
}

async function setPreferenceData() {
  let result = await invoke('set_app_preference_handler', { p: JSON.stringify(preference.value) });
  console.log(result);
}

// /**
//  * 设置窗口模式
//  *
//  */
// async function setWindowMode() {
//   // console.log(mode.value);
//   await invoke("set_window_mode_handler", { mode: parseInt(mode.value + "") });
// }

// /**
//  * 设置是否启用内置脚本
//  */
// async function enableInternalScriptHandler() {
//   await invoke('enable_internal_script_handler', { enable: (enableInternalScript.value + "") == 'true' });
// }

// /**
//  * 点击窗口外侧自动隐藏（任务栏模式）
//  */
// async function setAutoHideWhenClickOutside() {
//   await invoke('set_preference_handler', { key: PREFERENCE_AUTO_HIDE_WHEN_CLICK_OUTSIDE, value: autoHideWhenClickOutside.value });
// }

</script>

<template>
  <div class="column-layout">
    <!-- 模式切换 -->
    <div class="width-parent preference-card">
      <div class="row-layout" @change="setPreferenceData">
        <span class="set-title">模式切换 </span>
        <el-radio-group v-model="preference.window_mode" style="margin-left: 10px;">
          <el-radio-button :label="WindowMode[0]">窗口模式</el-radio-button>
          <el-radio-button :label="WindowMode[1]">任务栏模式</el-radio-button>
          <el-radio-button :label="WindowMode[2]">侧边栏模式（试验）</el-radio-button>
          <el-radio-button :label="WindowMode[3]">QQ模式（试验）</el-radio-button>
        </el-radio-group>
      </div>

      <!-- 窗口模式子设置项 -->
      <div class="row-layout common-margin-top" v-if="preference.window_mode.toString() == WindowMode[0]"
        @change="setPreferenceData">
        <span class="set-title">点击关闭退出应用 </span>
        <el-radio-group v-model="preference.exit_app" style="margin-left: 10px;">
          <el-radio-button :label="true">是</el-radio-button>
          <el-radio-button :label="false">否</el-radio-button>
        </el-radio-group>
      </div>

      <!-- 任务栏模式子设置项 -->
      <div class="row-layout common-margin-top" v-if="preference.window_mode.toString() == WindowMode[1]" @change="setPreferenceData">
        <span class="set-title">点击窗口外侧自动隐藏窗口 </span>
        <el-radio-group v-model="preference.auto_hide_when_click_outside" style="margin-left: 10px;">
          <el-radio-button :label="true">开启</el-radio-button>
          <el-radio-button :label="false">关闭</el-radio-button>
        </el-radio-group>
      </div>
    </div>

    <!-- 是否启用内置脚本 -->
    <div class="column-layout width-parent preference-card" style="margin-top: 4px;">
      <div class="row-layout" @change="setPreferenceData">
        <span class="set-title">内置脚本 </span>
        <el-radio-group v-model="preference.enable_internal_script" style="margin-left: 10px;">
          <el-radio-button :label="true">开启</el-radio-button>
          <el-radio-button :label="false">关闭</el-radio-button>
        </el-radio-group>
      </div>
      <span class="set-subtitle common-margin-top-8">该功能启用后，可能会导致程序不稳定，请谨慎开启！更改设置后，重启后生效。如果遇到卡顿或者其它问题，请关闭此选项。</span>
    </div>

    <!-- <div class="column-layout width-parent preference-card" style="margin-top: 4px;">
                        </div> -->
  </div>
</template>

<style>
.width-parent {
  width: 100%;
}

/* .el-card {
  --el-card-padding: 15px;
  --el-card-border-radius: 8px
} */

.preference-card {
  box-sizing: border-box;
  padding: 12px;
  border-radius: 8px;
  border: 1px solid #e4e7ed;
  background-color: white;
  transition: 0.3s;
  color: #303133;
  overflow: hidden;
  /* box-shadow: 1px 1px 1px rgba(0, 0, 0, .1);
  -moz-box-shadow: 1px 1px 1px rgba(0, 0, 0, .1);
  -webkit-box-shadow: 1px 1px 1px rgba(0, 0, 0, .1); */
}

.set-title {
  font-size: 14px;
  color: black;
}

.set-subtitle {
  font-size: 10px;
  color: rgba(0, 0, 0, 0.5);
}
</style>
