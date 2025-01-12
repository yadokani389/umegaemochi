<script setup lang="ts">
import { onMounted, ref } from 'vue'
import BaseWidget from "./components/BaseWidget.vue";
import WidgetWeather from "./components/WidgetWeather.vue";
import WidgetNews from "./components/WidgetNews.vue";
import WidgetAtCoder from "./components/WidgetAtCoder.vue";
import WidgetCalendar from './components/WidgetCalendar.vue';
import WidgetClock from './components/WidgetClock.vue';
import WindowSettings from "./components/WindowSettings.vue";
import ButtonSettings from "./components/ButtonSettings.vue";
import WidgetPicto from './components/WidgetPicto.vue';

const isSettingsOpen = ref(false);

const widgets = [
  WidgetWeather,
  WidgetNews,
  WidgetAtCoder,
  WidgetCalendar,
  WidgetClock
];

const widgetIndex = ref(0);

function nextWidget() {
  widgetIndex.value = (widgetIndex.value + 1) % widgets.length;
}

function prevWidget() {
  widgetIndex.value = (widgetIndex.value - 1 + widgets.length) % widgets.length;
}

onMounted(() => {
  setInterval(nextWidget, 5000);
});

</script>

<template>
  <main>
    <ButtonSettings :class="$style.buttonsettings" v-model="isSettingsOpen" />
    <div :class="$style.container">
      <div :class="$style.widgetContainer">
        <transition name="slide-fade">
          <BaseWidget :class="$style.movewidget" :key="widgetIndex">
            <component :is="widgets[widgetIndex]" />
          </BaseWidget>
        </transition>
      </div>
      <BaseWidget :class="$style.picto">
        <WidgetPicto />
      </BaseWidget>
    </div>
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

.slide-fade-enter-active {
  transition: opacity 1.5s ease, transform 3.0s ease;
}

.slide-fade-leave-active {
  transition: opacity 1.5s ease, transform 3.0s ease;
}

.slide-fade-enter-from {
  opacity: 0;
  transform: translateY(-50vh);
}

.slide-fade-leave-to {
  opacity: 0;
  transform: translateY(50vh);
}

.slide-fade-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.slide-fade-leave-from {
  opacity: 1;
  transform: translateY(0);
}
</style>

<style module>
.container {
  display: flex;
  width: 100vw;
  height: 100vh;
  padding: 10vmin;
  gap: 8vmin;
  align-items: center;
  justify-items: center;
}

.widgetContainer {
  position: absolute;
  top: 17.5vh;
  width: 55vw;
  height: 65vh;
  display: flex;
  align-items: center;
  justify-content: center;
}

.movewidget {
  position: absolute;
  width: 100%;
  height: 100%;
}

.picto {
  position: absolute;
  right: 5vw;
  width: 30vw;
  height: 65vh;
  display: flex;
  align-items: center;
  justify-content: center;
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
