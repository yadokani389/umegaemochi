<script setup lang="ts">
const pictoImagesImport = import.meta.glob('../assets/picto/*.{gif,png}', { eager: true });
const pictoImages = Object.values(pictoImagesImport).map(module => (module as { default: string }).default || (module as string));
</script>

<template>
  <ul :class="$style.cube">
    <li :class="$style.cubeFront">
      <div :class="$style.imageBox">
        <img :src="pictoImages[0]" alt="front" />
      </div>
    </li>
    <li :class="$style.cubeRight">
      <div :class="$style.imageBox">
        <img :src="pictoImages[1]" alt="right" />
      </div>
    </li>
    <li :class="$style.cubeBack">
      <div :class="$style.imageBox">
        <img :src="pictoImages[2]" alt="back" />
      </div>
    </li>
    <li :class="$style.cubeLeft">
      <div :class="$style.imageBox">
        <img :src="pictoImages[3]" alt="left" />
      </div>
    </li>
  </ul>
</template>

<style module>
.cube {
  position: relative;
  width: 100%;
  height: 100%;
  transform-style: preserve-3d;
  animation: rotate 20s linear infinite;
  display: flex;
  align-items: center;
  justify-content: center;
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

.cubeFront {
  transform: translate3d(0, 0, -100px);
}

.cubeRight {
  transform: translate3d(-100px, 0, 0) rotateY(90deg);
}

.cubeBack {
  transform: translate3d(0px, 0px, 100px) rotateY(180deg);
}

.cubeLeft {
  transform: translate3d(100px, 0, 0) rotateY(-90deg);
}

.imageBox {
  position: relative;
  display: flex;
  justify-content: center;
  overflow: hidden;
  height: 100%;
}

img {
  max-width: 200px;
  height: auto;
  object-fit: contain;
  transition: all 20s ease;
  background-color: rgba(240, 240, 240, 0.8);
  /* #f0f0f0 = rgb (240, 240, 240)*/
}
</style>
