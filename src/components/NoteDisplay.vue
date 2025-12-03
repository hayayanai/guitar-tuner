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
    <span class="note-name">{{ noteInfo.name }}</span>
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
  margin-bottom: 15px;
}
.note-name {
  font-size: 4em;
  font-weight: bold;
  display: block;
  line-height: 1;
}
.current-freq {
  font-size: 1.1em;
  color: #888;
  margin-top: 5px;
  display: inline;
}
.target-freq {
  font-size: 1em;
  color: #4fc3f7;
  margin-left: 8px;
}
.note-display.perfect .note-name {
  color: #4caf50;
  text-shadow: 0 0 20px rgba(76, 175, 80, 0.5);
}
.note-display.good .note-name {
  color: #ffeb3b;
}
.note-display.off .note-name {
  color: #ef5350;
}
</style>
