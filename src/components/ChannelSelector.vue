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

  label {
    font-size: var(--font-size-sm);
    color: var(--dads-gray-640);
  }
}

.channel-buttons {
  display: flex;
  gap: 4px;

  button {
    padding: 4px 12px;
    font-size: var(--font-size-xs);
    border: 1px solid var(--color-border);
    background: var(--color-background);
    color: var(--color-text);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;

    &.active {
      background: var(--color-primary);
      border-color: var(--color-primary);
      color: #ffffff;
    }

    &:hover:not(.active) {
      background: var(--color-background-secondary);
      border-color: var(--color-primary);
    }
  }
}
</style>
