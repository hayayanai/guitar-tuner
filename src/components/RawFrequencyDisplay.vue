<script setup lang="ts">
defineProps<{
  rawFrequency: number | null;
}>();
</script>

<template>
  <div class="raw-frequency">
    <span class="label">Detected:</span>
    <span class="value">{{ rawFrequency ? rawFrequency.toFixed(1) + " Hz" : "--- Hz" }}</span>
  </div>
</template>

<style scoped>
.raw-frequency {
  text-align: center;
  margin-bottom: var(--space-md);
  padding: var(--space-sm);
  background: var(--dads-gray-420);
  border-radius: 6px;

  .label {
    font-size: var(--font-size-xs);
    color: var(--dads-gray-640);
    margin-right: var(--space-sm);
  }

  .value {
    font-size: var(--font-size-md);
    color: var(--dads-cyan-400);
    font-family: monospace;
  }
}
</style>
