<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, onMounted } from "vue";

const articleTitles = ref<string[]>([]);
const topic = "baseball"; 

const model = defineModel();
model.value = "/picto/gorogoro.gif";

async function getNews() {
  try {
    articleTitles.value = await invoke("get_sports_news", { topic });
    console.log("Fetched sports news", articleTitles.value);
  } catch (error) {
    console.error("Failed to fetch sports news", error);
  }
}

onMounted(() => {
  getNews();
});

listen("daily_reload", async () => {
  await getNews();
});

</script>

<template>
  <div :class="$style.container">
    <h1>スポーツニュース</h1>
    <div :class="$style.content">
      <h2 v-for="title in articleTitles" :key="title" :class="$style.news">
        {{ title }}
      </h2>
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
</style>
