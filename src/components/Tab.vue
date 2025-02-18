<script setup lang="ts">
import { ref, watch } from 'vue';
import { useMousePressed } from '@vueuse/core';

const currentTime = ref('');
const imgSrc = ref('/picto/sleep.gif');

const clock = () => {
  const currentDate = new Date();
  const year = currentDate.getFullYear();
  const month = (currentDate.getMonth() + 1).toString().padStart(2, '0');
  const date = currentDate.getDate().toString().padStart(2, '0');
  const hours = currentDate.getHours().toString().padStart(2, '0');
  const minutes = currentDate.getMinutes().toString().padStart(2, '0');
  currentTime.value = `${year}/${month}/${date}/${hours}:${minutes}`;
}

const image = ref<HTMLElement | null>(null);
const { pressed } = useMousePressed({ target: image });

watch(pressed, (newVal) => {
  if (newVal) {
    imgSrc.value = '/picto/sleep_angry.png';

    setTimeout(() => {
      imgSrc.value = '/picto/sleep.gif';
    }, 10000);
  }
});

clock();
setInterval(clock, 1000);
</script>

<template>
  <div :class="$style.container">
    <div :class="$style.clock">
      {{ currentTime }}
      <img :class="$style.tabIcon" :src="imgSrc" ref="image" />
    </div>
  </div>
</template>

<style module>
.container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  border-bottom: 1vw solid #70ad47;
  padding-bottom: 1vw;
}

.clock {
  color: #70ad47;
  font-family: 'ADLaMFont';
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
