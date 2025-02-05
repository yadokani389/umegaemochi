<script setup lang="ts">
import { ref } from 'vue';

const model = defineModel();
const currentTime = ref('');

const clock = () => {
  const currentDate = new Date();
  const hours = currentDate.getHours().toString().padStart(2, '0');
  const minutes = currentDate.getMinutes().toString().padStart(2, '0');
  currentTime.value = `${hours}:${minutes}`;
}

model.value = '/picto/rock.png';

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
  justify-content: center;
  align-items: center;
}

.clock {
  color: #70ad47;
  font-family: 'ClockFont';
  font-size: 10vw;
  font-weight: bold;
}
</style>
