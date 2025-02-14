<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useSwipe, UseSwipeDirection } from '@vueuse/core';
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';
import BaseWidget from "./components/BaseWidget.vue";
import WidgetWeather from "./components/WidgetWeather.vue";
import WidgetNews from "./components/WidgetNews.vue";
import WidgetAtCoder from "./components/WidgetAtCoder.vue";
import WidgetCalendar from './components/WidgetCalendar.vue';
import WidgetClock from './components/WidgetClock.vue';
import WidgetPicto from './components/WidgetPicto.vue';
import WidgetTodo from './components/WidgetTodo.vue';
import WidgetSportsNews from './components/WidgetSportsNews.vue';
import WidgetWeeklyWeather from './components/WidgetWeeklyWeather.vue';
import WidgetExchangeRate from './components/WidgetExchangeRate.vue';
import ButtonSettings from "./components/ButtonSettings.vue";
import WindowSettings from "./components/WindowSettings.vue";
import WindowEmergency from './components/WindowEmergency.vue';
import Tab from './components/Tab.vue'
import { DisasterInfo, Settings } from './types';

const pictoSrc = ref('/picto/cloudy.gif');
const isSettingsOpen = ref(false);
const disasterInfo = ref<DisasterInfo | null>(null);

listen<DisasterInfo>('disaster_occurred', (info) => {
  disasterInfo.value = info.payload;
});

listen('disaster_clear', () => {
  disasterInfo.value = null;
});

const widgets = [
  { name: 'WidgetWeather' as const, component: WidgetWeather, available: true },
  { name: 'WidgetNews' as const, component: WidgetNews, available: true },
  { name: 'WidgetAtCoder' as const, component: WidgetAtCoder, available: true },
  { name: 'WidgetCalendar' as const, component: WidgetCalendar, available: true },
  { name: 'WidgetClock' as const, component: WidgetClock, available: true },
  { name: 'WidgetTodo' as const, component: WidgetTodo, available: true },
  { name: 'WidgetSportsNews' as const, component: WidgetSportsNews, available: true },
  { name: 'WidgetWeeklyWeather' as const, component: WidgetWeeklyWeather, available: true },
  { name: 'WidgetExchangeRate' as const, component: WidgetExchangeRate, available: true },
];

let slideInterval = 10000;
let slideIntervalId: NodeJS.Timeout | null = null;
const currentWidget = ref(0);
const currentWidgetName = computed(() => widgets[currentWidget.value].name);
const direction = ref(0);

const sportsNewsIndex = ref(0);

watch(currentWidget, (newval, _) => {
  if (widgets[newval].name === 'WidgetSportsNews') {
    sportsNewsIndex.value++;
  }
});

function startAutoSlide() {
  stopAutoSlide();
  slideIntervalId = setInterval(nextWidget, slideInterval);
}

function stopAutoSlide() {
  if (slideIntervalId) {
    clearInterval(slideIntervalId);
    slideIntervalId = null;
  }
}

function nextWidget() {
  direction.value = 0;
  do {
    currentWidget.value = (currentWidget.value + 1) % widgets.length;
  } while (!widgets[currentWidget.value].available);
}

function prevWidget() {
  direction.value = 1;
  do {
    currentWidget.value = (currentWidget.value + widgets.length - 1) % widgets.length;
  } while (!widgets[currentWidget.value].available);
}

async function setWidget(widgetName: TargetWidget) {
  const targetIndex = widgets.findIndex(widget => widget.name === widgetName);
  if (targetIndex === -1 || !widgets[targetIndex].available) {
    console.warn(`Widget not found: ${widgetName}`);
    return;
  }
  currentWidget.value = targetIndex;
}

const transitionName = computed(() => {
  return direction.value === 1 ? 'slide-up' : 'slide-down';
});

type TargetWidget = (typeof widgets[number]['name']);

listen<TargetWidget | 'prev' | 'next'>('scroll', (target) => {
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

async function applySettings() {
  const settings = await invoke<Settings>('get_settings');
  slideInterval = settings.widget_interval;
  startAutoSlide();
  widgets.forEach(widget => {
    widget.available = settings.using_widgets.includes(widget.name);
  });
  if (!widgets[currentWidget.value].available) {
    currentWidget.value = 0;
    nextWidget();
  }
}

async function init() {
  applySettings();
  const settings = await invoke<Settings>('get_settings');
  if (settings.auto_fullscreen) {
    getCurrentWindow().setFullscreen(true);
  }
  if (settings.auto_hide_cursor) {
    getCurrentWindow().setCursorVisible(false);
    getCurrentWindow().setCursorPosition(new LogicalPosition(0, 0));
  }
}
listen("settings_changed", applySettings);

const container = ref<HTMLElement | null>(null);

useSwipe(container, {
  onSwipeEnd(_: TouchEvent, direction: UseSwipeDirection) {
    console.log(direction);
    if (direction === 'up') {
      prevWidget();
      stopAutoSlide();
      setTimeout(startAutoSlide, 10000);
    } else if (direction === 'down') {
      nextWidget();
      stopAutoSlide();
      setTimeout(startAutoSlide, 10000);
    }
  }
});

init();
</script>

<template>
  <main>
    <Tab :class="$style.tab" />
    <div :class="$style.container" ref="container">
      <div :class="$style.widgetContainer">
        <transition-group :name="transitionName">
          <template v-for="(widget, index) in widgets" :key="widget.name">
            <BaseWidget :class="$style.moveWidget" v-show="currentWidget === index">
              <component :is="widget.component" v-model="pictoSrc" :widget-name="currentWidgetName"
                :sports-news-index="sportsNewsIndex" />
            </BaseWidget>
          </template>
        </transition-group>
      </div>
      <BaseWidget :class="$style.picto">
        <WidgetPicto :picto-src="pictoSrc" />
      </BaseWidget>
    </div>
    <WindowEmergency :disastar-info="disasterInfo" :class="$style.emergency" v-if="disasterInfo" />
    <Suspense>
      <WindowSettings :class="$style.settings" v-if="isSettingsOpen" />
    </Suspense>
    <ButtonSettings :class="$style.buttonsettings" v-model="isSettingsOpen" />
  </main>
</template>

<style>
@font-face {
  font-family: "Koruri";
  src: url('/src/assets/fonts/Koruri-Semibold.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}

@font-face {
  font-family: 'ADLaMFont';
  src: url('/src/assets/fonts/ADLaMDisplay-Regular.ttf') format('truetype');
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

::-webkit-scrollbar {
  display: none;
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
  overflow: hidden;
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

.emergency {
  position: absolute;
  top: 0vmin;
  right: 0vmin;
}

.tab {
  position: absolute;
  top: 3vmin;
  left: 8vmin;
  height: 10%;
}
</style>

<style scoped>
.slide-up-enter-active,
.slide-up-leave-active,
.slide-down-enter-active,
.slide-down-leave-active {
  transition: opacity 1.5s ease, transform 3.0s ease;
}

.slide-up-enter-to,
.slide-up-leave-from,
.slide-down-enter-to,
.slide-down-leave-from {
  opacity: 1;
  transform: translateY(0);
}

.slide-up-leave-to,
.slide-down-enter-from {
  opacity: 0;
  transform: translateY(-50vh);
}

.slide-up-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(50vh);
}
</style>
