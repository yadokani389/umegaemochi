<script setup lang="ts">
import BaseWidget from "./components/BaseWidget.vue";
import WidgetWeather from "./components/WidgetWeather.vue";
import WidgetNews from "./components/WidgetNews.vue";
import WidgetPicto from "./components/WidgetPicto.vue";
</script>

<template>
  <main :class="$style.container">
    <BaseWidget :class="$style.weather">
      <WidgetWeather />
    </BaseWidget>
    <BaseWidget :class="$style.news">
      <WidgetNews />
    </BaseWidget>
    <BaseWidget :class="$style.picto">
      <WidgetPicto />
    </BaseWidget>
  </main>
</template>

<style>
@font-face {
  font-family: "Koruri";
  src: url('../assets/fonts/Koruri-Semibold.ttf') format('truetype');
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

.news {
  grid-column: 1 / 2;
  grid-row: 2 / 3;
}

.picto {
  grid-column: 2 / 3;
  grid-row: 1 / 3;
}
</style>
