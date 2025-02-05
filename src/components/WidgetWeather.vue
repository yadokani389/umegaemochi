<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { computedAsync } from "@vueuse/core";
import { ref, triggerRef } from "vue";
import { Settings } from "../types";

type Weather = {
  location: {
    city: string;
  };
  forecasts: {
    dateLabel: string;
    telop: string;
    temperature: {
      min: {
        celsius: string;
      };
      max: {
        celsius: string;
      };
    };
    image: {
      url: string;
      title: string;
    };
  }[];
} | { error: string; };

const emit = defineEmits(["updatePicto"]);
const cityId = ref((await invoke<Settings>("get_settings")).weather_city_id);
const weather = computedAsync(async () => {
  return await (await fetch("https://weather.tsukumijima.net/api/forecast/city/" + cityId.value)).json() as Weather;
}, null, { onError: (e) => console.error(e) });

emit("updatePicto", '/picto/cloudy.gif');

listen("settings_changed", async () => {
  cityId.value = (await invoke<Settings>("get_settings")).weather_city_id;
});

listen("daily_reload", async () => {
  triggerRef(cityId);
});
</script>

<template>
  <div :class="$style.container" v-if="weather">
    <template v-if="!('error' in weather)">
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
    </template>
    <template v-else>
      <h1>天気情報の取得に失敗しました</h1>
      <h2>{{ weather.error }}</h2>
    </template>
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
  width: 40%;
  height: auto;
  max-height: 100%;
  object-fit: contain;
  background-color: transparent;
}
</style>
