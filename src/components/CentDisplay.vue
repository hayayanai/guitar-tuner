<script setup lang="ts">
import type { TuningStatus } from '../types';

defineProps<{
  centDisplay: string;
  tuningStatus: TuningStatus;
  hasFrequency: boolean;
}>();
</script>

<template>
  <div
    class="cent-display"
    :class="tuningStatus"
  >
    <span v-if="hasFrequency">{{ centDisplay }} cent</span>
    <span v-else>---</span>
  </div>
</template>

<style scoped>
.cent-display {
  text-align: center;
  font-size: 2em;
  font-weight: bold;
  margin-bottom: 15px;
}
.cent-display.perfect {
  color: #4caf50;
}
.cent-display.good {
  color: #ffeb3b;
}
.cent-display.off {
  color: #ef5350;
}
</style>
