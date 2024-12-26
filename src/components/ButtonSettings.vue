<script setup lang="ts">
import { ref } from "vue";

const JudgeSetting = ref(false)

const emit = defineEmits(['childEvent']);

function viewsettings() {
  JudgeSetting.value = !JudgeSetting.value
  emit('childEvent', { arg1: JudgeSetting.value });
}
</script>

<template>
  <div :class="$style.container">
    <button :class="$style.content" @click="viewsettings">
      <div v-if="JudgeSetting">
        <img :class="$style.image" src="../assets/icons/batu_icon.png" />
      </div>
      <div v-else>
        <img :class="$style.image" src="../assets/icons/setting_icon.png" />
      </div>
    </button>
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
  gap: 10px;
}

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
  max-width: 60px;
  max-height: 60px;
  flex: 0.7;
  object-fit: cover;
}
</style>
