<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { ref } from "vue";

type Settings = {
  weather_city_id: String;
  atcoder_id: String;
};

async function setup() {
  const cityId = await (async () => {
    try {
      return (await invoke("get_settings") as Settings).weather_city_id;
    } catch (error) {
      console.error(error);
      return 130010;
    }
  })();

  const weather = await (await fetch("https://weather.tsukumijima.net/api/forecast/city/" + cityId)).json();
  return weather;
}

listen("settings_changed", async () => {
  weather.value = await setup();
});

let weather = ref(await setup());
</script>

<template>
  <div :class="$style.container" v-if="weather">
    <h1>{{ weather.location.city }}の{{ weather.forecasts[1].dateLabel }}の天気</h1>
    <div :class="$style.content">
      <div>
        <h2>天気: {{ weather.forecasts[1].telop }}</h2>
        <h2>気温: {{ weather.forecasts[1].temperature.min.celsius }}°C - {{
          weather.forecasts[1].temperature.max.celsius }}°C</h2>
      </div>
      <img :class="$style.image" :src="weather.forecasts[1].image.url"
        :alt="'weather image:' + weather.forecasts[1].image.title" />
    </div>
  </div>
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
