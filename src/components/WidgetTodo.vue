<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { useAsyncState } from "@vueuse/core";
import { computed, watch } from "vue";

type Todo = {
  id: string,
  text: string,
  completed: boolean,
};

const { widgetName } = defineProps<{ widgetName: string; }>();
const model = defineModel();
const { state: todoList, execute: refetch } = useAsyncState(async () => {
  return (await invoke<Todo[]>('get_todos')).filter(todo => !todo.completed);
}, [], { onError: (e) => console.error(e) });

const scrollDuration = computed(() => { return `${5 * todoList.value.length}s`; });

function complete(id: string) {
  invoke('complete_todo', { id });
  refetch();
}

watch(() => widgetName, () => {
  if (widgetName === 'WidgetTodo') {
    model.value = '/picto/todo.gif';
  }
});

listen("todo_changed", async () => {
  await refetch();
});

listen("daily_reload", async () => {
  await refetch();
});
</script>

<template>
  <div :class="$style.container">
    <h1>Todo List</h1>
    <div :class="$style.content">
      <div v-if="0 < todoList.length" :class="$style.scrollTrack" :style="{ animationDuration: scrollDuration }">
        <div v-for="(todo, index) in [...todoList, ...todoList]" :key="index" :class="$style.todo"
          @click="complete(todo.id)">
          <h2> □ </h2>
          <h2>{{ todo.text }}</h2>
        </div>
      </div>
      <h2 v-else>All done</h2>
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
  gap: 10px;
  padding: 10px;
}

.content {
  overflow-y: auto;
}

@keyframes infiniteScroll {
  0% {
    transform: translateY(0);
  }

  100% {
    transform: translateY(-50%);
  }
}

.scrollTrack {
  animation: infiniteScroll linear infinite;
  justify-content: left;
}

.todo {
  font-size: 5vmin;
  padding: 20px;
  display: flex;
}

.todo h2 {
  margin: 5px;
}
</style>
