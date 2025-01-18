<script setup lang="ts">
defineProps({
  title: String,
  description: String,
  warning: String,
  occurred: String,
})
</script>

<template>
  <div :class="$style.container">
    <h1 :class="$style.title">⚠{{ title }}</h1>
    <div :class="$style.content">
      <div :class="$style.description">{{ description }}</div>
      <div :class="$style.warning">{{ warning }}</div>
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
