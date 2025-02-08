<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, computed } from "vue";

const articleTitles = ref<string[]>([]);
const topics = ["baseball", "soccer", "sports", "baseball/highschool"];

const model = defineModel();
model.value = "/picto/gorogoro.gif";

const { sportsNewsIndex } = defineProps<{ sportsNewsIndex: number }>();

async function getNews() {
  try {
    articleTitles.value = await invoke("get_sports_news", { topic: topics[sportsNewsIndex % topics.length] });
  } catch (error) {
    console.error("Failed to fetch sports news", error);
  }
}

getNews();

listen("daily_reload", async () => {
  await getNews();
});

const scrollDuration = computed(() => {
  return `${5 * articleTitles.value.length}s`;
});

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
