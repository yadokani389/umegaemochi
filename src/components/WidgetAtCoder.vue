<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

type Settings = {
  weather_city_id: String;
  atcoder_id: String;
};

const userName = await (async () => {
  try {
    return (await invoke("get_settings") as Settings).atcoder_id;
  } catch (error) {
    console.error(error);
    return "1step621";
  }
})();

let d = Math.trunc(new Date().getTime() / 1000) - 86400;
let url = "https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions?user=" + userName + "&from_second=" + d;
let submissions = await (await fetch(url)).json();
submissions = submissions.concat(submissions);
</script>

<template>
  <div :class="$style.container">
    <h1>{{ userName }}の最近の提出</h1>
    <div :class="$style.content">
      <div :class="$style.scrollTrack">
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
