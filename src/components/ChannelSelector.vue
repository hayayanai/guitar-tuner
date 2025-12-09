<script setup lang="ts">
import type { ChannelMode } from "../types";

defineProps<{
  modelValue: ChannelMode;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: ChannelMode];
}>();
</script>

<template>
  <div class="channel-select">
    <div class="channel-buttons">
      <button :class="{ active: modelValue === 0 }" @click="emit('update:modelValue', 0)">L</button>
      <button :class="{ active: modelValue === 2 }" @click="emit('update:modelValue', 2)">
        L+R
      </button>
      <button :class="{ active: modelValue === 1 }" @click="emit('update:modelValue', 1)">R</button>
    </div>
  </div>
</template>

<style scoped>
.channel-select {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 8px;
}
.channel-select label {
  font-size: 12px;
  color: var(--dads-gray-640);
}
.channel-buttons {
  display: flex;
  gap: 4px;
}
.channel-buttons button {
  padding: 4px 12px;
  font-size: 11px;
  border: 1px solid var(--dads-blue-500);
  background: var(--dads-gray-420);
  color: var(--dads-gray-640);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}
.channel-buttons button.active {
  background: var(--dads-blue-500);
  color: var(--dads-white);
}
.channel-buttons button:hover:not(.active) {
  background: var(--dads-gray-420);
}
</style>
