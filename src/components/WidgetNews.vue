<script setup lang="ts">
import BaseWidget from "./BaseWidget.vue";
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

let newsList = ref(['Loading...']);
let newsTitle = ref('Loading...');
invoke('get_yahoo_news', { url: 'https://news.yahoo.co.jp/rss/topics/top-picks.xml' }).then((response) => {
  newsList.value = [];
  response.forEach((news, i) => {
    if (i === 0) {
      newsTitle.value = news;
      return;
    }
    newsList.value.push(news);
  })
}).catch((error) => {
  newsList.value = ['Error: ' + error];
});
</script>

<template>
  <BaseWidget>
    <div class="container">
      <h3>{{ newsTitle }}</h3>
      <ul>
        <li v-for="(news, index) in newsList" :key="index">
          {{ news }}
        </li>
      </ul>
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
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  font-family: "Koruri";
}

h3 {
  font-size: 2vmax;
}

ul {
  height: 80%;
  overflow-y: auto;
  font-size: 2vmax;
}
</style>
