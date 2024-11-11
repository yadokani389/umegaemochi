<script setup lang="ts">

import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

const pictoImagesImport = import.meta.glob('./assets/picto/*.png', { eager: true });
const pictoImages = Object.values(pictoImagesImport).map(module => (module as { default: string }).default || (module as string));
const currentPictoIndex = ref(0)
const currentPictoImage = ref(pictoImages[currentPictoIndex.value])
console.log(pictoImages)

</script>

<template>
    <main class="container">
        <div class="left"></div>
        <div class="right">
            <img :src="currentPictoImage" alt="picto-san" />
        </div>
    </main>
</template>

<style scoped>
.container {
    display: flex;
    width: 100%;
    height: 100vh;
    flex-direction: row;
}

.left {
    width: 50%;
    background-color: #f0f0f0;
}

.right {
    width: 50%;
    background-color: #f0f0f0;
}

.right img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
}
</style>