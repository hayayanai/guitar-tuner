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
  background: var(--dads-gray-420);
  border-radius: 10px;
}
.string-note {
  width: 45px;
  height: 45px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--dads-white);
  border-radius: 50%;
  font-weight: bold;
  font-size: 14px;
  color: var(--dads-gray-640);
  transition: all 0.2s;
}
.string-note.active {
  background: var(--dads-blue-500);
  color: var(--dads-white);
  transform: scale(1.1);
  box-shadow: 0 0 15px var(--dads-blue-500);
}
</style>
