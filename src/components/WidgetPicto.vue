<script setup lang="ts">
import { useMousePressed } from '@vueuse/core';
import { ref, watch } from 'vue';


const pictoSrc = defineModel<string>({ required: true });
const image = ref<HTMLElement | null>(null);
const { pressed } = useMousePressed({ target: image });
const noTransition = ref(false);

watch(pressed, (newVal) => {
  if (newVal) {
    noTransition.value = true;

    if (pictoSrc.value === '/picto/sleep.gif') {
      pictoSrc.value = '/picto/sleep_angry.png';
    } else if (pictoSrc.value === '/picto/news.gif') {
      pictoSrc.value = '/picto/news_odoroki.png';
    }

    setTimeout(() => {
      noTransition.value = false;
    }, 500);
  }
});
</script>

<template>
  <transition :name="noTransition ? '' : 'fade'" mode="out-in">
    <img :src="pictoSrc" :key="pictoSrc" ref="image" />
  </transition>
</template>

<style scoped>
img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.fade-enter-to,
.fade-leave-from {
  opacity: 1;
}
</style>
