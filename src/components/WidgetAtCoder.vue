<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { computedAsync } from "@vueuse/core";
import { computed, ref } from "vue";
import { Settings } from "../types";

type Submission = {
  problem_id: string;
  language: string;
  result: string;
};

const emit = defineEmits(["updatePicto"]);
const userName = ref((await invoke<Settings>("get_settings")).atcoder_id);
const oneDayAgo = ref(Math.trunc(new Date().getTime() / 1000) - 86400);
const url = computed(() => `https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions?user=${userName.value}&from_second=${oneDayAgo.value}`);
const evaluating = ref(false);
const submissions = computedAsync(async () => {
  return await (await fetch(url.value)).json() as Submission[];
}, [], evaluating);

emit("updatePicto", '/picto/banana.png');

listen("settings_changed", async () => {
  userName.value = (await invoke<Settings>("get_settings")).atcoder_id;
  oneDayAgo.value = Math.trunc(new Date().getTime() / 1000) - 86400;
});

listen("daily_reload", async () => {
  oneDayAgo.value = Math.trunc(new Date().getTime() / 1000) - 86400;
});
</script>

<template>
  <div :class="$style.container" v-if="submissions">
    <h1>{{ userName }}の最近の提出</h1>
    <div :class="$style.content">
      <div v-if="!evaluating" :class="$style.scrollTrack">
        <div v-for="(submission, index) in [...submissions, ...submissions]" :key="index" :class="$style.submission">
          <h2>問題: {{ submission.problem_id }}</h2>
          <h2>言語: {{ submission.language }}</h2>
          <h2>結果: {{ submission.result }}</h2>
        </div>
      </div>
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

.submission {
  font-size: 4vmin;
  padding: 15px;
}
</style>
