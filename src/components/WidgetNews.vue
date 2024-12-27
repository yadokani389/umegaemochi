<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useAsyncState } from "@vueuse/core";

const newsList = useAsyncState(async () => {
  return (await invoke<string[]>('get_yahoo_news', { url: 'https://news.yahoo.co.jp/rss/topics/top-picks.xml' })).toSpliced(0, 1);
}, [], { onError: (e) => console.error(e) }).state;
</script>

<template>
  <div :class="$style.container">
    <h1>Yahoo!ニュース</h1>
    <div :class="$style.content">
      <div v-if="1 < newsList.length" :class="$style.scrollTrack">
        <h2 v-for="(news, index) in [...newsList, ...newsList]" :key="index" :class="$style.news">
          {{ news }}
        </h2>
      </div>
      <h2 v-else>Loading...</h2>
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
  padding: 10px;
}

.content {
  overflow-y: auto;
}

@keyframes infiniteScroll {
  0% {
    transform: translateY(0);
  }

  100% {
    transform: translateY(-50%);
  }
}

.scrollTrack {
  animation: infiniteScroll 20s linear infinite;
}

.news {
  font-size: 4vmin;
  padding: 30px;
}
</style>
