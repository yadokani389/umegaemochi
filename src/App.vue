<script setup lang="ts">

// import { defineAsyncComponent } from "vue";
// import { invoke } from "@tauri-apps/api/core";
// const WidgetWeather = defineAsyncComponent(() => import("./components/WidgetWeather.vue"));
import BaseWidget from "./components/BaseWidget.vue";
import NewsWidget from "./components/NewsWidget.vue";
import WidgetWeather from "./components/WidgetWeather.vue";
import WidgetNews from "./components/WidgetNews.vue";

const pictoImagesImport = import.meta.glob('./assets/picto/*.{gif,png}', { eager: true });
const pictoImages = Object.values(pictoImagesImport).map(module => (module as { default: string }).default || (module as string));
//const currentPictoIndex = ref(0)
//const currentPictoImage = ref(pictoImages[currentPictoIndex.value])
console.log(pictoImages)

</script>

<template>
  <main class="container">
    <div class="left-side">
      <div class="left-top-side">
        <Suspense>
          <WidgetWeather />
          <template #fallback>
            <BaseWidget>
              <div>
                <h1>Loading...</h1>
              </div>
            </BaseWidget>
          </template>
        </Suspense>
      </div>
      <div class="left-bottom-side">
        <Suspense>
          <WidgetNews />
          <template #fallback>
            <BaseWidget>
              <div>
                <h1>Loading...</h1>
              </div>
            </BaseWidget>
          </template>
        </Suspense>
      </div>
    </div>
    <div class="right-side">
      <ul class="cube">
        <li class="cube-front">
          <div class="image-box">
            <img :src="pictoImages[0]" alt="front" />
          </div>
        </li>
        <li class="cube-right">
          <div class="image-box">
            <img :src="pictoImages[1]" alt="right" />
          </div>
        </li>
        <li class="cube-back">
          <div class="image-box">
            <img :src="pictoImages[2]" alt="back" />
          </div>
        </li>
        <li class="cube-left">
          <div class="image-box">
            <img :src="pictoImages[3]" alt="left" />
          </div>
        </li>
      </ul>
    </div>
  </main>
</template>

<style>
* {
  padding: 0;
  margin: 0;
  box-sizing: border-box;
}
</style>

<style scoped>
.container {
  display: flex;
  width: 100%;
  height: 100vh;
  flex-direction: row;
  justify-content: center;
  align-items: center;
}

.left-side {
  display: flex;
  flex-direction: column;
  width: 50%;
  height: 100%;
}

.left-top-side,
.left-bottom-side {
  height: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
}


.right-side {
  width: 45%;
  height: 80%;
  justify-content: center;
  display: flex;
  background-color: #f0f0f0;
  align-items: center;
  border-radius: 10%;
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

.cube-front {
  transform: translate3d(0, 0, -100px);
}

.cube-right {
  transform: translate3d(-100px, 0, 0) rotateY(90deg);
}

.cube-back {
  transform: translate3d(0px, 0px, 100px) rotateY(180deg);
}

.cube-left {
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
