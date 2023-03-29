<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const isCollapse = ref(true)
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}

</script>

<template>
  <el-menu default-active="4" class="el-menu-vertical-demo" :collapse="isCollapse" @open="handleOpen"
  @close="handleClose">
  <!-- <el-sub-menu index="1">
    <template #title>
      <el-icon>
        <location />
      </el-icon>
        <span>Navigator One</span>
        </template>
        <el-menu-item-group>
              <template #title><span>Group One</span></template>
              <el-menu-item index="1-1">item one</el-menu-item>
              <el-menu-item index="1-2">item two</el-menu-item>
            </el-menu-item-group>
            <el-menu-item-group title="Group Two">
              <el-menu-item index="1-3">item three</el-menu-item>
            </el-menu-item-group>
            <el-sub-menu index="1-4">
              <template #title><span>item four</span></template>
              <el-menu-item index="1-4-1">item one</el-menu-item>
            </el-sub-menu>
      </el-sub-menu> -->
    <el-menu-item index="1">
      <el-icon><location /></el-icon>
      <template #title>Navigator Two</template>
    </el-menu-item>
    <el-menu-item index="2">
      <el-icon><home /></el-icon>
      <template #title>Navigator Two</template>
    </el-menu-item>
    <el-menu-item index="3">
      <el-icon>
        <document />
      </el-icon>
      <template #title>Navigator Three</template>
    </el-menu-item>
    <el-menu-item index="4">
      <el-icon>
        <setting />
      </el-icon>
      <template>
        <div style="width: 100%; height: 100vh; background-color: brown;">
          <iframe src="https://yiyan.baidu.com"></iframe>
        </div>
      </template>
    </el-menu-item>
  </el-menu>
</template>


