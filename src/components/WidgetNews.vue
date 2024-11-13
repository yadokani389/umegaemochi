<script setup lang="ts">
import BaseWidget from "./BaseWidget.vue";
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

let newsList = ref(['Loading...']);
invoke('get_yahoo_news', { url: 'https://news.yahoo.co.jp/rss/topics/top-picks.xml' }).then((response) => {
  newsList.value = [];
  (response as string[]).toSpliced(0, 1).forEach((news) => {
    newsList.value.push(news);
  })
}).catch((error) => {
  newsList.value = ['Error: ' + error];
});
</script>

<template>
  <BaseWidget>
    <div :class="$style.container">
      <h1>Yahoo!ニュース</h1>
      <ul :class="$style.news">
        <li v-for="(news, index) in newsList" :key="index">
          {{ news }}
        </li>
      </ul>
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
  padding: 10px;
}

.news {
  overflow-y: auto;
}
</style>
