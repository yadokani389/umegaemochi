<script setup lang="ts">

const isSettingsOpen = defineModel()

function toggleSettings() {
  isSettingsOpen.value = !isSettingsOpen.value
}
</script>

<template>
  <button :class="$style.content" @click="toggleSettings">
    <div v-if="isSettingsOpen">
      <img :class="$style.image" src="../assets/icons/batu_icon.png" />
    </div>
    <div v-else>
      <img :class="$style.image" src="../assets/icons/setting_icon.png" />
    </div>
  </button>
</template>

<style module>
.content {
  background: none;
  border: none;
  padding: none;
  cursor: pointer;
  display: inline-flex;
  justify-content: center;
  align-items: center;
}

.image {
  width: 50px;
  height: auto;
  object-fit: cover;
}
</style>
