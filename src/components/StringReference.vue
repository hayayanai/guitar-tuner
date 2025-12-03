<script setup lang="ts">
import type { GuitarNote } from "../types";

defineProps<{
  notes: GuitarNote[];
  activeNoteName: string;
}>();
</script>

<template>
  <div class="string-reference">
    <div
      v-for="note in notes"
      :key="note.name"
      class="string-note"
      :class="{ active: activeNoteName === note.name }"
    >
      {{ note.name }}
    </div>
  </div>
</template>

<style scoped>
.string-reference {
  display: flex;
  justify-content: center;
  gap: 10px;
  padding: 15px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
}
.string-note {
  width: 45px;
  height: 45px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #2a2a3e;
  border-radius: 50%;
  font-weight: bold;
  font-size: 14px;
  color: #888;
  transition: all 0.2s;
}
.string-note.active {
  background: #4fc3f7;
  color: #000;
  transform: scale(1.1);
  box-shadow: 0 0 15px rgba(79, 195, 247, 0.5);
}
</style>
