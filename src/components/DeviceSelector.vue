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
  font-size: var(--font-size-sm);
  border-radius: 6px;
  border: 1px solid var(--dads-gray-420);
  background: var(--dads-gray-420);
  color: var(--color-text);
  width: 100%;
}
</style>
