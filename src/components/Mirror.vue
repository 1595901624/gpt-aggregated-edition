<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="card">
    <a class="app-item" title="ChatGPT">
      <div class="app-icon">
        <img alt="ico" height="68" src="https://st.ai55.cc/favicon/gpt.xcbl.cc.ico" width="68">
      </div>
      <div class="app-text">
        <p class="app-title">Url.ai-node.com</p>
        <p class="app-desc">😄⭐ </p>
      </div>
    </a>
  </div>

</template>
