<script setup lang="ts">
import { computed } from 'vue';

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
  margin: 0 auto 15px;
  padding: 10px 15px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
}
.input-level label {
  display: block;
  text-align: center;
  font-size: 12px;
  color: #aaa;
  margin-bottom: 8px;
}
.level-bar {
  width: 100%;
  height: 12px;
  background: #222;
  border-radius: 6px;
  overflow: hidden;
}
.level-fill {
  height: 100%;
  background: linear-gradient(90deg, #4caf50, #8bc34a, #ffeb3b);
  transition: width 0.05s ease-out;
  border-radius: 6px;
}
.level-fill.hot {
  background: linear-gradient(90deg, #4caf50, #ffeb3b, #ff5722);
}
.level-labels {
  display: flex;
  justify-content: space-between;
  font-size: 9px;
  color: #666;
  margin-top: 4px;
}
</style>
