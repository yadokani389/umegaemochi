<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { ref } from "vue";

type DisasterInfo = {
  title: string,
  description: string,
  warning: string,
  occurred: string,
};

const info = ref({ title: "a", description: "b", warning: "c", occurred: "d" } as DisasterInfo);

listen<DisasterInfo>("disaster_occurred", (info_) => {
  info.value = info_.payload;
});
</script>

<template>
  <div :class="$style.container">
    <h1 :class="$style.title">⚠{{ info.title }}</h1>
    <div :class="$style.content">
      <div :class="$style.description">{{ info.description }}</div>
      <div :class="$style.warning">{{ info.warning }}</div>
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
  gap: 10px;
  background-color: #1c8c41;
  position: relative;
}

.title {
  color: rgb(255, 255, 255);
  font-size: 8vmin;
  position: absolute;
  top: 4vh;
  align-self: center;
  max-width: 95vw;
  padding: 8px;
  border: 40px solid rgb(56, 280, 130);
  border-radius: 10px;
  width: fit-content;
}

.content {
  position: absolute;
  top: 62vh;
  text-align: left;
  max-width: 90vw;
  margin-left: 5vh;
}

.description {
  color: #ffffff;
  font-size: 6vmin;
}

.warning {
  color: #ffffff;
  font-size: 6vmin;
}
</style>