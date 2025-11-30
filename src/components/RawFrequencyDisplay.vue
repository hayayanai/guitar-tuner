<script setup lang="ts">
defineProps<{
  rawFrequency: number | null;
}>();
</script>

<template>
  <div class="raw-frequency">
    <span class="label">Detected:</span>
    <span class="value">{{ rawFrequency ? rawFrequency.toFixed(1) + ' Hz' : '--- Hz' }}</span>
  </div>
</template>

<style scoped>
.raw-frequency {
  text-align: center;
  margin-bottom: 15px;
  padding: 8px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: 6px;
}
.raw-frequency .label {
  font-size: 11px;
  color: #666;
  margin-right: 8px;
}
.raw-frequency .value {
  font-size: 14px;
  color: #4fc3f7;
  font-family: monospace;
}
</style>
