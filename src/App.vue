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

const widgets = [
  { name: 'WidgetWeather', component: WidgetWeather },
  { name: 'WidgetNews', component: WidgetNews },
  { name: 'WidgetAtCoder', component: WidgetAtCoder },
  { name: 'WidgetCalendar', component: WidgetCalendar },
  { name: 'WidgetClock', component: WidgetClock }
];

const slideInterval = ref<number | undefined>(undefined);
const widgetIndex = ref(0);
const direction = ref(0);

function startAutoSlide() {
  stopAutoSlide();
  slideInterval.value = setInterval(() => {
    nextWidget();
  }, 10000) as unknown as number;
}

function stopAutoSlide() {
  if (slideInterval.value) {
    clearInterval(slideInterval.value);
    slideInterval.value = undefined;
  }
}

function nextWidget() {
  widgetIndex.value = (widgetIndex.value + 1) % widgets.length;
  direction.value = 0;
}

function prevWidget() {
  widgetIndex.value = (widgetIndex.value - 1 + widgets.length) % widgets.length;
  direction.value = 1;
}

async function setWidget(widgetName: String) {
  stopAutoSlide();

  const targetIndex = widgets.findIndex(widget => widget.name === widgetName);
  const currentIndex = widgetIndex.value;
  const totalWidgets = widgets.length;
  const forwardDistance = (targetIndex - currentIndex + totalWidgets) % totalWidgets;
  const backwardDistance = (currentIndex - targetIndex + totalWidgets) % totalWidgets;
  const directionForward = forwardDistance <= backwardDistance;
  const steps = directionForward ? forwardDistance : backwardDistance;

  for (let i = 0; i < steps; i++) {
    setTimeout(() => {
      if (directionForward) {
        nextWidget();
      } else {
        prevWidget();
      }
    }, 300);
  }

  setTimeout(() => {
    startAutoSlide();
  }, 30000);
}

const transitionName = computed(() => {
  return direction.value === 1 ? 'slide-up' : 'slide-down';
});

type ScrollTarget =
  | "WidgetWeather"
  | "WidgetNews"
  | "WidgetAtCoder"
  | "WidgetCalendar"
  | "WidgetClock"
  | "prev"
  | "next";

const scrollActions: Record<ScrollTarget, () => void> = {
  WidgetWeather: () => setWidget('WidgetWeather'),
  WidgetNews: () => setWidget('WidgetNews'),
  WidgetAtCoder: () => setWidget('WidgetAtCoder'),
  WidgetCalendar: () => setWidget('WidgetCalendar'),
  WidgetClock: () => setWidget('WidgetClock'),
  prev: () => {
    stopAutoSlide();
    prevWidget();
    setTimeout(() => {
      startAutoSlide();
    }, 30000);
  },
  next: () => {
    stopAutoSlide();
    nextWidget();
    setTimeout(() => {
      startAutoSlide();
    }, 30000);
  }
};

listen<ScrollTarget>('scroll', (target) => {
  const action = scrollActions[target.payload];  
  if (action) {
    action();
  } else {
    console.warn(`Unhandled scroll target: ${target.payload}`);
  }
});


startAutoSlide();

</script>
<template>
  <main>
    <ButtonSettings :class="$style.buttonsettings" v-model="isSettingsOpen" />
    <div :class="$style.container">
      <div :class="$style.widgetContainer">
        <transition :name="transitionName">
          <BaseWidget :class="$style.moveWidget" :key="widgetIndex">
            <component :is="widgets[widgetIndex].component" />
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
