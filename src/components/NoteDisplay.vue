<script setup lang="ts">
import type { NoteInfo, TuningStatus } from "../types";

defineProps<{
  noteInfo: NoteInfo;
  tuningStatus: TuningStatus;
  frequency: number | null;
}>();
</script>

<template>
  <div class="note-display" :class="tuningStatus">
    <span class="note-name">{{ noteInfo.name || "—" }}</span>
    <span v-if="frequency" class="current-freq">{{ frequency.toFixed(1) }} Hz</span>
    <span v-else class="current-freq">--- Hz</span>
    <span v-if="noteInfo.targetFreq > 0" class="target-freq"
      >→ {{ noteInfo.targetFreq.toFixed(2) }} Hz</span
    >
  </div>
</template>

<style scoped>
.note-display {
  text-align: center;
  margin-bottom: var(--space-md);
}
.note-name {
  font-size: var(--font-size-huge);
  font-weight: bold;
  display: block;
  line-height: 1;
  min-height: 1.1em;
}
.current-freq {
  font-size: var(--font-size-md);
  color: var(--dads-gray-640);
  margin-top: var(--space-xs);
  display: inline;
}
.target-freq {
  font-size: var(--font-size-base);
  color: var(--dads-cyan-400);
  margin-left: var(--space-sm);
}
.note-display.perfect .note-name {
  color: var(--semantic-success-main);
  text-shadow: 0 0 20px color-mix(in srgb, var(--semantic-success-main) 50%, transparent 50%);
}
.note-display.good .note-name {
  color: var(--semantic-warning-low);
}
.note-display.off .note-name {
  color: var(--semantic-error-main);
}
</style>
