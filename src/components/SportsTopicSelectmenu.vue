<script lang="ts" setup>
import { reactive } from 'vue';

const sportsTopics = reactive([
  { name: 'プロ野球', id: 'baseball/professional', selected: true },
  { name: '高校野球', id: 'baseball/highschool', selected: true },
  { name: 'サッカー', id: 'soccer', selected: true },
  { name: 'スポーツ', id: 'sports', selected: true },
  { name: 'ゴルフ', id: 'golf', selected: true },
  { name: 'ラグビー', id: 'rugby', selected: true },
  { name: 'テニス', id: 'tennis', selected: true },
  { name: 'バスケ', id: 'basketball', selected: true },
  { name: 'バレー', id: 'volley', selected: true },
  { name: '水泳', id: 'swimming', selected: true }
]);

function toggleTopic(topic: { selected: boolean }) {
  topic.selected = !topic.selected;
}
</script>

<template>
  <div :class="$style.container">
    <div v-for="topic in sportsTopics" :key="topic.id"
      :class="topic.selected ? $style['gridItem-selected'] : $style['gridItem-unselected']" @click="toggleTopic(topic)">
      {{ topic.name }}
    </div>
  </div>
</template>

<style module>
.container {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  grid-template-rows: repeat(2, 1fr);
  gap: 8px;
  padding: 16px;
}

.gridItem-selected {
  padding: 8px;
  border: 1px solid #ccc;
  text-align: center;
}

.gridItem-unselected {
  padding: 8px;
  border: 1px solid #ccc;
  text-align: center;
  color: lightgray;
}
</style>