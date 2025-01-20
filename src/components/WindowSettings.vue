<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";
import { ref } from "vue";
import QRCode from "qrcode";

function getServerAddress() {
  invoke<string>("get_server_address").then(async (address) => {
    localIp.value = address;
    await QRCode.toCanvas(document.getElementById("qr"), address, { scale: 3, errorCorrectionLevel: "L" });
    showQR.value = true;
  }).catch((error) => {
    console.error(error);
  });
}

const localIp = ref("");
const showQR = ref(false);
</script>

<template>
  <div :class="$style.container">
    <button @click="getServerAddress">Get server address</button>
    <canvas v-show="showQR" id="qr" />
    <div>{{ localIp }}</div>
    <button @click="Window.getCurrent().setFullscreen(true)">Set Fullscreen</button>
    <button @click="Window.getCurrent().setFullscreen(false)">Exit Fullscreen</button>
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
  border-style: solid;
  border-radius: 30px;
  background-color: #f0f0f0;
}
</style>
