<script setup lang="ts">
import type { TuningStatus } from "../types";

defineProps<{
  centDisplay: string;
  tuningStatus: TuningStatus;
  hasFrequency: boolean;
}>();
</script>

<template>
  <div class="cent-display" :class="tuningStatus">
    <span v-if="hasFrequency">{{ centDisplay }} cent</span>
    <span v-else>---</span>
  </div>
</template>

<style scoped>
.cent-display {
  text-align: center;
  font-size: var(--font-size-2xl);
  font-weight: bold;
  margin-bottom: 15px;

  &.perfect {
    color: var(--semantic-success-main);
  }

  &.good {
    color: var(--semantic-warning-low);
  }

  &.off {
    color: var(--semantic-error-main);
  }
}
</style>
