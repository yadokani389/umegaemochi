<script setup lang="ts">
defineProps<{ pictoSrc: string }>()
</script>

<template>
  <transition name="fade" mode="out-in">
    <img :src="pictoSrc" :key="pictoSrc" />
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
