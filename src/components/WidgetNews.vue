<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { useAsyncState } from "@vueuse/core";
import { computed, watch } from "vue";

const { widgetName } = defineProps<{ widgetName: string; }>();
const model = defineModel();
const { state: newsList, execute: refetch } = useAsyncState(async () => {
  return await invoke<string[]>('get_yahoo_news', { url: 'https://news.yahoo.co.jp/rss/topics/top-picks.xml' });
}, [], { onError: (e) => console.error(e) });

const scrollDuration = computed(() => { return `${5 * newsList.value.length}s`; });

watch(() => widgetName, () => {
  if (widgetName === 'WidgetNews') {
    model.value = '/picto/news.gif';
  }
});

listen("daily_reload", async () => {
  await refetch();
});
</script>

<template>
  <div :class="$style.container">
    <h1>Yahoo!ニュース</h1>
    <div :class="$style.content">
      <div v-if="1 < newsList.length" :class="$style.scrollTrack" :style="{ animationDuration: scrollDuration }">
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
  gap: 3%;
  padding: 3%;
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
  animation: infiniteScroll linear infinite;
}

.news {
  font-size: 4vmin;
  padding: 30px;
}
</style>
