<script setup lang="ts">
defineProps<{
  devices: string[];
  modelValue: string;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();
</script>

<template>
  <div class="device-select">
    <label for="device-select">Input:</label>
    <select
      id="device-select"
      :value="modelValue"
      @change="emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
    >
      <option v-for="d in devices" :key="d" :value="d">
        {{ d }}
      </option>
    </select>
  </div>
</template>

<style scoped>
.device-select {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 8px;
}
.device-select select {
  padding: 6px 10px;
  font-size: 12px;
  border-radius: 6px;
  border: 1px solid #4fc3f7;
  background: #1a1a2e;
  color: #fff;
  max-width: 200px;
}
</style>
