<script setup lang="ts">
import { ref, watch } from 'vue';

const { widgetName } = defineProps<{ widgetName: string; }>();
const model = defineModel();
const currentTime = ref('');

const clock = () => {
  const currentDate = new Date();
  const hours = currentDate.getHours().toString().padStart(2, '0');
  const minutes = currentDate.getMinutes().toString().padStart(2, '0');
  currentTime.value = `${hours}:${minutes}`;
}

watch(() => widgetName, () => {
  if (widgetName === 'WidgetClock') {
    const hours = new Date().getHours();
    if (6 <= hours && hours <= 12) {
      model.value = '/picto/train.gif';
    } else if (13 <= hours && hours <= 20) {
      model.value = '/picto/gorogoro.gif';
    } else {
      model.value = '/picto/sleep.gif';
    }
  }
});

clock();
setInterval(clock, 1000);
</script>

<template>
  <div :class="$style.container">
    <div :class="$style.clock">{{ currentTime }}</div>
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
}

.clock {
  color: #70ad47;
  font-family: 'ADLaMFont';
  font-size: 10vw;
  font-weight: bold;
}
</style>
