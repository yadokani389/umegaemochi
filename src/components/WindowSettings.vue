<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
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

function getSettings() {
  invoke("get_settings").then((response) => {
    console.log(response);
  }).catch((error) => {
    console.error(error);
  });
}

let cityId = "";
let atcoderId = "";
const localIp = ref("");
const showQR = ref(false);
</script>

<template>
  <div :class="$style.container">
    <button @click="getServerAddress">Get server address</button>
    <canvas v-show="showQR" id="qr" />
    <div>{{ localIp }}</div>
    <button @click="getSettings">Settings</button>
    <input v-model="cityId" placeholder="City id" />
    <button @click="invoke('set_weather_city_id', { cityId: cityId })">set cityId</button>
    <input v-model="atcoderId" placeholder="AtCoder id" />
    <button @click="invoke('set_atcoder_id', { atcoderId: atcoderId })">set atcoderId</button>
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
