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
  font-size: 2em;
  font-weight: bold;
  margin-bottom: 15px;
}
.cent-display.perfect {
  color: var(--semantic-success-main);
}
.cent-display.good {
  color: var(--semantic-warning-low);
}
.cent-display.off {
  color: var(--semantic-error-main);
}
</style>
