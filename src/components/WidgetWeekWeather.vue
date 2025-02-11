<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { computedAsync } from "@vueuse/core";
import { ref, triggerRef } from "vue";
import { Settings } from "../types";
import { resolveResource } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/plugin-fs';

const resourcePath = await resolveResource('city_code.json');
const citys = JSON.parse(await readTextFile(resourcePath));

type Weather = {
  daily: {
    weather_code: number[];
    temperature_2m_max: number[];
    temperature_2m_min: number[];
    time: string[];
  };
} | { error: string; };

function weathername(weathercode: number) {
  if (weathercode < 2) return '晴れ'
  else if (weathercode < 4) return '曇り'
  else if (weathercode < 49) return ' 霧 '
  else if (weathercode < 67) return ' 雨 '
  else if (weathercode < 78) return ' 雪 '
  else if (weathercode < 83) return ' 雨 '
  else if (weathercode < 86) return ' 雪 '
  else if (weathercode < 100) return '雷雨'
}

const cityId = ref((await invoke<Settings>("get_settings")).weather_city_id);
const weather = computedAsync(async () => {
  return await (await fetch("https://api.open-meteo.com/v1/forecast?latitude=35.6895&longitude=139.6917&daily=weather_code,temperature_2m_max,temperature_2m_min")).json() as Weather;
}, null, { onError: (e) => console.error(e) });
console.log(cityId.value);
console.log(citys[cityId.value]);

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
      <h1>{{ citys[cityId].name }}の一週間天気</h1>
      <div :class="$style.content">
        <div :class="$style.detail">
          <h2>日付</h2>
          <h2 v-for="n in 7">{{ weather.daily.time[n - 1].slice(5) }}</h2>
        </div>
        <div :class="$style.detail">
          <h2>天気</h2>
          <h2 v-for="n in 7">{{ weathername(weather.daily.weather_code[n - 1]) }}</h2>
        </div>
        <div :class="$style.detail">
          <h2 :class="$style.max_temperature">最高気温</h2>
          <h2 :class="$style.max_temperature" v-for="n in 7">{{ weather.daily.temperature_2m_max[n - 1] }}°C</h2>
        </div>
        <div :class="$style.detail">
          <h2 :class="$style.min_temperature">最低気温</h2>
          <h2 :class="$style.min_temperature" v-for="n in 7">{{ weather.daily.temperature_2m_min[n - 1] }}°C</h2>
        </div>
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
  background-color: none;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 10px;
  padding-left: 50px;
}

.content {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  align-items: center;
  gap: 10px;
}

.detail {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 5px;
}

.max_temperature {
  color: rgb(255, 100, 100);
}

.min_temperature {
  color: rgb(100, 100, 255);
}
</style>
