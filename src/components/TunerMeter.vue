<script setup lang="ts">
import type { NoteInfo, TuningStatus } from "../types";

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
        <div class="center-zone" />
        <div class="meter-marks">
          <div v-for="i in 11" :key="i" class="tick" :class="{ center: i === 6 }" />
        </div>
        <div class="indicator" :style="{ left: `${50 + noteInfo.cent}%` }" :class="tuningStatus" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.tuner-meter {
  margin-bottom: var(--space-md);
}
.meter-track {
  padding: var(--space-sm) var(--space-md) var(--space-md);
  background: rgba(0, 0, 0, 0.3); /* 非テキスト装飾はそのまま */
  border-radius: 10px;
}
.meter-labels {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-xs);
  color: var(--dads-gray-536);
  margin-bottom: var(--space-xs);
  padding: 0 var(--space-xs);
}
.meter-bar {
  position: relative;
  height: 40px;
  background: var(--dads-gray-420);
  border-radius: 6px;
  overflow: hidden;

  .center-zone {
    position: absolute;
    left: 50%;
    top: 0;
    bottom: 0;
    width: 20px;
    transform: translateX(-50%);
    background: var(--dads-gray-420);
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

    .tick {
      width: 2px;
      height: 10px;
      background: var(--dads-gray-536);

      &.center {
        height: 100%;
        background: var(--semantic-success-main);
        width: 3px;
      }
    }
  }

  .indicator {
    position: absolute;
    top: 4px;
    bottom: 4px;
    width: 8px;
    background: var(--semantic-error-main);
    border-radius: 4px;
    transform: translateX(-50%);
    transition: left 0.08s ease-out;
    box-shadow: 0 0 10px color-mix(in srgb, var(--semantic-error-main) 50%, transparent 50%);

    &.perfect {
      background: var(--semantic-success-main);
      box-shadow: 0 0 15px color-mix(in srgb, var(--semantic-success-main) 70%, transparent 30%);
    }

    &.good {
      background: var(--semantic-warning-low);
      box-shadow: 0 0 10px color-mix(in srgb, var(--semantic-warning-low) 50%, transparent 50%);
    }
  }
}
</style>
