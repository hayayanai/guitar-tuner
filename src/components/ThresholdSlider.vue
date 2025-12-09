<script setup lang="ts">
defineProps<{
  modelValue: number;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: number];
}>();

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:modelValue", parseFloat(target.value));
}
</script>

<template>
  <div class="threshold-control">
    <div class="slider-wrapper">
      <input
        id="sensitivity-slider"
        type="range"
        min="1.2"
        max="5.0"
        step="0.1"
        :value="modelValue"
        @input="handleInput"
      />
    </div>
    <div class="threshold-labels">
      <span>High</span>
      <span class="threshold-value">{{ modelValue.toFixed(1) }}</span>
      <span>Low</span>
    </div>
  </div>
</template>

<style scoped>
.threshold-control {
  width: 100%;
}
/* Slider値の表示 */
.slider-wrapper {
  /* 値を中央に配置 */
  .threshold-value {
    font-size: var(--font-size-sm);
    color: var(--dads-gray-420);
    margin: 0 12px;
  }
  .threshold-labels {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 8px;
    font-size: var(--font-size-sm);
    color: var(--dads-gray-640);
    margin-top: var(--space-xs, 4px);
  }
  padding: 6px 12px; /* つまみがはみ出さないように上下左右にパディング */
  overflow: hidden;
}
.threshold-control input[type="range"] {
  display: block;
  width: -webkit-fill-available;
  height: 8px;
  -webkit-appearance: none;
  appearance: none;
  background: var(--dads-gray-420);
  border-radius: 4px;
  outline: none;
  margin: 0;
}
.threshold-control input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 20px;
  height: 20px;
  background: var(--dads-blue-500);
  border: 2px solid var(--dads-white);
  border-radius: 50%;
  cursor: pointer;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}
.threshold-control input[type="range"]::-webkit-slider-thumb:hover {
  background: var(--dads-blue-600);
}
.threshold-control input[type="range"]:focus {
  outline: none;
}
.threshold-control input[type="range"]:focus::-webkit-slider-thumb {
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--dads-blue-500) 30%, transparent 70%);
}
.threshold-labels {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-sm);
  color: var(--dads-gray-640);
  margin-top: var(--space-xs, 4px);
  padding: 0 10px; /* ラベルも同じパディング */
}
</style>
