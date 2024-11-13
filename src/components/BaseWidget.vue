<script setup lang="ts">
</script>

<template>
  <div :class="$style.widgetFlame">
    <div :class="$style.widgetContent">
      <Suspense>
        <slot></slot>
        <template #fallback>
          <div>
            <h1>Loading...</h1>
          </div>
        </template>
      </Suspense>
    </div>
  </div>
</template>

<style module>
.widgetFlame {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f0f0f0;
  border-radius: 30px;
}

.widgetContent {
  width: 98%;
  height: 98%;
}
</style>
