<script setup lang="ts">
import { DisasterInfo, Settings } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { type } from '@tauri-apps/plugin-os';
import alertSound from '../assets/sounds/meka_mi_radar03.mp3';

defineProps<{
  disastarInfo: DisasterInfo,
}>();

async function notifyWithSound() {
  const playSound = (await invoke<Settings>('get_settings')).use_sound_when_disaster;
  if (playSound && type() !== 'linux') {
    const audio = new Audio(alertSound);
    audio.play();
  }
}

notifyWithSound();

</script>

<template>
  <div :class="$style.container">
    <h1 :class="$style.title">⚠{{ disastarInfo.title }}</h1>
    <div :class="$style.content">
      <div>
        <h2>{{ disastarInfo.description }}</h2>
        <h2>{{ disastarInfo.warning }}</h2>
      </div>
      <img :class="$style.image" src="/picto/earthquake.gif" :alt="'disaster image'" />
    </div>
  </div>
</template>

<style module>
.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 5%;
  background-color: #1c8c41;
  position: relative;
}

.title {
  color: rgb(255, 255, 255);
  font-size: 8vmin;
  align-self: center;
  max-width: 95vw;
  border: 0.6em solid rgb(56, 280, 130);
  border-radius: 0.5em;
  width: fit-content;
  padding: 2%;
}

.content {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  text-align: left;
  max-width: 90vw;
}

.content h2 {
  color: #ffffff;
  font-size: 6vmin;
}

.image {
  width: 30%;
  height: auto;
  max-height: 100%;
  justify-content: flex-end;
  object-fit: contain;
}
</style>
