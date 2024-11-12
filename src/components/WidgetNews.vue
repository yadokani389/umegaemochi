<script setup lang="ts">
import BaseWidget from "./BaseWidget.vue";
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

let newsList = ref(['Loading...']);
invoke('get_yahoo_news', { url: 'https://news.yahoo.co.jp/rss/topics/top-picks.xml' }).then((response) => {
  newsList.value = [];
  response.forEach((news) => {
    newsList.value.push(news);
  })
}).catch((error) => {
  newsList.value = ['Error: ' + error];
});
</script>

<template>
  <BaseWidget>
    <div class="container">
      <li v-for="news in newsList">
        {{ news }}
      </li>
    </div>
  </BaseWidget>
</template>

<style scoped></style>
