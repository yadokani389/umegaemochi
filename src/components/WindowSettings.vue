<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type } from "@tauri-apps/plugin-os";
import * as autostart from "@tauri-apps/plugin-autostart";
import { computed, ref } from "vue";
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

async function toggleFullscreen() {
  if (await getCurrentWindow().isFullscreen()) {
    await getCurrentWindow().setFullscreen(false);
  } else {
    await getCurrentWindow().setFullscreen(true);
  }
}

async function toggleCursorVisible() {
  if (previousCursorVisible) {
    await getCurrentWindow().setCursorVisible(false);
  } else {
    await getCurrentWindow().setCursorVisible(true);
  }
  previousCursorVisible = !previousCursorVisible;
}

async function toggleAutostart() {
  if (isAutostartEnabled.value) {
    await autostart.disable();
  } else {
    await autostart.enable();
  }
  isAutostartEnabled.value = await autostart.isEnabled();
}

const localIp = ref("");
const showQR = ref(false);
const osType = type();
let previousCursorVisible = true;
const isAutostartEnabled = ref(await autostart.isEnabled());
const autostartStatus = computed(() => isAutostartEnabled.value ? "Enabled" : "Disabled");
const version = await invoke<string>("get_version");
</script>

<template>
  <div :class="$style.container">
    <button @click="getServerAddress">Get server address</button>
    <canvas v-show="showQR" id="qr" />
    <div>{{ localIp }}</div>
    <template v-if="['linux', 'windows', 'macos'].includes(osType)">
      <button @click="toggleFullscreen">Toggle Fullscreen</button>

      <button @click="toggleCursorVisible">Toggle Cursor</button>

      <div :class="[$style.autostartStatus, autostartStatus === 'Enabled' ? $style.enabled : $style.disabled]">
        Autostart: {{ autostartStatus }}
      </div>
      <button @click="toggleAutostart()">Toggle Autostart</button>
    </template>
    <div>Version: v{{ version }}</div>
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
  gap: 15px;
  border-style: solid;
  border-radius: 30px;
  background-color: #f0f0f0;
}

.autostartStatus {
  font-weight: bold;
}

.enabled {
  color: green;
}

.disabled {
  color: grey;
}
</style>
