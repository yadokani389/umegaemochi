<script setup lang="ts">
import BaseWidget from "./BaseWidget.vue";

const weather = await (await fetch("https://weather.tsukumijima.net/api/forecast/city/130010")).json();
</script>

<template>
  <BaseWidget>
    <div>
      <h1>{{ weather.location.city + "の" + weather.forecasts[1].dateLabel + "の天気" }}</h1>
      <p>Weather: {{ weather.forecasts[1].telop }}</p>
      <p>Temperature: {{ weather.forecasts[1].temperature.min.celsius }}°C - {{
        weather.forecasts[1].temperature.max.celsius }}°C</p>
      <img :src="weather.forecasts[1].image.url" :alt="'weather image:' + weather.forecasts[1].image.title" />
    </div>
  </BaseWidget>
</template>
