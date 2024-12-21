<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

function startServer() {
  invoke("start_server").then((response) => {
    console.log(response);
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
</script>

<template>
  <div :class="$style.container">
    <h1>Phone connection</h1>
    <button @click="startServer">Start server</button>
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
