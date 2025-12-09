<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  level: number; // 0〜1の値
}>();

const levelPercent = computed(() => props.level * 100);
</script>

<template>
  <div class="input-level">
    <label>Input Level</label>
    <div class="level-bar">
      <div
        class="level-fill"
        :style="{ width: `${levelPercent}%` }"
        :class="{ hot: levelPercent > 90 }"
      />
    </div>
    <div class="level-labels">
      <span>-80dB</span>
      <span>-40dB</span>
      <span>0dB</span>
    </div>
  </div>
</template>

<style scoped>
.input-level {
  max-width: 300px;
  margin: var(--space-lg, 24px) auto;
  padding: var(--space-md, 16px);
  background: var(--dads-gray-420);
  border-radius: var(--radius-md, 8px);
}
.input-level label {
  display: block;
  text-align: center;
  font-size: 14px;
  color: var(--dads-gray-640);
  margin-bottom: var(--space-sm, 8px);
}
.level-bar {
  width: 100%;
  height: 12px;
  background: var(--dads-white);
  border-radius: 6px;
  overflow: hidden;
}
.level-fill {
  height: 100%;
  background: var(--semantic-success-main);
  transition: width 0.05s ease-out;
  border-radius: 6px;
}
.level-fill.hot {
  background: var(--semantic-error-main);
}
.level-labels {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--dads-gray-640);
  margin-top: var(--space-xs, 4px);
}
</style>
