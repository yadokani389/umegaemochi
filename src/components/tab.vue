<script setup lang="ts">
import { ref } from 'vue';

const currentTime = ref('');

const clock = () => {
  const currentDate = new Date();
  const year = currentDate.getFullYear();
  const month = (currentDate.getMonth() + 1).toString().padStart(2, '0');
  const date = currentDate.getDate().toString().padStart(2, '0');
  const hours = currentDate.getHours().toString().padStart(2, '0');
  const minutes = currentDate.getMinutes().toString().padStart(2, '0');
  currentTime.value = `${year}/${month}/${date}/${hours}:${minutes}`;
}

clock();
setInterval(clock, 1000);

</script>

<template>
  <div :class="$style.container">
    <div :class="$style.clock">{{ currentTime }}</div>
  </div>
</template>

<style module>

@font-face {
  font-family: 'ClockFont';
  src: url('/src/assets/fonts/ADLaMDisplay-Regular.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}

.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: left;
  align-items: left;
}

.clock {
  color: #70ad47;
  font-family: 'ClockFont';
  font-size: 5vw;
  font-weight: bold;
}
</style>