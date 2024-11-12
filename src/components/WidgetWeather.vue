<script setup lang="ts">
import BaseWidget from "./BaseWidget.vue";

const weather = await (await fetch("https://weather.tsukumijima.net/api/forecast/city/130010")).json();
</script>

<template>
  <BaseWidget>
    <div class="weather-container">
      <div class="weather-header">
        <h1 class="title">{{ weather.location.city + "の" + weather.forecasts[1].dateLabel + "の天気" }}</h1>
      </div>
      <div class="weather-content">
        <div class="weather-info">
          <p>Weather: {{ weather.forecasts[1].telop }}</p>
          <p>Temperature: {{ weather.forecasts[1].temperature.min.celsius }}°C - {{
            weather.forecasts[1].temperature.max.celsius }}°C</p>
        </div>
        <img class="weather-image" :src="weather.forecasts[1].image.url"
          :alt="'weather image:' + weather.forecasts[1].image.title" />
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

.weather-container {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  font-family: "Koruri";
}

.weather-header {
  width: 100%;
  display: flex;
  justify-content: center;
  margin-bottom: 1rem;
}

.title {
  font-size: 1.5rem;
  margin: 0;
}

.weather-content {
  display: flex;
  justify-content: space-between;
  width: 100%;
  align-items: center;
}

.weather-info {
  text-align: left;
  flex: 1;
  margin-left: 10px;
}

.weather-image {
  width: 45%;
  height: auto;
  margin-left: 1rem;
}
</style>