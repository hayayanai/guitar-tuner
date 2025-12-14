<script setup lang="ts">
const props = defineProps<{ devices: string[] }>();
const model = defineModel<string>({ required: true });
</script>

<template>
  <div class="device-select">
    <select
      id="device-select"
      :value="model"
      @change="model = ($event.target as HTMLSelectElement).value"
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
  width: 100%;

  select {
    margin-left: var(--space-sm);
    padding: var(--space-xs) var(--space-sm);
    border-radius: 4px;
    background: var(--dads-gray-420);
    color: var(--dads-gray-640);
    border: 1px solid var(--dads-gray-420);
    box-sizing: border-box;
    max-width: 100%;
    width: 100%;

    &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
      background: var(--dads-gray-420);
    }
  }
}
</style>
