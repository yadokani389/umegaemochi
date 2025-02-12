<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { computedAsync } from "@vueuse/core";
import { ref, triggerRef, watch } from "vue";
import { Settings } from "../types";
import { resolveResource } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/plugin-fs';

type Cities = { [key: string]: { lnglat: [number, number]; name: string; } };

type Weather = {
  daily: {
    weather_code: number[];
    temperature_2m_max: number[];
    temperature_2m_min: number[];
    time: string[];
  };
} | {
  error: boolean;
  reason: string;
};

function weatherName(weatherCode: number) {
  if (weatherCode < 2) return '晴れ';
  else if (weatherCode < 4) return '曇り';
  else if (weatherCode < 49) return '霧';
  else if (weatherCode < 67) return '雨';
  else if (weatherCode < 78) return '雪';
  else if (weatherCode < 83) return '雨';
  else if (weatherCode < 86) return '雪';
  else if (weatherCode < 100) return '雷雨';
  else return '不明';
}

const week = ["月", "火", "水", "木", "金", "土", "日"];

const resourcePath = await resolveResource('city_code.json');
const cities = JSON.parse(await readTextFile(resourcePath)) as Cities;

const { widgetName } = defineProps<{ widgetName: string; }>();
const model = defineModel();
const cityId = ref((await invoke<Settings>("get_settings")).weather_city_id);
const weather = computedAsync(async () => {
  if (!(cityId.value in cities)) return { error: true, reason: 'The specified city ID is invalid.' } as Weather;
  const [lng, lat] = cities[cityId.value].lnglat;
  const url = `https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${lng}&daily=weather_code,temperature_2m_max,temperature_2m_min`;
  return await (await fetch(url)).json() as Weather;
}, null, { onError: (e) => console.error(e) });

watch(() => widgetName, () => {
  if (widgetName === 'WidgetWeeklyWeather') {
    model.value = '/picto/rain_dohshaburi.gif';
  }
});

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
      <h1>{{ cities[cityId].name }}の一週間の天気</h1>
      <div :class="$style.content">
        <div :class="$style.detail">
          <h2>日付</h2>
          <h2 v-for="n in 7">{{ weather.daily.time[n - 1].slice(5) }}({{ week[((new Date).getDay() + n - 2) % 7] }})
          </h2>
        </div>
        <div :class="$style.detail">
          <h2>天気</h2>
          <h2 v-for="n in 7">{{ weatherName(weather.daily.weather_code[n - 1]) }}</h2>
        </div>
        <div :class="$style.detail">
          <h2 :class="$style.max_temperature" style="font-size: 0.8em">最高気温</h2>
          <h2 :class="$style.max_temperature" v-for="n in 7">{{ weather.daily.temperature_2m_max[n - 1] }}°C</h2>
        </div>
        <div :class="$style.detail">
          <h2 :class="$style.min_temperature" style="font-size: 0.8em">最低気温</h2>
          <h2 :class="$style.min_temperature" v-for="n in 7">{{ weather.daily.temperature_2m_min[n - 1] }}°C</h2>
        </div>
      </div>
    </template>
    <template v-else>
      <h1>天気情報の取得に失敗しました</h1>
      <h2>{{ weather.reason }}</h2>
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
