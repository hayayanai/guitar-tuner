<script setup lang="ts">
defineProps<{
  modelValue: number;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: number];
}>();

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit('update:modelValue', parseFloat(target.value));
}
</script>

<template>
  <div class="threshold-control">
    <label>Sensitivity: {{ modelValue.toFixed(1) }}</label>
    <input
      type="range"
      min="1.2"
      max="5.0"
      step="0.1"
      :value="modelValue"
      @input="handleInput"
    />
    <div class="threshold-labels">
      <span>High</span>
      <span>Low</span>
    </div>
  </div>
</template>

<style scoped>
.threshold-control {
  max-width: 300px;
  margin: 0 auto 20px;
  padding: 10px 15px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
}
.threshold-control label {
  display: block;
  text-align: center;
  font-size: 12px;
  color: #aaa;
  margin-bottom: 8px;
}
.threshold-control input[type="range"] {
  width: 100%;
  height: 6px;
  -webkit-appearance: none;
  appearance: none;
  background: #333;
  border-radius: 3px;
  outline: none;
}
.threshold-control input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  background: #4fc3f7;
  border-radius: 50%;
  cursor: pointer;
}
.threshold-labels {
  display: flex;
  justify-content: space-between;
  font-size: 10px;
  color: #666;
  margin-top: 4px;
}
</style>
