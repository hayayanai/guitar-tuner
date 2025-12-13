<script setup lang="ts">
import type { ChannelMode } from "../types";

import { defineModel } from "vue";
const model = defineModel<ChannelMode>({ required: true });
</script>

<template>
  <div class="channel-select">
    <div class="channel-buttons">
      <button :class="{ active: model === 0 }" @click="model = 0">L</button>
      <button :class="{ active: model === 2 }" @click="model = 2">L+R</button>
      <button :class="{ active: model === 1 }" @click="model = 1">R</button>
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
  font-size: var(--font-size-sm);
  color: var(--dads-gray-640);
}
.channel-buttons {
  display: flex;
  gap: 4px;
}
.channel-buttons button {
  padding: 4px 12px;
  font-size: var(--font-size-xs);
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
