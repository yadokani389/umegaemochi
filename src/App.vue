<script setup lang="ts">

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const pictoImagesImport = import.meta.glob('./assets/picto/*.png', { eager: true });
const pictoImages = Object.values(pictoImagesImport).map(module => (module as { default: string }).default || (module as string));
//const currentPictoIndex = ref(0)
//const currentPictoImage = ref(pictoImages[currentPictoIndex.value])
console.log(pictoImages)

</script>

<template>
    <main class="container">
        <div class="left-side"></div>
        <div class="right-side">
            <ul class="cube">
                <li class="front">
                    <div class="image-box">
                        <img :src="pictoImages[0]" alt="front" />
                    </div>
                </li>
                <li class="right">
                    <div class="image-box">
                        <img :src="pictoImages[1]" alt="right" />
                    </div>
                </li>
                <li class="back">
                    <div class="image-box">
                        <img :src="pictoImages[2]" alt="back" />
                    </div>
                </li>
                <li class="left">
                    <div class="image-box">
                        <img :src="pictoImages[3]" alt="left" />
                    </div>
                </li>
            </ul>
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

.left-side {
    width: 50%;
    background-color: #f0f0f0;
}

.right-side {
    width: 50%;
    display: flex;
    background-color: #f0f0f0;
    align-items: center;
}

.cube {
    position: relative;
    margin: 30px auto;
    width: 200px;
    height: 200px;
    transform-style: preserve-3d;
    animation: rotate 20s linear infinite;
}

@keyframes rotate {
    from {
        transform: rotateY(20deg);
    }

    to {
        transform: rotateY(-340deg);
    }
}

.cube li {
    position: absolute;
    left: 0;
    right: 0;
    width: 100%;
    height: 100%;
    list-style: none;
}

.front {
    transform: translate3d(0, 0, -100px);
}

.right {
    transform: translate3d(-100px, 0, 0) rotateY(90deg);
}

.back {
    transform: translate3d(0px, 0px, 100px) rotateY(180deg);
}

.left {
    transform: translate3d(100px, 0, 0) rotateY(-90deg);
}

.image-box {
    position: relative;
    display: flex;
    justify-content: center;
    overflow: hidden;
}

img {
    max-width: 200px;
    height: auto;
    object-fit: contain;
    transition: all 1s ease;
    background-color: rgba(240, 240, 240, 0.8); 
    /* #f0f0f0 = rgb (240, 240, 240)*/
}
</style>