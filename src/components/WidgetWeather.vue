<script setup lang="ts">
import BaseWidget from "./BaseWidget.vue";

const weather = await (await fetch("https://weather.tsukumijima.net/api/forecast/city/130010")).json();
</script>

<template>
  <BaseWidget>
    <div :class="$style.container">
      <h1>{{ weather.location.city + "の" + weather.forecasts[1].dateLabel + "の天気" }}</h1>
      <div :class="$style.content">
        <div>
          <h2>天気: {{ weather.forecasts[1].telop }}</h2>
          <h2>気温: {{ weather.forecasts[1].temperature.min.celsius }}°C - {{
            weather.forecasts[1].temperature.max.celsius }}°C</h2>
        </div>
        <img :class="$style.image" :src="weather.forecasts[1].image.url" :alt="'weather image:' + weather.forecasts[1].image.title" />
      </div>
    </div>
  </BaseWidget>
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

.content {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 10px;
}

.image {
  flex: 0.7;
  object-fit: cover;
}
</style>
