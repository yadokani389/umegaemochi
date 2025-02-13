<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, computed, watch } from "vue";
import { Settings } from "../types";

const articleTitles = ref<string[]>([]);

const selectedTopics = ref<string[]>(["baseball/professional"]);

const topicsToId = [
  { name: 'プロ野球', id: 'baseball/professional'},
  { name: '高校野球', id: 'baseball/highschool'},
  { name: 'サッカー', id: 'soccer'},
  { name: 'スポーツ', id: 'sports'},
  { name: 'ゴルフ', id: 'sports/golf'},
  { name: 'ラグビー', id: 'sports/rugby'},
  { name: 'テニス', id: 'sports/tennis'},
  { name: 'バスケ', id: 'sports/basketball'},
  { name: 'バレー', id: 'sports/volley'},
  { name: '水泳', id: 'sports/swimming'},
]

const cache = new Map<string, string[]>(); 
const lastFetch = new Map<string, number>();

const model = defineModel();
const { widgetName, sportsNewsIndex } = defineProps<{ widgetName: string, sportsNewsIndex: number }>();

watch(() => widgetName, () => {
  if (widgetName === 'WidgetSportsNews') {
    model.value = '/picto/baseball.gif';
  }
});

async function getNews() {
  if (cache.has(selectedTopics.value[sportsNewsIndex % selectedTopics.value.length]) && lastFetch.has(selectedTopics.value[sportsNewsIndex % selectedTopics.value.length])) {
    const lastFetched = lastFetch.get(selectedTopics.value[sportsNewsIndex % selectedTopics.value.length])!;
    if (Date.now() - lastFetched < 1000 * 60 * 60) {
      articleTitles.value = cache.get(selectedTopics.value[sportsNewsIndex % selectedTopics.value.length])!;
      return;
    }
  }

  try {
    articleTitles.value = await invoke<string[]>("get_sports_news", { topic: selectedTopics.value[sportsNewsIndex % selectedTopics.value.length] });
    cache.set(selectedTopics.value[sportsNewsIndex % selectedTopics.value.length], articleTitles.value);
    lastFetch.set(selectedTopics.value[sportsNewsIndex % selectedTopics.value.length], Date.now());
  } catch (error) {
    console.error("Failed to fetch sports news", error);
  }
}

applyWidgets();
getNews();

watch(() => sportsNewsIndex, async () => {
  await getNews();
});

listen("daily_reload", async () => {
  await getNews();
});

listen("settings_changed", async () => {
  await applyWidgets();
});

const scrollDuration = computed(() => {
  return `${5 * articleTitles.value.length}s`;
});

async function applyWidgets() {
  const settings = await invoke<Settings>("get_settings");
  selectedTopics.value = topicsToId.filter(topic => settings.using_sports_news.includes(topic.name)).map(topic => topic.id);
}

</script>

<template>
  <div :class="$style.container">
    <h1>スポーツニュース</h1>
    <div :class="$style.content">
      <div v-if="1 < articleTitles.length" :class="$style.scrollTrack" :style="{ animationDuration: scrollDuration }">
        <h2 v-for="(title, index) in [...articleTitles, ...articleTitles]" :key="index" :class="$style.news">
          {{ title }}
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
  animation: infiniteScroll linear infinite;
}

.news {
  font-size: 4vmin;
  padding: 30px;
}
</style>
