<script setup lang="ts">
import type { NoteInfo, TuningStatus } from '../types';

defineProps<{
  noteInfo: NoteInfo;
  tuningStatus: TuningStatus;
}>();
</script>

<template>
  <div class="tuner-meter">
    <div class="meter-track">
      <div class="meter-labels">
        <span>-50</span>
        <span>0</span>
        <span>+50</span>
      </div>
      <div class="meter-bar">
        <div class="center-zone"></div>
        <div class="meter-marks">
          <div v-for="i in 11" :key="i" class="tick" :class="{ center: i === 6 }"></div>
        </div>
        <div
          class="indicator"
          :style="{ left: `${50 + noteInfo.cent}%` }"
          :class="tuningStatus"
        ></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tuner-meter {
  margin-bottom: 15px;
}
.meter-track {
  padding: 10px 15px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 10px;
}
.meter-labels {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: #666;
  margin-bottom: 6px;
  padding: 0 5px;
}
.meter-bar {
  position: relative;
  height: 40px;
  background: #1a1a2e;
  border-radius: 6px;
  overflow: hidden;
}
.center-zone {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 20px;
  transform: translateX(-50%);
  background: rgba(76, 175, 80, 0.25);
}
.meter-marks {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  padding: 0 10%;
}
.tick {
  width: 2px;
  height: 10px;
  background: #444;
}
.tick.center {
  height: 100%;
  background: #4caf50;
  width: 3px;
}
.indicator {
  position: absolute;
  top: 4px;
  bottom: 4px;
  width: 8px;
  background: #ef5350;
  border-radius: 4px;
  transform: translateX(-50%);
  transition: left 0.08s ease-out;
  box-shadow: 0 0 10px rgba(239, 83, 80, 0.5);
}
.indicator.perfect {
  background: #4caf50;
  box-shadow: 0 0 15px rgba(76, 175, 80, 0.7);
}
.indicator.good {
  background: #ffeb3b;
  box-shadow: 0 0 10px rgba(255, 235, 59, 0.5);
}
</style>
