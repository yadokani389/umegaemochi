<script setup lang="ts">
import BaseWidget from "./BaseWidget.vue";

const weather = await (await fetch("https://weather.tsukumijima.net/api/forecast/city/130010")).json();
</script>

<template>
  <BaseWidget>
    <div class="container">
      <h1 class="title">{{ weather.location.city + "の" + weather.forecasts[1].dateLabel + "の天気" }}</h1>
      <div class="content">
        <div class="info">
          <h3>天気: {{ weather.forecasts[1].telop }}</h3>
          <h3>気温: {{ weather.forecasts[1].temperature.min.celsius }}°C - {{
            weather.forecasts[1].temperature.max.celsius }}°C</h3>
        </div>
        <div class="image">
          <img :src="weather.forecasts[1].image.url" :alt="'weather image:' + weather.forecasts[1].image.title" />
        </div>
      </div>
    </div>
  </BaseWidget>
</template>

<style scoped>
@font-face {
  font-family: "Koruri";
  src: url('../assets/fonts/Koruri-Semibold.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}

.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  font-family: "Koruri";
}

.title {
  display: flex;
  font-size: 3vmax;
  margin: 0;
}

.content {
  display: flex;
  justify-content: space-between;
  width: 100%;
  height: 100%;
  align-items: center;
}

.info {
  text-align: left;
  flex: 3;
}

h3 {
  font-size: 2vmax;
}

.image {
  height: 100%;
  flex: 2;
}

img {
  width: 100%;
  height: 100%;
  object-fit: fill;
}
</style>
