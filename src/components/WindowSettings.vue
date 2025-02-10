<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type } from "@tauri-apps/plugin-os";
import * as autostart from "@tauri-apps/plugin-autostart";
import { computed, ref } from "vue";
import QRCode from "qrcode";
import SportsTopicSelectmenu from "./SportsTopicSelectmenu.vue";

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
const isAutostartEnabled = ref(['linux', 'windows', 'macos'].includes(osType) ? await autostart.isEnabled() : false);
const autostartStatus = computed(() => isAutostartEnabled.value ? "Enabled" : "Disabled");
const version = await invoke<string>("get_version");

const isSportSelectmenuOpen = ref(false);

function openSportSelectmenu() {
  if(isSportSelectmenuOpen.value) {
    isSportSelectmenuOpen.value = false;
  } else {
    isSportSelectmenuOpen.value = true;
  }
}

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
      <button @click="openSportSelectmenu">Edit Display Sports News Topic</button>
      <div v-show="isSportSelectmenuOpen" :class="$style.overlay" @click.self="openSportSelectmenu">
        <div :class="$style.sportSelectMenu">
          <SportsTopicSelectmenu />
        </div>
      </div>
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

.overlay {
  position: fixed;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 999;
}

.sportSelectMenu {
  position: fixed;
  left: 0;
  bottom: 0;
  height: 25vmin;
  width: 100%;
  background: white;
  border-radius: 10px;
  animation: slideUp 0.3s forwards;
}

@keyframes slideUp {
  from { transform: translateY(100%); }
  to { transform: translateY(0); }
}
</style>
