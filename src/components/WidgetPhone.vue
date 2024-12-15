<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

function startServer() {
  invoke("start_server").then((response) => {
    console.log(response);
  }).catch((error) => {
    console.log(error);
  });
}

</script>

<template>
  <div :class="$style.container">
    <h1>Phone connection</h1>
    <button @click="startServer">Start</button>
  </div>
</template>

<style module>
.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 10px;
}
</style>
