<script setup lang="ts">
import { ref, computed } from 'vue'
import { listen } from '@tauri-apps/api/event';
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

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const widgets = {
  "WidgetWeather": WidgetWeather,
  "WidgetNews": WidgetNews,
  "WidgetAtCoder": WidgetAtCoder,
  "WidgetCalendar": WidgetCalendar,
  "WidgetClock": WidgetClock
};

const slideInterval = ref<number | undefined>(undefined);
const currentWidget = ref<keyof typeof widgets>("WidgetWeather");
const direction = ref(0);

function startAutoSlide() {
  stopAutoSlide();
  slideInterval.value = setInterval(nextWidget, 10000) as unknown as number;
}

function stopAutoSlide() {
  if (slideInterval.value) {
    clearInterval(slideInterval.value);
    slideInterval.value = undefined;
  }
}

function nextWidget() {
  const widgetKeys = Object.keys(widgets);
  currentWidget.value = widgetKeys[(widgetKeys.indexOf(currentWidget.value) + 1) % widgetKeys.length] as keyof typeof widgets;
  direction.value = 0;
}

function prevWidget() {
  const widgetKeys = Object.keys(widgets);
  currentWidget.value = widgetKeys[(widgetKeys.indexOf(currentWidget.value) + widgetKeys.length - 1) % widgetKeys.length] as keyof typeof widgets;
  direction.value = 1;
}

async function setWidget(widgetName: ScrollTarget) {
  const widgetKeys = Object.keys(widgets);
  const targetIndex = widgetKeys.indexOf(widgetName as keyof typeof widgets);
  if (targetIndex === -1) {
    console.warn(`Widget not found: ${widgetName}`);
    return;
  }
  stopAutoSlide();
  const currentIndex = widgetKeys.indexOf(currentWidget.value);
  const totalWidgets = widgetKeys.length;
  const forwardDistance = (targetIndex - currentIndex + totalWidgets) % totalWidgets;
  const backwardDistance = (currentIndex - targetIndex + totalWidgets) % totalWidgets;
  const directionForward = forwardDistance <= backwardDistance;
  const steps = directionForward ? forwardDistance : backwardDistance;

  for (let i = 0; i < steps; i++) {
    if (directionForward) {
      nextWidget();
    } else {
      prevWidget();
    }
    await sleep(1000);
  }

  setTimeout(startAutoSlide, 30000);
}

const transitionName = computed(() => {
  return direction.value === 1 ? 'slide-up' : 'slide-down';
});

type ScrollTarget = typeof widgets[keyof typeof widgets] | "prev" | "next";

listen<ScrollTarget>('scroll', (target) => {
  stopAutoSlide();
  if (target.payload === 'prev') {
    prevWidget();
  } else if (target.payload === 'next') {
    nextWidget();
  } else {
    setWidget(target.payload);
  }
  setTimeout(startAutoSlide, 30000);
});


startAutoSlide();

</script>
<template>
  <main>
    <ButtonSettings :class="$style.buttonsettings" v-model="isSettingsOpen" />
    <div :class="$style.container">
      <div :class="$style.widgetContainer">
        <transition :name="transitionName">
          <BaseWidget :class="$style.moveWidget" :key="currentWidget">
            <component :is="widgets[currentWidget]" />
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
  color: #70ad47;
}

h1 {
  font-size: 6vmin;
}

.slide-up-enter-active,
.slide-down-enter-active {
  transition: opacity 1.5s ease, transform 3.0s ease;
}

.slide-up-leave-active,
.slide-down-leave-active {
  transition: opacity 1.5s ease, transform 3.0s ease;
}

.slide-up-enter-from {
  opacity: 0;
  transform: translateY(50vh);
}

.slide-up-leave-to {
  opacity: 0;
  transform: translateY(-50vh);
}

.slide-up-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.slide-up-leave-from {
  opacity: 1;
  transform: translateY(0);
}

.slide-down-enter-from {
  opacity: 0;
  transform: translateY(-50vh);
}

.slide-down-leave-to {
  opacity: 0;
  transform: translateY(50vh);
}

.slide-down-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.slide-down-leave-from {
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

.moveWidget {
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
  overflow: hidden;
}

.settings {
  position: absolute;
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
