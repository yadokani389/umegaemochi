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
    <div :class="$style.clock">
      {{ currentTime }}
      <img :class="$style.tabIcon" src="/src/assets/picto/sleep.gif" />
    </div>
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
  border-bottom: 1vw solid #70ad47;
  padding-bottom: 1vw;
}

.clock {
  color: #70ad47;
  font-family: 'ClockFont';
  font-size: 5vw;
  font-weight: bold;
}

.tabIcon {
  width: 10vw;
  height: 10vw;
  object-fit: cover;
  background-color: transparent;
  vertical-align: -2.5vw;
}
</style>