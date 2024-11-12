<script setup lang="ts">
</script>

<template>
  <div class="widget-flame">
    <div class="widget-content">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.widget-flame {
  width: 80%;
  height: 80%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f0f0f0;
  border-radius: 40px;
}

.widget-content {
  width: 90%;
  height: 90%;
}
</style>
