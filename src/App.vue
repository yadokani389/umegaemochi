<script setup lang="ts">
import { ref } from 'vue'
import BaseWidget from "./components/BaseWidget.vue";
import WidgetWeather from "./components/WidgetWeather.vue";
import WidgetAtCoder from "./components/WidgetAtCoder.vue";
import WindowSettings from "./components/WindowSettings.vue";
import ButtonSettings from "./components/ButtonSettings.vue";


const isSettingsOpen = ref(false)
</script>

<template>
  <main>
    <ButtonSettings :class="$style.buttonsettings" v-model="isSettingsOpen" />
    <dev :class="$style.container">
      <BaseWidget :class="$style.weather">
        <WidgetWeather />
      </BaseWidget>
      <BaseWidget :class="$style.atcoder">
        <WidgetAtCoder />
      </BaseWidget>
    </dev>
    <BaseWidget :class="$style.settings" v-if="isSettingsOpen">
      <WindowSettings />
    </BaseWidget>
  </main>
</template>

<style>
@font-face {
  font-family: "Koruri";
  src: url('/src/assets/fonts/Koruri-Semibold.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}

* {
  padding: 0;
  margin: 0;
  box-sizing: border-box;
  font-family: "Koruri";
  font-size: 4vmin;
}

h1 {
  font-size: 6vmin;
}
</style>

<style module>
.container {
  display: grid;
  width: 100vw;
  height: 100vh;
  padding: 10vmin;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
  gap: 8vmin;
}

.weather {
  grid-column: 1 / 2;
  grid-row: 1 / 2;
}

.atcoder {
  grid-column: 2 / 3;
  grid-row: 1 / 2;
}

main {
  position: relative;
}

.settings {
  position: absolute;
  border-style: solid;
  border-color: rgb(78, 78, 78);
  top: 0vmin;
  right: 0vmin;
  transform: scale(0.8);
}

.buttonsettings {
  position: absolute;
  top: 5px;
  right: 5px;
}
</style>
