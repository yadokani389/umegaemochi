<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { ref } from "vue";

type Settings = {
  weather_city_id: string;
  atcoder_id: string;
};

async function setup() {
  await invoke<Settings>("get_settings").then((settings) => {
    userName.value = settings.atcoder_id;
  }).catch((error) => {
    console.error(error);
  });

  let oneDayAgo = Math.trunc(new Date().getTime() / 1000) - 86400;
  let url = `https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions?user=${userName.value}&from_second=${oneDayAgo}`;

  let submissions = await (await fetch(url)).json();
  submissions = submissions.concat(submissions);

  return submissions;
}

listen("settings_changed", async () => {
  submissions.value = await setup();
});

let userName = ref("1step621");
let submissions = ref(await setup());
</script>

<template>
  <div :class="$style.container" v-if="submissions">
    <h1>{{ userName }}の最近の提出</h1>
    <div :class="$style.content">
      <div v-if="1 < submissions.length" :class="$style.scrollTrack">
        <div v-for="(submission, index) in submissions" :key="index" :class="$style.submission">
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
